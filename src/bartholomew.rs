use crate::content::{self, all_pages_load};
use crate::response::{self, *};
use crate::template::{self, DynamicTemplateParams};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{Request, Response},
    http_component,
    key_value::Store,
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
struct RenderedPageCacheEntry {
    expiry_time: DateTime<Utc>,
    content: String,
}

/// The entry point to Bartholomew.
#[http_component]
pub fn render(req: Request) -> Result<Response> {
    // Preview mode lets you see content marked as unpublished.
    let preview_mode = match std::env::var(PREVIEW_MODE_ENV) {
        Ok(val) if val == "1" => {
            eprintln!("Bartholomew is running in PREVIEW_MODE");
            true
        }
        _ => false,
    };
    let disable_cache = match std::env::var("DISABLE_CACHE") {
        Ok(val) if val == "1" => {
            eprintln!("INFO: Bartholomew is running with DISABLE_CACHE=1");
            true
        }
        _ => false,
    };

    // Get the request path.
    let path_info = match req.headers().get("spin-path-info") {
        Some(p) => {
            if p == "/" {
                DEFAULT_INDEX.to_owned()
            } else {
                p.to_str()?.to_owned()
            }
        }
        None => DEFAULT_INDEX.to_owned(),
    };

    eprintln!("Request path: {}", &path_info);
    let store = Store::open_default()?;

    if !disable_cache {
        let cached_content = store.get(path_info.clone());
        if let Ok(rendered) = cached_content {
            let data: RenderedPageCacheEntry = serde_json::from_slice(&rendered)?;
            if Utc::now() < data.expiry_time {
                let content_encoding = req.headers().get(http::header::ACCEPT_ENCODING);
                return response::send_result(
                    path_info,
                    data.content,
                    DEFAULT_CONTENT_TYPE.to_owned(),
                    content_encoding,
                    None,
                );
            }
        }
    }

    // Load the site config.
    let mut config: template::SiteInfo = toml::from_slice(&std::fs::read(CONFIG_FILE)?)?;
    let dynamic_templates = config
        .dynamic_templates
        .clone()
        .unwrap_or_default()
        .into_iter()
        .map(|t| {
            (
                t.dynamic_content_url,
                DynamicTemplateParams {
                    dynamic_content_path: t.dynamic_content_path,
                    dynamic_template_name: t.dynamic_template_name,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    // check if path exists in dynamic templates list
    let dynamic_template_params = dynamic_templates.get(&path_info as &str);

    let base_url = std::env::var(BASE_URL_ENV);

    if let Ok(..) = base_url {
        config.base_url = Some(base_url.unwrap());
    }
    eprintln!("Base URL: {:?}", &config.base_url);

    // If a theme is specifed, create theme path
    let theme_dir = if config.theme.is_some() {
        let mut path: PathBuf = PathBuf::from(THEME_PATH);
        path.push(config.theme.as_ref().unwrap());
        Some(path)
    } else {
        None
    };

    let mut engine = template::Renderer::new(
        PathBuf::from(TEMPLATE_PATH),
        theme_dir,
        PathBuf::from(SCRIPT_PATH),
        PathBuf::from(CONTENT_PATH),
    );

    // If running in preview mode, show unpublished content.
    engine.show_unpublished = preview_mode;
    engine.disable_cache = disable_cache;

    // Load the template directory.
    engine.load_template_dir()?;

    // Load the scripts.
    // This is currently a separate call because it might be disabled or optionally
    // enabled by the user in the future.
    engine.load_script_dir()?;

    // Load the content.
    let content_path = match dynamic_template_params {
        Some(val) => content::content_path(PathBuf::from(CONTENT_PATH), &val.dynamic_content_path),
        None => content::content_path(PathBuf::from(CONTENT_PATH), &path_info),
    };

    eprintln!("Path {}", content_path.to_string_lossy());

    match std::fs::read_to_string(&content_path) {
        Ok(full_document) => {
            let mut doc: content::Content = full_document.parse()?;

            // Hide unpublished content unless PREVIEW_MODE is on.
            if !doc.published && !preview_mode {
                eprintln!(
                    "WARNING: Unpublished document was requested. {}",
                    &path_info
                );
                let err_vals =
                    template::error_values("Not Found", "The requested page was not found.");
                let body = engine.render_template(err_vals, config, req.headers().to_owned())?;
                return response::not_found(path_info, body);
            }

            let status_opt = doc.head.status;
            let loc_opt = doc.head.redirect.clone();

            match loc_opt {
                Some(location) => {
                    let status =
                        status_opt.unwrap_or_else(|| http::StatusCode::MOVED_PERMANENTLY.as_u16());
                    response::send_redirect(path_info, location, status)
                }
                None => {
                    let content_type = doc
                        .head
                        .content_type
                        .clone()
                        .unwrap_or_else(|| DEFAULT_CONTENT_TYPE.to_owned());

                    if let Some(val) = dynamic_template_params {
                        doc.head.template = Some(val.dynamic_template_name.to_owned());
                    }
                    let disable_page_cache = doc.head.disable_page_cache;

                    let data = engine
                        .render_template(doc, config, req.headers().to_owned())
                        .map_err(|e| anyhow!("Rendering {:?}: {}", &content_path, e))?;

                    // If global and page specific caching has not been turned off by default, cache rendered content
                    if !disable_cache && !disable_page_cache.unwrap_or_default() {
                        cache_rendered_page(&store, &path_info, data.clone())?;
                    }

                    let content_encoding = req.headers().get(http::header::ACCEPT_ENCODING);

                    response::send_result(path_info, data, content_type, content_encoding, None)
                }
            }
        }
        Err(_) => {
            let err_vals = template::error_values("Not Found", "The requested page was not found.");
            let body = engine.render_template(err_vals, config, req.headers().to_owned())?;
            response::not_found(path_info, body)
        }
    }
}

// This function caches the rendered content
// along with the expiry time in the cache
// unless the page has been opted out
fn cache_rendered_page(store: &Store, path_info: &str, rendered_content: String) -> Result<()> {
    let index_cache = all_pages_load(PathBuf::from_str(CONTENT_PATH)?, false)?;
    let rendered_cache = RenderedPageCacheEntry {
        expiry_time: index_cache.cache_expiration,
        content: rendered_content,
    };
    store.set(path_info, serde_json::to_string(&rendered_cache)?)?;
    Ok(())
}

use crate::content;
use crate::response::{self, *};
use crate::template::{self};
use anyhow::{anyhow, Result};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use std::path::PathBuf;

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
    let mut path_info = match req.header("spin-path-info").and_then(|hv| hv.as_str()) {
        Some(p) => {
            if p == "/" {
                DEFAULT_INDEX.to_owned()
            } else {
                p.to_owned()
            }
        }
        None => DEFAULT_INDEX.to_owned(),
    };
    eprintln!("Request path: {}", &path_info);

    // Load the site config.
    let mut config: template::SiteInfo = toml::from_slice(&std::fs::read(CONFIG_FILE)?)?;

    let base_url = std::env::var(BASE_URL_ENV);

    if let Ok(url) = base_url {
        config.base_url = Some(url);
    }

    if let Some(url) = &config.base_url {
        eprintln!("Base URL: {}", url);
    };
    eprintln!("Prepend route info: {:?}", &config.prepend_route_info);

    if config.prepend_route_info {
        let route_info = match req.header("spin-component-route") {
            Some(route) => route.as_str().unwrap_or_default(),
            None => "",
        };
        path_info = format!("{route_info}{path_info}");
        eprintln!("Updated request path: {:?}", path_info);
    }

    // If a theme is specified, create theme path
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
    let content_path = content::content_path(PathBuf::from(CONTENT_PATH), &path_info);

    eprintln!("Path: {}", content_path.to_string_lossy());

    match std::fs::read_to_string(&content_path) {
        Ok(full_document) => {
            let mut doc: content::Content = full_document.parse()?;

            // If a dynamic content source if specified get the body from that file instead
            if let Some(val) = &doc.head.body_source {
                match std::fs::read_to_string(content::content_path(
                    PathBuf::from(CONTENT_PATH),
                    val,
                )) {
                    Ok(alternate_content) => {
                        let new_content: content::Content = alternate_content.parse()?;
                        doc.body = new_content.body;
                    }
                    Err(err) => {
                        eprintln!("Failed to read body source: {err}");
                        let err_vals = template::error_values(
                            "Not Found",
                            "The requested page was not found.",
                        );
                        let body = engine.render_template(err_vals, config, req.headers())?;
                        return response::not_found(path_info, body);
                    }
                }
            }

            // Hide unpublished content unless PREVIEW_MODE is on.
            if !doc.published && !preview_mode {
                eprintln!(
                    "WARNING: Unpublished document was requested. {}",
                    &path_info
                );
                let err_vals =
                    template::error_values("Not Found", "The requested page was not found.");
                let body = engine.render_template(err_vals, config, req.headers())?;
                return response::not_found(path_info, body);
            }

            let status_opt = doc.head.status;
            let loc_opt = doc.head.redirect.clone();
            if doc.head.path_info.is_none() {
                doc.head.path_info = Some(path_info.clone());
            }
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

                    let cache_control = doc.head.cache_control.clone();

                    let data = engine
                        .render_template(doc, config, req.headers())
                        .map_err(|e| anyhow!("Rendering {:?}: {}", &content_path, e))?;

                    let content_encoding = req.header(http::header::ACCEPT_ENCODING.as_str());

                    response::send_result(
                        path_info,
                        data,
                        content_type,
                        cache_control,
                        content_encoding,
                        None,
                    )
                }
            }
        }
        Err(_) => {
            let err_vals = template::error_values("Not Found", "The requested page was not found.");
            let body = engine.render_template(err_vals, config, req.headers())?;
            response::not_found(path_info, body)
        }
    }
}

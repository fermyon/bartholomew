use bartholomew::{content, response, template};
use std::path::PathBuf;

const CONTENT_PATH: &str = "/content/";
const TEMPLATE_PATH: &str = "/templates/";
const SCRIPT_PATH: &str = "/scripts/";
const CONFIG_FILE: &str = "/config/site.toml";

const DEFAULT_INDEX: &str = "/index";
const DEFAULT_CONTENT_TYPE: &str = "text/html; charset=utf-8";

const DEFAULT_500_ERR: &str = "An internal error occurred";
const DEFAULT_3XX_CODE: &str = "301 Moved Permanently";

fn main() {
    let debug_mode = match std::env::var("SHOW_DEBUG") {
        Ok(mode) => mode == "1",
        Err(_) => false,
    };
    match exec() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Internal Server Error: {}", e);
            let msg = if debug_mode {
                e.to_string()
            } else {
                DEFAULT_500_ERR.to_owned()
            };
            response::internal_server_error(msg)
        }
    }
}

/// The main entrypoint. This is executed for each HTTP request.
fn exec() -> anyhow::Result<()> {
    // Preview mode lets you see content marked unpublished.
    let preview_mode = match std::env::var("PREVIEW_MODE") {
        Ok(val) if val == "1" => {
            eprintln!("INFO: Bartholomew is running in PREVIEW_MODE");
            true
        }
        _ => false,
    };

    // Get the path from the WAGI env vars
    let path_info = match std::env::var("PATH_INFO") {
        Ok(path) => {
            if path == "/" {
                DEFAULT_INDEX.to_owned()
            } else {
                path
            }
        }
        Err(_) => "/index".to_owned(),
    };
    eprintln!("Request path: {}", &path_info);

    // Load the site config
    let raw_config = std::fs::read(CONFIG_FILE)?;
    let mut config: template::SiteInfo = toml::from_slice(&raw_config)?;

    let base_url = std::env::var("BASE_URL");
    if let Ok(..) = base_url {
        config.base_url = Some(base_url.unwrap());
    }
    eprintln!("Base URL: {:?}", &config.base_url);

    let mut engine = template::Renderer::new(
        PathBuf::from(TEMPLATE_PATH),
        PathBuf::from(SCRIPT_PATH),
        PathBuf::from(CONTENT_PATH),
    );

    // If we are in preview mode, show unpublished content.
    engine.show_unpublished = preview_mode;

    // Load the template directory
    engine.load_template_dir()?;

    // Load scripts
    // Right now, I have this as a separate call b/c I might disable it, or add an
    // option for a user to enable/disable.
    engine.load_script_dir()?;

    // Load the content
    let content_path = content::content_path(PathBuf::from(CONTENT_PATH), &path_info);
    eprintln!("Path {}", content_path.to_string_lossy());
    match std::fs::read_to_string(&content_path) {
        Ok(full_document) => {
            let doc: content::Content = full_document.parse()?;

            // Hide unpublished content unless PREVIEW_MODE is on.
            if !doc.published && !preview_mode {
                eprintln!(
                    "WARNING: Unpublished document was requested. {}",
                    &path_info
                );
                let err_vals =
                    template::error_values("Not Found", "The requested page was not found.");
                let body = engine.render_template(err_vals, config)?;
                response::not_found(path_info, body);
                return Ok(());
            }

            let status_opt = doc.head.status.clone();
            let loc_opt = doc.head.redirect.clone();

            match loc_opt {
                Some(location) => {
                    let status = status_opt.unwrap_or_else(|| DEFAULT_3XX_CODE.to_owned());
                    response::send_redirect(path_info, location, status);
                }
                None => {
                    let content_type = doc
                        .head
                        .content_type
                        .clone()
                        .unwrap_or_else(|| DEFAULT_CONTENT_TYPE.to_owned());

                    let data = engine
                        .render_template(doc, config)
                        .map_err(|e| anyhow::anyhow!("Rendering {:?}: {}", &content_path, e))?;
                    
                    response::send_result(path_info, data, content_type, status_opt);
                }
            }

            Ok(())
        }
        Err(_) => {
            let err_vals = template::error_values("Not Found", "The requested page was not found.");
            let body = engine.render_template(err_vals, config)?;
            response::not_found(path_info, body);
            Ok(())
        }
    }
}

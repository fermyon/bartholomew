use std::path::{Path, PathBuf};

use cgi::{cgi_try_main, html_response, Request, Response};

mod content;
mod template;

const CONTENT_PATH: &str = "/content/";
const TEMPLATE_PATH: &str = "/templates/";
const SCRIPT_PATH: &str = "/scripts/";

const DEFAULT_INDEX: &str = "/index";

// Generate the top-level handler for running this CGI script.
cgi_try_main!(exec);

/// The main entrypoint. This is executed for each HTTP request.
fn exec(request: Request) -> anyhow::Result<Response> {
    let path_info = match request.headers().get("X-CGI-Path-Info") {
        Some(header) => {
            let path = header.to_str()?;
            if path == "/" {
                DEFAULT_INDEX
            } else {
                path
            }
        }
        None => "/index",
    };

    // Load the template directory
    let mut engine = template::Renderer::new(
        PathBuf::from(TEMPLATE_PATH),
        PathBuf::from(SCRIPT_PATH),
        PathBuf::from(CONTENT_PATH),
    );
    engine.load_template_dir()?;

    // Right now, I have this as a separate call b/c I might disable it, or add an
    // option for a user to enable/disable.
    engine.load_script_dir()?;

    let verb = request
        .headers()
        .get("X-CGI-Request-Method")
        .map(|hv| hv.to_str().unwrap_or("GET"))
        .unwrap_or("GET");
    let content_path = content::content_path(PathBuf::from(CONTENT_PATH), path_info);
    eprintln!("Loading {}", content_path.to_string_lossy());
    match std::fs::read_to_string(content_path) {
        Ok(full_document) => {
            eprintln!("200 {} {}", verb, path_info);
            let doc: content::Content = full_document.parse()?;
            let data = engine.render_template(doc)?;
            Ok(html_response(200, data))
        }
        Err(_) => {
            eprintln!("404 {} {}", verb, path_info);
            let err_vals = template::error_values("Not Found", "The requested page was not found.");
            let body = engine.render_template(err_vals)?;
            Ok(html_response(404, body))
        }
    }
}

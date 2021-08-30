use std::path::PathBuf;

mod content;
mod template;

const CONTENT_PATH: &str = "/content/";
const TEMPLATE_PATH: &str = "/templates/";
const SCRIPT_PATH: &str = "/scripts/";
const CONFIG_FILE: &str = "/config/site.toml";

const DEFAULT_INDEX: &str = "/index";

fn main() {
    match exec() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Internal Server Error: {}", e);
            internal_server_error()
        }
    }
}

/// The main entrypoint. This is executed for each HTTP request.
fn exec() -> anyhow::Result<()> {
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

    // Load the site config
    let raw_config = std::fs::read(CONFIG_FILE)?;
    let config: template::SiteInfo = toml::from_slice(&raw_config)?;

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
    let content_path = content::content_path(PathBuf::from(CONTENT_PATH), &path_info);
    eprintln!("Loading {}", content_path.to_string_lossy());
    match std::fs::read_to_string(content_path) {
        Ok(full_document) => {
            let doc: content::Content = full_document.parse()?;
            let data = engine.render_template(doc, config)?;
            html_ok(path_info, data);
            Ok(())
        }
        Err(_) => {
            let err_vals = template::error_values("Not Found", "The requested page was not found.");
            let body = engine.render_template(err_vals, config)?;
            not_found(path_info, body);
            Ok(())
        }
    }
}

fn not_found(route: String, body: String) {
    eprintln!("Not Found: {}", route);
    println!("Content-Type: text/html; charset=utf-8");
    println!("Status: 404 Not Found\n");
    println!("{}", body);
}

fn internal_server_error() {
    println!("Content-Type: text/plain");
    println!("Status: 500 Internal Server Error\n");
    println!("In internal error occurred");
}

fn html_ok(route: String, body: String) {
    eprintln!("OK: {}", route);
    println!("Content-Type: text/html; charset=utf-8\n");
    println!("{}", body);
}

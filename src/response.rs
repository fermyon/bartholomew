use anyhow::{bail, Result};
use flate2::{
    write::{DeflateEncoder, GzEncoder},
    Compression,
};
use spin_sdk::http::Response;
use std::io::Write;

pub const CONTENT_PATH: &str = "/content/";
pub const TEMPLATE_PATH: &str = "/templates/";
pub const THEME_PATH: &str = "/themes/";
pub const SCRIPT_PATH: &str = "/scripts/";
pub const CONFIG_FILE: &str = "/config/site.toml";

pub const DEFAULT_INDEX: &str = "/index";
pub const DEFAULT_CONTENT_TYPE: &str = "text/html; charset=utf-8";

pub const PREVIEW_MODE_ENV: &str = "PREVIEW_MODE";
pub const BASE_URL_ENV: &str = "BASE_URL";

#[derive(PartialEq)]
enum ContentEncoding {
    Gzip,
    Deflate,
    None,
}

pub fn not_found(route: String, body: String) -> Result<Response> {
    eprintln!("Not found: {route}");
    let mut response = Response::new(http::StatusCode::NOT_FOUND.as_u16(), body);
    response.set_header(
        http::header::CONTENT_TYPE.to_string(),
        "text/html; charset=utf-8",
    );
    Ok(response)
}

pub fn internal_server_error(body: String) -> Result<Response> {
    let mut response = Response::new(http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(), body);
    response.set_header(
        http::header::CONTENT_TYPE.to_string(),
        "text/html; charset=utf-8",
    );
    Ok(response)
}

pub fn send_result(
    route: String,
    body: String,
    content_type: String,
    cache_control: Option<String>,
    content_encoding: Option<&spin_sdk::http::HeaderValue>,
    status: Option<u16>,
) -> Result<Response> {
    eprintln!("Responded: {route}");
    let mut bldr = Response::builder();
    bldr.header(http::header::CONTENT_TYPE.as_str(), content_type);

    if let Some(val) = cache_control {
        bldr.header(http::header::CACHE_CONTROL.as_str(), val);
    }
    if let Some(status) = status {
        bldr.status(status);
    }

    match parse_encoding(content_encoding) {
        Ok(enc) => match enc {
            ContentEncoding::Gzip => {
                bldr.header(http::header::CONTENT_ENCODING.as_str(), "gzip");
                let mut e = GzEncoder::new(Vec::new(), Compression::default());
                e.write_all(body.as_bytes())?;
                bldr.body(e.finish()?);
                Ok(bldr.build())
            }
            ContentEncoding::Deflate => {
                bldr.header(http::header::CONTENT_ENCODING.as_str(), "deflate");
                let mut e = DeflateEncoder::new(Vec::new(), Compression::default());
                e.write_all(body.as_bytes())?;
                bldr.body(e.finish()?);
                Ok(bldr.build())
            }
            _ => {
                bldr.body(body);
                Ok(bldr.build())
            }
        },
        Err(e) => bail!(e),
    }
}

pub fn send_redirect(route: String, location: String, status: u16) -> Result<Response> {
    eprintln!("Redirected {} to {} (Code: {})", route, &location, &status);

    let response = Response::builder()
        .status(status)
        .header(
            http::header::CONTENT_TYPE.as_str(),
            "text/html; charset=utf-8",
        )
        .header(http::header::LOCATION.as_str(), location)
        .build();

    Ok(response)
}

/// Based on the Accept-Encoding header, return the best Content-Encoding.
fn parse_encoding(enc: Option<&spin_sdk::http::HeaderValue>) -> Result<ContentEncoding> {
    let res = match enc.and_then(|hv| hv.as_str()) {
        Some(encoding) => encoding
            .split(',')
            .map(|s| {
                let enc = s.trim();
                match enc {
                    "gzip" => ContentEncoding::Gzip,
                    "deflate" => ContentEncoding::Deflate,
                    _ => ContentEncoding::None,
                }
            })
            .reduce(|a, b| {
                if a == ContentEncoding::Gzip || b == ContentEncoding::Gzip {
                    return ContentEncoding::Gzip;
                }
                if a != ContentEncoding::None {
                    a
                } else {
                    b
                }
            })
            .unwrap_or(ContentEncoding::None),
        None => ContentEncoding::None,
    };

    Ok(res)
}

use anyhow::{bail, Result};
use flate2::{
    write::{DeflateEncoder, GzEncoder},
    Compression,
};
use http::{response::Builder, HeaderValue};
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
    eprintln!("Not found: {}", route);
    let bldr = Builder::new()
        .status(http::StatusCode::NOT_FOUND)
        .header(http::header::CONTENT_TYPE, "text/html; charset=utf-8");
    Ok(bldr.body(Some(body.into()))?)
}

pub fn internal_server_error(body: String) -> Result<Response> {
    let bldr = Builder::new()
        .status(http::StatusCode::INTERNAL_SERVER_ERROR)
        .header(http::header::CONTENT_TYPE, "text/html; charset=utf-8");
    Ok(bldr.body(Some(body.into()))?)
}

pub fn send_result(
    route: String,
    body: String,
    content_type: String,
    content_encoding: Option<&HeaderValue>,
    status: Option<u16>,
) -> Result<Response> {
    eprintln!("Responded: {}", route);
    let mut bldr = Builder::new().header(http::header::CONTENT_TYPE, content_type);
    if let Some(status) = status {
        bldr = bldr.status(status);
    }

    match parse_encoding(content_encoding) {
        Ok(enc) => match enc {
            ContentEncoding::Gzip => {
                bldr = bldr.header(http::header::CONTENT_ENCODING, "gzip");
                let mut e = GzEncoder::new(Vec::new(), Compression::default());
                e.write_all(body.as_bytes())?;
                Ok(bldr.body(Some(e.finish()?.into()))?)
            }
            ContentEncoding::Deflate => {
                bldr = bldr.header(http::header::CONTENT_ENCODING, "deflate");
                let mut e = DeflateEncoder::new(Vec::new(), Compression::default());
                e.write_all(body.as_bytes())?;
                Ok(bldr.body(Some(e.finish()?.into()))?)
            }
            _ => Ok(bldr.body(Some(body.into()))?),
        },
        Err(e) => bail!(e),
    }
}

pub fn send_redirect(route: String, location: String, status: u16) -> Result<Response> {
    eprintln!("Redirected {} to {} (Code: {})", route, &location, &status);

    let bldr = Builder::new()
        .status(status)
        .header(http::header::CONTENT_TYPE, "text/html; charset=utf-8")
        .header(http::header::LOCATION, location);

    Ok(bldr.body(None)?)
}

/// Based on the Accept-Encoding header, return the best Content-Encoding.
fn parse_encoding(enc: Option<&HeaderValue>) -> Result<ContentEncoding> {
    let res = match enc {
        Some(encoding) => encoding
            .to_str()?
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

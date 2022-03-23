use brotli::CompressorWriter;
use flate2::write::{DeflateEncoder, GzEncoder};
use flate2::Compression;
use std::io::{self, Write};

#[derive(PartialEq)]
enum ContentEncoding {
    Gzip,
    Brotli,
    Deflate,
    None,
}

// 5 is better compression, but
const BROTLI_LEVEL: u32 = 3;

pub fn not_found(route: String, body: String) {
    eprintln!("Not Found: {}", route);
    println!("Content-Type: text/html; charset=utf-8");
    println!("Status: 404 Not Found\n");
    println!("{}", body);
}

pub fn internal_server_error(body: String) {
    println!("Content-Type: text/plain");
    println!("Status: 500 Internal Server Error\n");
    println!("{}", body);
}

// This function is getting a little gnarly.
pub fn send_result(route: String, body: String, content_type: String, status_opt: Option<String>) {
    eprintln!("responded: {}", route);

    // Intentionally do not override the Wagi default behavior with a default Bartholomew message.
    if let Some(status) = status_opt {
        println!("Status: {}", status);
    }
    println!("Content-Type: {}", content_type);

    match content_encoding() {
        ContentEncoding::Gzip => {
            println!("content-encoding: gzip\n");
            let mut e = GzEncoder::new(Vec::new(), Compression::default());
            e.write_all(body.as_bytes()).unwrap();
            if let Err(e) = io::stdout().write_all(&e.finish().unwrap()) {
                eprintln!("Error gzipping: {}", e)
            }
        }
        ContentEncoding::Brotli => {
            println!("content-encoding: br\n");
            let out = std::io::stdout();
            let mut input = CompressorWriter::new(out, 4096, BROTLI_LEVEL, 20);
            if let Err(e) = input.write(body.as_bytes()) {
                eprintln!("Error compressing to Brotli: {}", e);
            }
        }
        ContentEncoding::Deflate => {
            println!("content-encoding: deflate\n");
            let mut e = DeflateEncoder::new(Vec::new(), Compression::default());
            e.write_all(body.as_bytes()).unwrap();
            if let Err(e) = io::stdout().write_all(&e.finish().unwrap()) {
                eprintln!("Error deflating: {}", e)
            }
        }
        _ => {
            println!("\n{}", body);
        }
    }
}

pub fn send_redirect(route: String, location: String, status: String) {
    eprintln!("redirected {} to {} (Code: {})", route, &location, &status);
    println!("Status: {}\nLocation: {}\n", status, location)
}

/// Based on the Accept-Encoding header, return the best Content-Encoding
fn content_encoding() -> ContentEncoding {
    match std::env::var("HTTP_ACCEPT_ENCODING") {
        Ok(encoding) => {
            encoding
                .split(',')
                .map(|s| {
                    let enc = s.trim();
                    match enc {
                        "gzip" => ContentEncoding::Gzip,
                        "br" => ContentEncoding::Brotli,
                        "deflate" => ContentEncoding::Deflate,
                        _ => ContentEncoding::None,
                    }
                })
                .reduce(|a, b| {
                    // Right now, order is Brotli, Gzip, anything else.
                    if a == ContentEncoding::Brotli || b == ContentEncoding::Brotli {
                        return ContentEncoding::Brotli;
                    }
                    if a == ContentEncoding::Gzip || b == ContentEncoding::Gzip {
                        return ContentEncoding::Gzip;
                    }
                    if a != ContentEncoding::None {
                        a
                    } else {
                        b
                    }
                })
                .unwrap_or(ContentEncoding::None)
        }
        _ => ContentEncoding::None,
    }
}

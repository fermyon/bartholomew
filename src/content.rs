use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fs::{read_dir, DirEntry, File};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use pulldown_cmark as markdown;

use crate::template::PageValues;

use handlebars::Handlebars;

const DOC_SEPARATOR: &str = "\n---\n";
// Cache for page values to reduce IO on each request
const CACHE_FILE: &str = "/config/_cache.json";

const SHORTCODE_PATH: &str = "/shortcodes/";

/// Head contains the front matter for a document
#[derive(Default, Deserialize, Serialize)]
pub struct Head {
    /// The title of the document
    pub title: String,
    // The publication date
    pub date: Option<DateTime<Utc>>,
    /// A short description of the document
    pub description: Option<String>,
    /// The template to be used. If None, the `main` template is used.
    pub template: Option<String>,
    /// An explicit flag to control publish state.
    ///
    /// If this is set, it will explicitly set the publish state of a piece of content.
    /// If this is None, then the publish state will be derived from things like date.
    pub published: Option<bool>,
    /// Article tags
    ///
    /// In the template language, having an Option<Vec<>> can get really confusing,
    /// so the deserializer just creates an empty vec if not present in the source
    /// doc.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Content type overrides the MIME/media type of the body.
    ///
    /// This should only be used if the referenced `template` will generate a type other
    /// than HTML. For examle, a site map may use text/xml and a robots.txt may use text/plain.
    ///
    /// This may result in HTTP headers being altered.
    pub content_type: Option<String>,
    /// An optional status line
    ///
    /// This should only ever be set if the status code is _not_ 200.
    /// It can be used for redirects (3xx), intentional error messages (4xx, 5xx) or
    /// for specialized responses (2xx). It should not be used for 1xx codes unless
    /// you really know what you are doing.
    pub status: Option<u16>,
    /// A fully qualified URL to another resources.
    ///
    /// If no status code is set, this will set the status code to 301 Moved Permanently
    pub redirect: Option<String>,
    /// If set to true, this will enable shortcode support for the document
    pub enable_shortcodes: Option<bool>,
    /// A map of string/string pairs that are user-customizable.
    pub extra: Option<HashMap<String, String>>,
}

/// Given a PATH_INFO variable, transform it into a path for a specific markdown file
/// within the content directory.
pub fn content_path(content_dir: PathBuf, path_info: &str) -> PathBuf {
    // Add .md
    let buf = PathBuf::from(path_info).with_extension("md");
    // PATH_INFO must have a leading slash. So we strip that.
    content_dir.join(buf.strip_prefix("/").unwrap_or(&buf))
}

pub fn all_pages(
    dir: PathBuf,
    show_unpublished: bool,
    skip_cache: bool,
) -> anyhow::Result<BTreeMap<String, PageValues>> {
    if skip_cache {
        let index_cache: IndexCache = all_pages_load(dir, show_unpublished)?;
        return Ok(index_cache.contents);
    }

    // Try loading the cached object:
    let cache = PathBuf::from(CACHE_FILE);
    let cache_contents_file = File::open(&cache);
    // Assume cache is invalid until expiry time is verified
    let mut valid_cache = false;
    let contents = match cache_contents_file {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut content = String::new();
            let mut cache_expiry_time = String::new();
            // The first line of the cache contents file contains the expiry timestamp
            // The value on the first line of the cache contents is 0 if there is no expiry of the cache
            // A value 0 is used because it fails to be parsed into  a DateTime object
            let read_first_line = reader.read_line(&mut cache_expiry_time);
            if read_first_line.is_ok() {
                let cache_invalidation_time =
                    DateTime::parse_from_rfc3339(cache_expiry_time.trim());
                if let Ok(val) = cache_invalidation_time {
                    if Utc::now() < val {
                        let result = reader.read_to_string(&mut content);
                        if result.is_ok() {
                            // if cache is valid and read properly set it to true
                            valid_cache = true;
                        }
                    }
                }
            }
            content
        }
        _ => "".to_string(),
    };
    match valid_cache {
        true => {
            // We have the whole site here.
            serde_json::from_str(&contents)
                .map_err(|e| anyhow::anyhow!("Failed to parse page cache TOML: {}", e))
        }
        false => {
            let index_cache: IndexCache = all_pages_load(dir, show_unpublished)?;
            let cache_expiry_time = match index_cache.cache_expiration {
                Some(date) => date.to_rfc3339(),
                _ => "0".to_string(),
            };
            // Serialize the files back out to disk for subsequent requests.
            let cache_data = serde_json::to_string(&index_cache.contents)?;

            let cache_file = File::create(CACHE_FILE);
            match cache_file {
                Ok(f) => {
                    let mut f = BufWriter::new(f);
                    println!("Writing string  is {}", cache_expiry_time);
                    println!(
                        "Writing Expiry Time is {:#?}",
                        DateTime::parse_from_rfc3339(&cache_expiry_time)
                    );
                    // Cache_contents is stored in a file in the following manner
                    // Cache expiry timer on the 1st line
                    // The rest of the file contains the cache contents
                    writeln!(f, "{}", cache_expiry_time)?;
                    write!(f, "{}", cache_data)?;
                }
                _ => {
                    eprintln!("Cannot create Cache file");
                }
            }
            Ok(index_cache.contents)
        }
    }
}

pub struct IndexCache {
    contents: BTreeMap<String, PageValues>,
    cache_expiration: Option<DateTime<Utc>>,
}

/// Fetch all pages from disk.
///
/// If show_unpublished is `true`, this will include pages that Bartholomew has determined are
/// unpublished.
pub fn all_pages_load(dir: PathBuf, show_unpublished: bool) -> anyhow::Result<IndexCache> {
    let files = all_files(dir)?;
    let mut contents = BTreeMap::new();
    let mut contains_unpublished: bool = false;
    let mut earliest_unpublished: Option<DateTime<Utc>> = None;
    for f in files {
        // Dotfiles should not be loaded.
        if f.file_name()
            .map(|f| f.to_string_lossy().starts_with('.'))
            .unwrap_or(false)
        {
            eprintln!("Skipping dotfile {:?}", f);
            continue;
        }
        let raw_data = std::fs::read_to_string(&f)
            .map_err(|e| anyhow::anyhow!("File is not string data: {:?}: {}", &f, e))?;
        match raw_data.parse::<Content>() {
            Ok(content) => {
                if show_unpublished || content.published {
                    contents.insert(f.to_string_lossy().to_string(), content.into());
                } else {
                    // find earliest unpublished article to save timestamp to refresh cache
                    let article_date = content.head.date;
                    match contains_unpublished {
                        true => {
                            if match earliest_unpublished {
                                Some(val) => article_date.map(|d| d <= val).unwrap_or(true),
                                _ => false,
                            } {
                                earliest_unpublished = article_date;
                            }
                        }
                        false => {
                            if let Some(val) = article_date {
                                if val > Utc::now() {
                                    earliest_unpublished = article_date;
                                    contains_unpublished = true;
                                }
                            };
                        }
                    }
                }
            }
            Err(e) => {
                // If a parse fails, don't take down the entire site. Just skip this piece of content.
                eprintln!("File {:?}: {}", &f, e);
                continue;
            }
        }
    }
    Ok(IndexCache {
        contents,
        cache_expiration: earliest_unpublished,
    })
}

/// Fetch a list of paths to every file in the directory
///
/// Note that this will return files that contain unpublished content, as publish state cannot be determined
/// until a file has been read.
pub fn all_files(dir: PathBuf) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = vec![];
    let mut cb = |d: &DirEntry| {
        files.push(d.path());
    };
    visit_files(dir, &mut cb)?;
    Ok(files)
}

/// Helper for all_content_files
fn visit_files(dir: PathBuf, cb: &mut dyn FnMut(&DirEntry)) -> anyhow::Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_files(path, cb)?;
            } else {
                cb(&entry)
            }
        }
    }
    Ok(())
}

/// Translate links of the form "./<name>.md" and "<name>.md" into "/<name>"
fn maybe_translate_relative_link(dest: markdown::CowStr) -> markdown::CowStr {
    //if absolute url, return as is
    let re = Regex::new(r"^(?:[a-z+]+:)?//").unwrap();
    if re.is_match(&dest) {
        return dest;
    };

    let firstleg = dest.replace(".md", "");

    let result = match firstleg.strip_prefix("./") {
        Some(val) => val,
        _ => firstleg.as_str(),
    };

    result.to_string().into()
}

/// Look for relative Markdown links of the form "./<name>.md" or "<name>.md" and translate them into "/<name>"
///
/// See also `maybe_translate_relative_link`.
fn translate_relative_links(event: markdown::Event) -> markdown::Event {
    match event {
        markdown::Event::Start(markdown::Tag::Link(markdown::LinkType::Inline, dest, title)) => {
            markdown::Event::Start(markdown::Tag::Link(
                markdown::LinkType::Inline,
                maybe_translate_relative_link(dest),
                title,
            ))
        }
        _ => event,
    }
}

/// The envelope for a page's content
/// The head contains front matter, while the body is the markdown body of the document.
pub struct Content {
    /// The front matter for this content.
    pub head: Head,
    /// The unparsed Markdown for this content.
    pub body: String,
    /// The published state.
    ///
    /// Published state is determined when a new Content is created.
    /// It is determined according to the following rules:
    ///
    /// - If the front matter explicitly sets a publish state (`head.published`), this MUST reflect that.
    /// - Else if a date is present in the front matter (`head.date`), then...
    ///   - If the date is in the future, `published` is `false`
    ///   - Otherwise, it is `true`
    /// - Else content is considered to be published.
    ///
    /// Practically speaking, what this means is that content is published by default.
    pub published: bool,
}

impl Content {
    /// Create new content from a head and a body.
    ///
    /// This determines published state based on the head.
    ///
    /// The environment is copied from the system environment. This is safe when executed inside of Spin or a WASI runtime.
    pub fn new(head: Head, body: String) -> Self {
        let pdate = head.date;

        // If explicitly published in the front matter, mark it published
        // Else if date is set, and the date is not in the future, it's published...
        // Else if no date is set, mark it published
        let published = head
            .published
            .unwrap_or_else(|| pdate.map(|d| d <= Utc::now()).unwrap_or(true));

        Content {
            head,
            body,
            published,
        }
    }
    pub fn load_shortcodes_dir(
        &mut self,
        handlebar: &mut handlebars::Handlebars,
        shortcodes_dir: &Option<PathBuf>,
    ) -> anyhow::Result<()> {
        let shortcodes_dir = match shortcodes_dir {
            Some(path) => path.to_owned(),
            None => PathBuf::from(SHORTCODE_PATH),
        };
        let scripts = all_files(shortcodes_dir)?;
        for script in scripts {
            // Relative file name without extension. Note that we skip any file
            // that doesn't have this.
            if let Some(fn_name) = script.file_stem() {
                eprintln!(
                    "shortcodes: registering {}",
                    fn_name.to_str().unwrap_or("unknown")
                );
                handlebar
                    .register_script_helper_file(&fn_name.to_string_lossy(), &script)
                    .map_err(|e| anyhow::anyhow!("Shortcode {:?}: {}", &script, e))?;
            }
        }
        Ok(())
    }

    /// Render the body using a Markdown renderer
    pub fn render_markdown(&mut self, shortcodes_dir: &Option<PathBuf>) -> String {
        let mut buf = String::new();
        // Might as well turn on all the lights on the Christmas tree
        let opt = markdown::Options::all();
        let out: String;
        let parser = match self.head.enable_shortcodes {
            Some(true) => {
                let mut handlebars = Handlebars::new();
                let _ = &self.load_shortcodes_dir(&mut handlebars, shortcodes_dir);

                // don't escape HTML so that rhai scripts can return html that will
                // be rendered as HTML
                handlebars.register_escape_fn(handlebars::no_escape);

                // run the markdown through the template engine to
                // enable any script helpers
                out = handlebars
                    .render_template(&self.body, &{})
                    .unwrap_or_else(|e| {
                        // print the error
                        eprintln!("Error rendering markdown: {}", e);
                        // return nothing
                        e.to_string()
                    });
                markdown::Parser::new_ext(&out, opt).map(translate_relative_links)
            }
            _ => markdown::Parser::new_ext(&self.body, opt).map(translate_relative_links),
        };
        markdown::html::push_html(&mut buf, parser);

        buf
    }
}

impl FromStr for Content {
    type Err = anyhow::Error;
    fn from_str(full_document: &str) -> Result<Self, Self::Err> {
        // This is a heavy-handed way of normalizing the document to only use "\n"
        // for line endings. This simplifies a number of things, including Rhai scripting.
        let doc = full_document.replace("\r\n", "\n");
        let (toml_text, body) = doc
            .split_once(DOC_SEPARATOR)
            .unwrap_or(("title = 'Untitled'", &doc));
        let head: Head =
            toml::from_str(toml_text).map_err(|e| anyhow::anyhow!("TOML parsing error: {}", e))?;

        Ok(Content::new(head, body.to_owned()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! maybe_translate_relative_link_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected, msg) = $value;
                assert_eq!(markdown::CowStr::from(expected), maybe_translate_relative_link(markdown::CowStr::from(input)), "{}", msg);
            }
        )*
        }
    }

    maybe_translate_relative_link_tests! {
        //relative links
        test_relative_links_0: ("link-to-page", "link-to-page", "relative link without slash and extension"),
        test_relative_links_1: ("link-to-page.md", "link-to-page", "relative link with .md extension"),
        test_relative_links_2: ("link-to-page.md#build", "link-to-page#build", "relative link with .md extension and hash"),
        test_relative_links_3: ("link-to-page.jpg", "link-to-page.jpg", "relative link with .jpg extension"),
        test_relative_links_4: ("./link-to-page", "link-to-page", "relative link with slash"),
        test_relative_links_5: ("./link-to-page.md", "link-to-page", "relative link with slash and .md extension"),
        test_relative_links_6: ("./link-to-page.jpg", "link-to-page.jpg", "relative link with slash and .jpg extension"),

        //relative deep links
        test_relative_links_7: ("deep/link-to-page", "deep/link-to-page", "deep relative link without slash and extension"),
        test_relative_links_8: ("deep/link-to-page.md", "deep/link-to-page", "deep relative link with .md extension"),
        test_relative_links_9: ("deep/link-to-page.jpg", "deep/link-to-page.jpg", "deep relative link with .jpg extension"),
        test_relative_links_10: ("./deep/link-to-page", "deep/link-to-page", "deep relative link with slash"),
        test_relative_links_11: ("./deep/link-to-page.md", "deep/link-to-page", "deep relative link with slash and .md extension"),
        test_relative_links_12: ("./deep/link-to-page.jpg", "deep/link-to-page.jpg", "deep relative link with slash and .jpg extension"),

        //absolute links
        test_relative_links_13: ("/link-to-page", "/link-to-page", "absolute link without slash and extension"),
        test_relative_links_14: ("/link-to-page.md", "/link-to-page", "absolute link with .md extension"),
        test_relative_links_15: ("/link-to-page.jpg", "/link-to-page.jpg", "absolute link with .jpg extension"),

        //absolute links with complete urls
        test_relative_links_16: ("https://example.com/link-to-page", "https://example.com/link-to-page", "absolute url without slash and extension"),
        test_relative_links_17: ("https://example.com/link-to-page.md", "https://example.com/link-to-page.md", "absolute url with .md extension"),
        test_relative_links_18: ("https://example.com/link-to-page.jpg", "https://example.com/link-to-page.jpg", "absolute url with .jpg extension"),
        test_relative_links_19: ("https://example.com/link-to-page.md#build", "https://example.com/link-to-page.md#build", "absolute url with .md extension and hash"),
    }
}

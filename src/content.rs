use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fs::{read_dir, DirEntry};
use std::path::PathBuf;
use std::str::FromStr;

use pulldown_cmark as markdown;
use chrono::{Utc, DateTime};

use crate::template::PageValues;

const DOC_SEPERATOR: &str = "\n---\n";

// TODO: This should probably be configurable.
const CACHE_FILE: &str = "/config/_cache.json";

/// Head contains the front matter for a document
#[derive(Deserialize, Serialize)]
pub struct Head {
    /// The title of the document
    pub title: String,
    // The publication date
    pub date: Option<DateTime<Utc>>,
    /// A short description of the document
    pub description: Option<String>,
    /// The template to be used. If None, the `main` template is used.
    pub template: Option<String>,
    /// A map of string/string pairs that are user-customizable.
    pub extra: Option<HashMap<String, String>>,
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
}

/// Given a PATH_INFO variable, transform it into a path for a specific markdown file
/// within the content directory.
pub fn content_path(content_dir: PathBuf, path_info: &str) -> PathBuf {
    // Add .md
    let buf = PathBuf::from(path_info).with_extension("md");
    // PATH_INFO must have a leading slash. So we strip that.
    content_dir.join(buf.strip_prefix("/").unwrap_or(&buf))
}

/// Fetch all pages.
/// 
/// If show_unpublished is `true`, this will include pages that Bartholomew has determined are
/// unpublished.
pub fn all_pages(dir: PathBuf, show_unpublished: bool, skip_cache: bool) -> anyhow::Result<BTreeMap<String, PageValues>> {

    if skip_cache {
        return all_pages_load(dir, show_unpublished);
    }

    // Try loading the cached object:
    let cache = PathBuf::from(CACHE_FILE);
    match std::fs::read_to_string(&cache) {
        Ok(data) => {
            // We have the whole site here.
            serde_json::from_str(&data).map_err(|e| anyhow::anyhow!("Failed to parse page cache TOML: {}", e))
        },
        Err(_) => {
            let contents = all_pages_load(dir, show_unpublished)?;
            // Serialize the files back out to disk for subsequent requests.
            let cache_data = serde_json::to_string(&contents)?;
            std::fs::write(&cache, cache_data)?;
            Ok(contents)
        }
    }
}

fn all_pages_load(dir: PathBuf, show_unpublished: bool) -> anyhow::Result<BTreeMap<String, PageValues>> {
    let files = all_files(dir)?;
    let mut contents = BTreeMap::new();
    for f in files {
        let content = Content::header_from_path(f.clone()).map_err(|e|anyhow::anyhow!("File {:?}: {}", &f, e))?;
        if show_unpublished || content.published {
            contents.insert(f.to_string_lossy().to_string(), content.into());
        }
    }
    // Serialize the files back out to disk for subsequent requests.
    Ok(contents)
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

/// The envelope for a page's content
/// The head contains front matter, while the body is the markdown body of the document.
pub struct Content {
    /// The front matter for this content.
    pub head: Head,
    /// The unparsed Markdown for this content.
    body: Option<String>,
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
    /// Path to the source file
    pub path: PathBuf,
}

impl Content {

    /// Create new content from a head and a body.
    ///
    /// This determines published state based on the head.
    ///
    /// The environment is copied from the system environment. This is safe when executed inside of Wagi or a WASI runtime.
    pub fn new(head: Head, body: String, path: PathBuf) -> Self {
        let pdate = head.date.clone();

        // If explicitly published in the front matter, mark it published
        // Else if date is set, and the date is not in the future, it's published...
        // Else if no date is set, mark it published
        let published = head.published.unwrap_or_else(|| pdate.map(|d|d <= Utc::now()).unwrap_or(true));

        Content { head, body: Some(body), published, path }
    }

    /// Render the body using a Markdown renderer
    pub fn render_markdown(&self) -> anyhow::Result<String> {
        let mut buf = String::new();

        // Might as well turn on all the lights on the Christmas tree
        let opt = markdown::Options::all();
        let body = self.body()?;
        let parser = markdown::Parser::new_ext(body.as_str(), opt);
        markdown::html::push_html(&mut buf, parser);

        Ok(buf)
    }

    /// Create a Content without a body.
    pub fn header_from_path(path: PathBuf) -> anyhow::Result<Self> {
        let full_document = std::fs::read_to_string(&path)?;
        let (toml_text, _body) = full_document
            .split_once(DOC_SEPERATOR)
            .unwrap_or(("title = 'Untitled'", &full_document));
        let head: Head = toml::from_str(toml_text).map_err(|e| anyhow::anyhow!("TOML parsing error: {}", e))?;
        Ok(Self::new(head, "".to_owned(), path))
    }

    pub fn from_path(path: PathBuf) -> anyhow::Result<Self> {
        let full_document = std::fs::read_to_string(&path)?;
        let (toml_text, body) = full_document
            .split_once(DOC_SEPERATOR)
            .unwrap_or(("title = 'Untitled'", &full_document));
        let head: Head = toml::from_str(toml_text).map_err(|e| anyhow::anyhow!("TOML parsing error: {}", e))?;

        Ok(Content::new(head, body.to_owned(), path.to_owned()))
    }

    pub fn body(&self) -> anyhow::Result<String> {
        match self.body.clone() {
            Some(data) => Ok(data),
            None => {
                // Reload the content
                let full_document = std::fs::read_to_string(self.path.clone())?;
                // Parse the content
                let (_toml_text, body) = full_document
                    .split_once(DOC_SEPERATOR)
                    .unwrap_or(("title = 'Untitled'", &full_document));
                // Return the body
                Ok(body.to_owned())
            }
        }
    }
}
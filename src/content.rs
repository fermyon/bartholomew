use serde::{Deserialize, Serialize};
use std::any;
use std::collections::{BTreeMap, HashMap};
use std::fs::{read_dir, DirEntry};
use std::path::PathBuf;
use std::str::FromStr;

use pulldown_cmark as markdown;

use crate::template::PageValues;

const DOC_SEPERATOR: &str = "\n---\n";

#[derive(Deserialize, Serialize)]
pub struct Frontmatter {
    /// The title of the document
    pub title: String,
    /// A short description of the document
    pub description: Option<String>,
    /// The template to be used. If None, the `main` template is used.
    pub template: Option<String>,
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

pub fn all_pages(dir: PathBuf) -> anyhow::Result<BTreeMap<String, PageValues>> {
    let files = all_files(dir)?;
    let mut contents = BTreeMap::new();
    for f in files {
        let raw_data = std::fs::read_to_string(&f)?;
        let content: Content = raw_data.parse()?;
        contents.insert(f.to_string_lossy().to_string(), content.into());
    }
    Ok(contents)
}

/// Fetch a list of paths to every file in the directory
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

pub struct Content {
    pub frontmatter: Frontmatter,
    pub body: String,
}

impl Content {
    /// Render the body using a Markdown renderer
    pub fn render_markdown(&self) -> String {
        let mut buf = String::new();

        // Might as well turn on all the lights on the Christmas tree
        let opt = markdown::Options::all();
        let parser = markdown::Parser::new_ext(&self.body, opt);
        markdown::html::push_html(&mut buf, parser);

        buf
    }
}

impl FromStr for Content {
    type Err = anyhow::Error;
    fn from_str(full_document: &str) -> Result<Self, Self::Err> {
        let (toml_text, body) = full_document
            .split_once(DOC_SEPERATOR)
            .unwrap_or(("title = 'Untitled'", &full_document));
        let frontmatter: Frontmatter = toml::from_str(toml_text)?;
        Ok(Content {
            frontmatter,
            body: body.to_owned(),
        })
    }
}

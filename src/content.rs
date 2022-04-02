use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fs::{read_dir, DirEntry};
use std::path::PathBuf;
use std::str::FromStr;
use handlebars::Handlebars;


use chrono::{DateTime, Utc};
use pulldown_cmark as markdown;

use crate::template::PageValues;

const DOC_SEPARATOR: &str = "\n---\n";

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
    pub status: Option<String>,
    /// A fully qualified URL to another resources.
    ///
    /// If no status code is set, this will set the status code to 301 Moved Permanently
    pub redirect: Option<String>,
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

/// Fetch all pages.
///
/// If show_unpublished is `true`, this will include pages that Bartholomew has determined are
/// unpublished.
pub fn all_pages(
    dir: PathBuf,
    show_unpublished: bool,
) -> anyhow::Result<BTreeMap<String, PageValues>> {
    let files = all_files(dir)?;
    let mut contents = BTreeMap::new();
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
                }
            }
            Err(e) => {
                // If a parse fails, don't take down the entire site. Just skip this piece of content.
                eprintln!("File {:?}: {}", &f, e);
                continue;
            }
        }
    }
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
pub struct Content<'a> {
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
    pub shortcode_dir: PathBuf,
    handlebars: handlebars::Handlebars<'a>,


}

impl <'a> Content<'a>{
    /// Create new content from a head and a body.
    ///
    /// This determines published state based on the head.
    ///
    /// The environment is copied from the system environment. This is safe when executed inside of Wagi or a WASI runtime.
    pub fn new(head: Head, body: String,  shortcode_dir: PathBuf) -> Self {
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
            shortcode_dir,
            handlebars: Handlebars::new(),

        }
    }

        pub fn load_shortcode_dir(&mut self) -> anyhow::Result<()> {
        // TODO: rewrite all_files so we don't need to clone here.
        let scripts = all_files(self.shortcode_dir.clone())?;
        for script in scripts {
            // Relative file name without extension. Note that we skip any file
            // that doesn't have this.
            if let Some(fn_name) = script.file_stem() {
                eprintln!(
                    "scripts: registering {}",
                    fn_name.to_str().unwrap_or("unknown")
                );
                self.handlebars
                    .register_script_helper_file(&fn_name.to_string_lossy(), &script)
                    .map_err(|e| anyhow::anyhow!("Script {:?}: {}", &script, e))?;
            }
        }

        Ok(())
    }


    /// Render the body using a Markdown renderer
    pub fn render_markdown(&mut self) -> String {
        let mut buf = String::new();
        let _ = &self.load_shortcode_dir().unwrap();

        // don't escape HTML so that rhai scripts can return html that will
        // be rendered as HTML
        let _ = &self.handlebars.register_escape_fn(handlebars::no_escape);

        // run the markdown through the template engine to
        // enable any script helpers
        let out = self
            .handlebars
            .render_template(&self.body, &{}).unwrap_or_else(|e| {
                // print the error
                eprintln!("Error rendering markdown: {}", e);
                // return nothing
                "Template Error".to_string()
            });

        // Might as well turn on all the lights on the Christmas tree
        let opt = markdown::Options::all();
        let parser = markdown::Parser::new_ext(&out, opt);
        markdown::html::push_html(&mut buf, parser);

        buf
    }
}

impl <'a> FromStr for Content<'a> {
    type Err = anyhow::Error;
    fn from_str(full_document: &str) -> Result<Self, Self::Err> {
        let (toml_text, body) = full_document
            .split_once(DOC_SEPARATOR)
            .unwrap_or(("title = 'Untitled'", full_document));
        let head: Head =
            toml::from_str(toml_text).map_err(|e| anyhow::anyhow!("TOML parsing error: {}", e))?;

        Ok(Content::new(head, body.to_owned(),
        PathBuf::from(SHORTCODE_PATH)))


    }
}

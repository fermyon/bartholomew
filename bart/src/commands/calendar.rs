use anyhow::{anyhow, Result};
use bartholomew::content::{self, Head};
use chrono::{Duration, Utc};
use colorful::{Color, Colorful};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

pub const DOC_SEPARATOR: &str = "\n---\n";
const DATE_FORMAT: &str = "%a, %b %d, %Y";
const MAX_DAYS: i64 = 7;

/// Print the content calendar for a Bartholomew website.
#[derive(StructOpt, Debug)]
#[structopt(alias = "cal")]
pub struct CalendarCommand {
    /// The path to the Bartholomew content directory.
    #[structopt()]
    pub paths: Vec<PathBuf>,
}

impl CalendarCommand {
    /// Run the bart calendar command.
    pub async fn run(self) -> Result<()> {
        let now = Utc::now();

        // Print a content calendar for every given directory.
        for dir in self.paths {
            // Walk the content directory.
            let files = content::all_files(dir)?;
            let mut posts = vec![];
            for f in files {
                let raw_data = std::fs::read_to_string(&f)
                    .map_err(|e| anyhow::anyhow!("File is not string data: {:?}: {}", &f, e))?;

                posts.push(from_file(raw_data, &f)?);
            }

            posts.sort_by(|a, b| a.head.date.partial_cmp(&b.head.date).unwrap());

            let mut last = Utc::now();
            for post in posts {
                let date = post.head.date.unwrap_or_else(Utc::now);
                let c = if date < now {
                    Color::Green
                } else {
                    Color::Yellow
                };

                let mut dc = "".to_owned();

                let dur = date.signed_duration_since(last);
                if dur > Duration::days(MAX_DAYS) {
                    dc = format!("({})", dur.num_days())
                        .color(Color::Red)
                        .to_string();
                }

                let filename = post.path.file_name().unwrap().to_string_lossy();
                println!(
                    "{} - '{}' {} {}",
                    date.format(DATE_FORMAT),
                    post.head.title.color(c),
                    filename,
                    dc
                );
                last = date;
            }
        }

        Ok(())
    }
}

fn from_file(full_doc: String, path: &Path) -> Result<Metadata> {
    let doc = full_doc.replace("\r\n", "\n");
    let (toml_text, _) = doc
        .split_once(DOC_SEPARATOR)
        .unwrap_or(("title = 'Untitled'", &doc));
    let head: Head = toml::from_str(toml_text).map_err(|e| anyhow!("TOML parsing error: {}", e))?;
    let path = path.into();

    Ok(Metadata { path, head })
}

struct Metadata {
    path: PathBuf,
    head: Head,
}

use anyhow::Result;
use bartholomew::content::Content;
use colorful::{Color, Colorful};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

/// Check content and identify errors or warnings.
#[derive(StructOpt, Debug)]
pub struct CheckCommand {
    /// The path to check.
    #[structopt()]
    pub paths: Vec<PathBuf>,
    /// The path to directory containing shortcodes
    #[structopt(long = "shortcodes")]
    pub shortcodes_dir: Option<PathBuf>,
}

impl CheckCommand {
    pub async fn run(self) -> Result<()> {
        if self.paths.is_empty() {
            anyhow::bail!("Supply one or more content files to check.")
        }
        let mut exit_with_err = false;
        for file_path in self.paths {
            if file_path.is_dir() {
                continue;
            }
            match check_file(&file_path, &self.shortcodes_dir).await {
                Ok(()) => println!(
                    "✅ {}",
                    &file_path.to_str().unwrap_or("").color(Color::Green)
                ),
                Err(e) => {
                    println!(
                        "❌ {}\t{}",
                        &file_path.to_str().unwrap_or("").color(Color::Red),
                        e
                    );
                    exit_with_err = true;
                }
            }
        }
        if exit_with_err {
            let msg = "One or more pieces of content are invalid".color(Color::Red);
            Err(anyhow::anyhow!("{}", msg))
        } else {
            Ok(())
        }
    }
}

async fn check_file(p: &Path, shortcodes_dir: &Option<PathBuf>) -> Result<()> {
    let raw_data = std::fs::read_to_string(p)
        .map_err(|e| anyhow::anyhow!("Could not read file {:?} as a string: {}", &p, e))?;
    let mut content: Content = raw_data
        .parse()
        .map_err(|e| anyhow::anyhow!("Could not parse file {:?}: {}", &p, e))?;

    // This will catch (only) panic cases.
    let _html = content.render_markdown(shortcodes_dir);

    // Things we could do from here:
    // - Check whether requested template is known
    // - Check that date parses correctly (Actually, is done already)
    // - Warn if there is a publish date

    if content.head.title.len() == 0 {
        anyhow::bail!("Title should not be empty");
    } else if content.head.title == "Untitled" {
        anyhow::bail!("Document seems to be missing title. Is there TOML metadata?");
    }

    if let Some(tpl) = content.head.template {
        let tpl_path = Path::new("templates").join(format!("{}.hbs", &tpl));
        if let Err(e) = std::fs::metadata(&tpl_path) {
            return Err(anyhow::anyhow!(
                "Failed to open template {:?}: {}",
                tpl_path,
                e
            ));
        }
    }

    Ok(())
}

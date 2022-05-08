use crate::commands::calendar::DOC_SEPARATOR;
use anyhow::{bail, Result};
use bartholomew::content::{Content, Head};
use chrono::Utc;
use std::{collections::HashMap, path::PathBuf};
use structopt::StructOpt;
use tokio::{fs::File, io::AsyncWriteExt, process::Command};

/// Create a new page or website from a template.
#[derive(StructOpt, Debug)]
pub enum NewCommand {
    Post(NewPostCommand),
    Site(NewSiteCommand),
}

impl NewCommand {
    pub async fn run(self) -> Result<()> {
        match self {
            NewCommand::Post(cmd) => cmd.run().await,
            NewCommand::Site(cmd) => cmd.run().await,
        }
    }
}

/// Create a new post.
#[derive(StructOpt, Debug)]
pub struct NewPostCommand {
    /// Path to the directory where to create the new post.
    pub dir: PathBuf,

    /// Name of the file.
    #[structopt(default_value = "untitled.md")]
    pub file: String,
    /// Title for the post.
    #[structopt(long = "title", default_value = "Untitled")]
    pub title: String,
    /// Description for the post.
    #[structopt(long = "description")]
    pub description: Option<String>,
    /// Template for the post.
    #[structopt(long = "template", default_value = "blog")]
    pub template: String,
    /// Type of the post.
    #[structopt(long = "type", default_value = "post")]
    pub post_type: String,
    /// Author of the post.
    #[structopt(long = "author", default_value = "John Doe")]
    pub author: String,
}

impl NewPostCommand {
    pub async fn run(self) -> Result<()> {
        let template = Some(self.template);
        let title = self.title;
        let date = Some(Utc::now());

        let mut extra = HashMap::new();
        extra.insert("author".to_string(), self.author);
        extra.insert("type".to_string(), self.post_type);

        let description = self.description;
        // Published means...
        // - If 'Some(true)', always mark it published
        // - If 'Some(false)', never mark it published
        // - If 'None', use the regular publishing rules (e.g. date)
        let published = None;

        let extra = Some(extra);
        let head = Head {
            title,
            date,
            description,
            template,
            extra,
            published,
            ..Default::default()
        };

        let body = r#"
Begin with intro paragraph

<!-- Ideally, for SEO there should be an image after the first paragraph or two -->

## Headers Should Be Second-level, Not First
"#
        .to_string();

        let content = Content {
            head,
            body,
            published: true,
        };

        let content = serialize_content(&content)?;
        let path = self.dir.join(&self.file);
        let mut file = File::create(&path).await?;
        file.write_all(content.as_bytes()).await?;

        println!("Wrote new post in file {}", path.display());

        Ok(())
    }
}

/// Create a new site from a Bartholomew Git template.
#[derive(StructOpt, Debug)]
pub struct NewSiteCommand {
    /// Path to the directory where to create the new site.
    pub dir: PathBuf,

    /// Git URL for the Bartholomew template
    #[structopt(long = "git", short = "g")]
    pub git: String,

    /// Git branch for the Bartholomew template
    #[structopt(long = "branch", short = "b")]
    pub branch: Option<String>,
}

impl NewSiteCommand {
    pub async fn run(self) -> Result<()> {
        self.clone_repo().await
    }

    async fn clone_repo(&self) -> Result<()> {
        let mut git = Command::new("git");
        git.arg("clone");

        if let Some(b) = &self.branch {
            git.arg("--branch").arg(b);
        }

        let clone_result = git.arg(&self.git).arg(&self.dir).output().await?;
        match clone_result.status.success() {
            true => {
                println!(
                    "Successfully created new Bartholomew website in directory {}.",
                    &self.dir.display()
                );
                println!(
                    "Run spin up --file {}/spin.toml to start your website locally.",
                    &self.dir.display()
                );
                Ok(())
            }
            false => bail!(
                "Error cloning Git repo {}: {}",
                &self.git,
                String::from_utf8(clone_result.stderr)
                    .unwrap_or_else(|_| "(cannot get error)".to_owned())
            ),
        }
    }
}

fn serialize_content(content: &Content) -> Result<String> {
    let mut res = toml::to_string(&content.head)?;
    res.push_str(DOC_SEPARATOR);
    res.push_str(&content.body);
    Ok(res)
}

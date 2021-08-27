use std::path::PathBuf;

use super::content::{Content, Frontmatter};
use handlebars::Handlebars;
use serde::Serialize;

const DEFAULT_TEMPLATE: &str = "main";

#[derive(Serialize)]
pub struct TemplateContext {
    request: RequestValues,
    page: PageValues,
    site: SiteValues,
}

#[derive(Serialize)]
pub struct RequestValues {}

#[derive(Serialize)]
pub struct SiteValues {}

/// The structured values sent to the template renderer.
/// The body should be legal HTML that can be inserted within the <body> tag.
#[derive(Serialize)]
pub struct PageValues {
    pub frontmatter: Frontmatter,
    pub body: String,
}

impl From<Content> for PageValues {
    fn from(c: Content) -> Self {
        PageValues {
            body: c.render_markdown(),
            frontmatter: c.frontmatter,
        }
    }
}

pub struct Renderer<'a> {
    pub template_dir: PathBuf,
    pub script_dir: PathBuf,
    pub content_dir: PathBuf,
    handlebars: handlebars::Handlebars<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(template_dir: PathBuf, script_dir: PathBuf, content_dir: PathBuf) -> Self {
        Renderer {
            template_dir,
            script_dir,
            content_dir,
            handlebars: Handlebars::new(),
        }
    }
    // pub fn load(&mut self, name: &str, file: &str) -> Result<(), anyhow::Error> {
    //     let filepath = self.template_dir.join(file);
    //     self.handlebars.register_template_file(name, filepath)?;
    //     Ok(())
    // }
    pub fn load_template_dir(&mut self) -> Result<(), anyhow::Error> {
        self.handlebars
            .register_templates_directory(".hbs", &self.template_dir)?;
        Ok(())
    }

    pub fn load_script_dir(&mut self) -> anyhow::Result<()> {
        self.handlebars
            .register_script_helper_file("lib", self.script_dir.join("lib.rhai"))?;
        Ok(())
    }

    pub fn render_template<T: Into<PageValues>>(&self, values: T) -> anyhow::Result<String> {
        let page: PageValues = values.into();
        let tpl = page
            .frontmatter
            .template
            .clone()
            .unwrap_or_else(|| DEFAULT_TEMPLATE.to_owned());

        let ctx = TemplateContext {
            page,
            request: RequestValues {},
            site: SiteValues {},
        };
        let out = self.handlebars.render(&tpl, &ctx)?;
        Ok(out)
    }
}

/// Describe an error to the template engine.
///
/// It should be assumed that all data passed into this function will be visible to teh
/// end user.
pub fn error_values(title: &str, msg: &str) -> PageValues {
    PageValues {
        frontmatter: Frontmatter {
            title: title.to_string(),
            description: None,
            extra: None,
            template: None,
        },
        body: msg.to_string(),
    }
}

use std::collections::BTreeMap;
use std::path::PathBuf;

use super::content::{Content, Head};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use handlebars_sprig;

/// The name of the default template.
/// This will be resolved to $TEMPLATE_DIR/$DEFAULT_TEMPLATE.hbs
const DEFAULT_TEMPLATE: &str = "main";

/// Describe the site itself
#[derive(Serialize, Deserialize)]
pub struct SiteInfo {
    pub title: String,
    pub logo: Option<String>,
    pub base_url: Option<String>,
    pub about: Option<String>,
    pub extra: BTreeMap<String, String>,
}

/// Context for a template render.
#[derive(Serialize)]
pub struct TemplateContext {
    request: RequestValues,
    page: PageValues,
    site: SiteValues,
    /// A copy of the environment variables
    ///
    /// Unlike a static site generator, Bartholomew can make decisions based on request.
    /// This provides templates with access to the request data along with custom
    /// environment variables.
    ///
    /// TODO: Should there be a way to filter out env vars that should never get to
    /// the template layer? As of this writing, there are no such variables, but in
    /// the future there could be.
    env: BTreeMap<String, String>,
}

#[derive(Serialize)]
pub struct RequestValues {}

/// Information about the site, including site info and all of the pages.
#[derive(Serialize)]
pub struct SiteValues {
    info: SiteInfo,
    pages: BTreeMap<String, PageValues>,
}

/// The structured values sent to the template renderer.
/// The body should be legal HTML that can be inserted within the <body> tag.
#[derive(Serialize)]
pub struct PageValues {
    pub head: Head,
    pub body: String,
    pub published: bool,
}

impl From<Content> for PageValues {
    fn from(c: Content) -> Self {
        PageValues {
            body: c.render_markdown(),
            head: c.head,
            published: c.published,
        }
    }
}

/// Renderer can execute a handlebars template and render the results into HTML.
pub struct Renderer<'a> {
    pub template_dir: PathBuf,
    pub script_dir: PathBuf,
    pub content_dir: PathBuf,
    pub show_unpublished: bool,
    handlebars: handlebars::Handlebars<'a>,
}

impl<'a> Renderer<'a> {
    /// Create a new renderer with the necessary directories attached.
    pub fn new(template_dir: PathBuf, script_dir: PathBuf, content_dir: PathBuf) -> Self {
        Renderer {
            template_dir,
            script_dir,
            content_dir,
            show_unpublished: false,
            handlebars: Handlebars::new(),
        }
    }
    // pub fn load(&mut self, name: &str, file: &str) -> Result<(), anyhow::Error> {
    //     let filepath = self.template_dir.join(file);
    //     self.handlebars.register_template_file(name, filepath)?;
    //     Ok(())
    // }

    /// Load the template directory.
    pub fn load_template_dir(&mut self) -> Result<(), anyhow::Error> {
        self.register_helpers();
        self.handlebars
            .register_templates_directory(".hbs", &self.template_dir)?;
        Ok(())
    }

    /// Load the scripts directory
    pub fn load_script_dir(&mut self) -> anyhow::Result<()> {
        // TODO: rewrite all_files so we don't need to clone here.
        let scripts = crate::content::all_files(self.script_dir.clone())?;
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

    /// Given values and a site object, render a template.
    pub fn render_template<T: Into<PageValues>>(
        &self,
        values: T,
        info: SiteInfo,
    ) -> anyhow::Result<String> {
        let page: PageValues = values.into();
        let tpl = page
            .head
            .template
            .clone()
            .unwrap_or_else(|| DEFAULT_TEMPLATE.to_owned());

        let ctx = TemplateContext {
            page,
            request: RequestValues {},
            site: SiteValues {
                info,
                // Right now, we literally include ALL OF THE CONTENT in its rendered
                // state. I take some consolation in knowing how PHP works. But
                // seriously, this is probably not the best thing to do.
                //
                // Options:
                // 1. Parse only the head out of pages
                // 2. Get all of the content paths, but load lazily (perhaps by helper)
                // 3. ???
                // 4. Leave it like it is
                // 5. Determine that this is out of scope
                pages: crate::content::all_pages(self.content_dir.clone(), self.show_unpublished)?,
            },
            // Copy the WASI env into the env template var.
            env: std::env::vars().collect(),
        };
        let out = self
            .handlebars
            .render(&tpl, &ctx)
            .map_err(|e| anyhow::anyhow!("Template '{}': {}", &tpl, e))?;
        Ok(out)
    }

    /// Add all of the helper functions to this renderer.
    fn register_helpers(&mut self) {
        handlebars_sprig::addhelpers(&mut self.handlebars)
    }
}

/*
fn pages_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    // get parameter from helper or throw an error
    let param = h
        .param(0)
        .ok_or_else(|| RenderError::new("Param 0 is required for format helper."))?;
    let rendered = format!("{} pts", param.value().render());
    out.write(rendered.as_ref())?;
    Ok(())
}
*/

/// Describe an error to the template engine.
///
/// It should be assumed that all data passed into this function will be visible to the
/// end user.
pub fn error_values(title: &str, msg: &str) -> PageValues {
    PageValues {
        head: Head {
            title: title.to_string(),
            date: Some(chrono::Utc::now()),
            description: None,
            extra: None,
            template: None,
            published: None,
            tags: vec![],
            content_type: None,
            status: None,
            redirect: None,
        },
        body: msg.to_string(),
        published: true,
    }
}

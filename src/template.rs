use std::collections::BTreeMap;
use std::path::PathBuf;

use super::content::{Content, Head};
use handlebars::{
    handlebars_helper, Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError,
};
use serde::{Deserialize, Serialize};

const DEFAULT_TEMPLATE: &str = "main";

/// Describe the site itself
#[derive(Serialize, Deserialize)]
pub struct SiteInfo {
    title: String,
    logo: Option<String>,
    base_url: Option<String>,
    about: Option<String>,
    extra: BTreeMap<String, String>,
}

#[derive(Serialize)]
pub struct TemplateContext {
    request: RequestValues,
    page: PageValues,
    site: SiteValues,
}

#[derive(Serialize)]
pub struct RequestValues {}

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
}

impl From<Content> for PageValues {
    fn from(c: Content) -> Self {
        PageValues {
            body: c.render_markdown(),
            head: c.head,
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
        self.register_helpers();
        self.handlebars
            .register_templates_directory(".hbs", &self.template_dir)?;
        Ok(())
    }

    pub fn load_script_dir(&mut self) -> anyhow::Result<()> {
        // TODO: rewrite all_files so we don't need to clone here.
        let scripts = crate::content::all_files(self.script_dir.clone())?;
        for script in scripts {
            // Relative file name without extension. Note that we skip any file
            // that doesn't have this.
            if let Some(fn_name) = script.file_stem() {
                eprintln!("registering {}", fn_name.to_str().unwrap_or("unknown"));
                self.handlebars
                    .register_script_helper_file(&fn_name.to_string_lossy(), &script)?;
            }
        }

        Ok(())
    }

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
                pages: crate::content::all_pages(self.content_dir.clone())?,
            },
        };
        let out = self.handlebars.render(&tpl, &ctx)?;
        Ok(out)
    }

    fn register_helpers(&mut self) {
        // This is a mess right now. I am trying to figure out what helpers should be
        // included by default.

        //let cdir = self.content_dir.clone();
        // TODO: Don't capture the error.
        //handlebars_helper!(head: |p: String| crate::content::load_head(p).unwrap_or_else(Head{}));
        handlebars_helper!(upper: |s: String| s.to_uppercase());
        handlebars_helper!(lower: |s: String| s.to_lowercase());
        /*handlebars_helper!(pages: |_| {
            let contents = content::all_pages(self.content_dir.clone());
            contents.into::<PageValues>()
        });*/
        self.handlebars.register_helper("upper", Box::new(upper));
        self.handlebars.register_helper("lower", Box::new(lower));
        //self.handlebars
        //    .register_helper("head", Box::new(head));
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
/// It should be assumed that all data passed into this function will be visible to teh
/// end user.
pub fn error_values(title: &str, msg: &str) -> PageValues {
    PageValues {
        head: Head {
            title: title.to_string(),
            description: None,
            extra: None,
            template: None,
        },
        body: msg.to_string(),
    }
}

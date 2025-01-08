#[cfg(feature = "server")]
use {
    handlebars::Handlebars, handlebars_sprig, http::HeaderMap, std::collections::BTreeMap,
    std::path::PathBuf,
};

use std::{collections::HashMap, fs::File, io::Read};

use crate::rhai_engine::custom_rhai_engine_init;

use super::content::{Content, Head};
use anyhow::bail;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

/// The name of the default template.
/// This will be resolved to $TEMPLATE_DIR/$DEFAULT_TEMPLATE.hbs
#[cfg(feature = "server")]
const DEFAULT_TEMPLATE: &str = "main";

#[derive(Serialize, Deserialize, Clone)]
pub struct DynamicTemplateConfig {
    pub dynamic_content_url: String,
    pub dynamic_content_path: String,
    pub dynamic_template_name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DynamicTemplateParams {
    pub dynamic_content_path: String,
    pub dynamic_template_name: String,
}

/// Describe the site itself
#[cfg(feature = "server")]
#[derive(Serialize, Deserialize)]
pub struct SiteInfo {
    pub title: String,
    pub logo: Option<String>,
    pub base_url: Option<String>,
    pub about: Option<String>,
    pub theme: Option<String>,
    pub index_site_pages: Option<Vec<String>>,
    pub dynamic_templates: Option<Vec<DynamicTemplateConfig>>,
    #[serde(default)]
    pub prepend_route_info: bool,
    pub extra: BTreeMap<String, String>,
}

/// Context for a template render.
#[cfg(feature = "server")]
#[derive(Serialize)]
pub struct TemplateContext {
    request: BTreeMap<String, String>,
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

// #[derive(Serialize)]
// pub struct RequestValues {}

/// Information about the site, including site info and all of the pages.
#[cfg(feature = "server")]
#[derive(Serialize)]
pub struct SiteValues {
    info: SiteInfo,
    pages: BTreeMap<String, PageValues>,
}

/// The structured values sent to the template renderer.
/// The body should be legal HTML that can be inserted within the <body> tag.
#[derive(Serialize, Deserialize)]
pub struct PageValues {
    pub head: Head,
    pub body: String,
    pub published: bool,
}

impl From<Content> for PageValues {
    fn from(mut c: Content) -> Self {
        PageValues {
            body: c.render_markdown(&None),
            head: c.head,
            published: c.published,
        }
    }
}
#[cfg(feature = "server")]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct TemplateMeta {
    read_pages_glob: Option<Vec<String>>,
}

/// Renderer can execute a handlebars template and render the results into HTML.
#[cfg(feature = "server")]
pub struct Renderer<'a> {
    pub template_dir: PathBuf,
    pub template_meta: HashMap<String, TemplateMeta>,
    pub theme_dir: Option<PathBuf>,
    pub script_dir: PathBuf,
    pub content_dir: PathBuf,
    pub show_unpublished: bool,
    pub disable_cache: bool,
    handlebars: handlebars::Handlebars<'a>,
}

#[cfg(feature = "server")]
impl Renderer<'_> {
    /// Create a new renderer with the necessary directories attached.
    pub fn new(
        template_dir: PathBuf,
        theme_dir: Option<PathBuf>,
        script_dir: PathBuf,
        content_dir: PathBuf,
    ) -> Self {
        let mut handlebars = Handlebars::new();
        // Create custom rhai engine and assign to handlebars
        let rhai_engine = custom_rhai_engine_init();
        handlebars.set_engine(rhai_engine);
        let template_meta = HashMap::new();

        Renderer {
            template_dir,
            template_meta,
            theme_dir,
            script_dir,
            content_dir,
            show_unpublished: false,
            disable_cache: false,
            handlebars,
        }
    }
    // pub fn load(&mut self, name: &str, file: &str) -> Result<(), anyhow::Error> {
    //     let filepath = self.template_dir.join(file);
    //     self.handlebars.register_template_file(name, filepath)?;
    //     Ok(())
    // }

    /// Load the template directory.
    pub fn load_template_dir(&mut self) -> Result<(), anyhow::Error> {
        #[cfg(feature = "server")]
        self.register_helpers();

        // If there is a theme, load the templates provided by it first
        // Allows for user defined templates to take precedence
        if self.theme_dir.is_some() {
            let mut templates = self.theme_dir.as_ref().unwrap().to_owned();
            templates.push("templates");
            self.handlebars
                .register_templates_directory(".hbs", templates)?;
        }
        for entry in WalkDir::new(&self.template_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() && entry.path().extension().map_or(false, |e| e == "hbs")
            {
                let filename = entry.file_name().to_str().unwrap().replace(".hbs", "");
                let res = parse_hbs_template(entry);
                match res {
                    Ok((template_meta, body)) => {
                        self.template_meta
                            .insert(filename.to_owned(), template_meta);
                        self.handlebars.register_template_string(&filename, body)?;
                    }
                    Err(err) => {
                        eprintln!("Error reading template {}: {}", filename, err);
                    }
                }
            }
        }
        Ok(())
    }

    /// Load the scripts directory
    pub fn load_script_dir(&mut self) -> anyhow::Result<()> {
        let mut theme_scripts: Vec<PathBuf> = Vec::new();

        // If theme has scripts,load it first to follow proper precedence
        if self.theme_dir.is_some() {
            let mut theme_scripts_path = self.theme_dir.as_ref().unwrap().to_owned();
            theme_scripts_path.push("scripts");
            theme_scripts = crate::content::all_files(theme_scripts_path)?;
        }
        let user_scripts = crate::content::all_files(self.script_dir.clone())?;
        // TODO: rewrite all_files so we don't need to clone here.
        let scripts = [theme_scripts, user_scripts].concat();

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
        request: HeaderMap,
    ) -> anyhow::Result<String> {
        let page: PageValues = values.into();
        let tpl = page
            .head
            .template
            .clone()
            .unwrap_or_else(|| DEFAULT_TEMPLATE.to_owned());

        let mut request_headers: BTreeMap<String, String> = BTreeMap::new();
        for (key, value) in request.iter() {
            let val = value.to_str()?;
            request_headers.insert(String::from(key.as_str()), String::from(val));
        }

        let ctx = TemplateContext {
            page,
            request: request_headers,
            site: SiteValues {
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
                pages: match &info.index_site_pages {
                    Some(templates) => {
                        if templates.contains(&tpl) {
                            let mut glob_pattern: Option<&Vec<String>> = None;
                            let default_meta = TemplateMeta::default();
                            let template_meta =
                                self.template_meta.get(&tpl).unwrap_or(&default_meta);
                            if let Some(val) = &template_meta.read_pages_glob {
                                glob_pattern = Some(val);
                            }
                            self.create_site_pages_index(glob_pattern, &tpl)?
                        } else {
                            BTreeMap::new()
                        }
                    }
                    None => BTreeMap::new(),
                },
                info,
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
    #[cfg(feature = "server")]
    fn register_helpers(&mut self) {
        handlebars_sprig::addhelpers(&mut self.handlebars)
    }
    fn create_site_pages_index(
        &self,
        glob_pattern: Option<&Vec<String>>,
        tpl: &str,
    ) -> anyhow::Result<BTreeMap<String, PageValues>> {
        match glob_pattern {
            Some(pattern) => {
                let pages = crate::content::get_pages_by_glob(
                    self.content_dir.clone(),
                    pattern,
                    self.show_unpublished,
                );
                match pages {
                    Ok(val) => Ok(val),
                    Err(err) => {
                        bail!("Error parsing glob in template \"{tpl}\": {err}")
                    }
                }
            }
            None => crate::content::all_pages(
                self.content_dir.clone(),
                self.show_unpublished,
                self.disable_cache,
            ),
        }
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
            enable_shortcodes: None,
            path_info: None,
            body_source: None,
            cache_control: None,
        },
        body: msg.to_string(),
        published: true,
    }
}

fn read_file(file_path: &str) -> anyhow::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[cfg(feature = "server")]
fn parse_hbs_template(entry: walkdir::DirEntry) -> anyhow::Result<(TemplateMeta, String)> {
    let path = entry.path();
    match read_file(path.to_str().unwrap()) {
        Ok(contents) => {
            let doc = contents.replace("\r\n", "\n");
            let (toml_text, body) = doc.split_once("\n---\n").unwrap_or(("", &doc));
            let toml_text = toml_text.trim_start_matches("---").trim();
            let template_meta = toml::from_str(toml_text)?;
            Ok((template_meta, body.to_owned()))
        }
        Err(err) => {
            bail!("Failed to parse hbs template \"entry\": {err}")
        }
    }
}

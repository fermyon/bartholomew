title = "Bartholomew themes"
date = "2022-07-20T22:44:01.300319091Z"

[extra]

---

Bartholomew supports theming which allows for easy customization of the site along with the user-defined tempaltes.

## Adding a theme

Once the initial site has been set up using the [quickstart section](/quickstart), create a themes folder where you will be able to download different themes.
```bash
mkdir themes
```

Once the folder is created, different themes can be added as submodules to the folder which can then in turn be used to theme the site.

```bash
cd themes
git submodule add <Source_to_the_theme>
```

Multiple themes can be added to the themes directory but only one of them will be active at a given time as described in the next section.

## Configuring the site to use the theme

To choose a theme for the website, the `theme` attribute in `config/site.toml` must be configured, where the value is the name of the theme as found in the `themes/` folder.

```toml
title = "Bartholomew Documentation"
base_url = "http://localhost:3000"
about = "The Micro-CMS for WebAssembly and Spin"
theme = "<theme-directory>"

[extra]
copyright = "Fermyon"
github = "https://github.com/fermyon/bartholomew"
twitter = "https://twitter.com/fermyontech"
ga_measurement_id = ""

date_style = "%B %e, %Y"
```
One more step that needs to be done before themes are fully available to the site is to the change the static file server component in the `spin.toml` configuration so that it provides the static assets of the selected theme. The convention of mounting the static assets of the themes before the user-defined static assets is recommened.  

```
.
.
.

[[component]]
source = "modules/spin_static_fs.wasm"
id = "fileserver"
files = [ {source = "themes/<name of theme>/static", destination ="/"}, { source = "static/", destination = "/" }, ]
[component.trigger]
route = "/static/..."

```


## Template precedence

When a theme is enabled for a site, both the user-defined and theme-provided assets like the templates, scripts and static assets will be available. If a theme-provided asset and user-provided asset have the same name, the user-defined asset takes precedence. This allows for the user to override the theme to allow for customization. 

As an example, if both `templates/main.hbs` and `themes/<name of theme>/templates/main.hbs` exist, the user-defined `templates/main.hbs` takes precedence leading to the rendering engine using the user-defined template overriding the theme.

## Creating a Theme

Creating a theme for Bartholomew is easy. Create a new folder and initialize it.

```
mkdir custom_theme
cd custom_theme
git init
```

Once the git repository is initialized, create the three required directories.
```
mkdir templates scripts static
```

Create the custom theme by placing the handlebar templates in the `template/` folder while the Rhai scripts are placed in the `scripts/` folder. All the static assets such as  the images, JS and CSS are placed in the static folder. For reference on creating templates, refer to the [templates section](/templates).

Once the required changes are done, commit and push the changes to a remote repository, so as to allow for the theme to cloned as a submodule that can be used for theming a site. 


Let's take a look at how you can do something special in your templates [using scripting](./scripting.md).

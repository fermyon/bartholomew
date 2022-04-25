title = "Configuration for Bartholomew"
template = "main"
date = "2022-04-24T21:51:00Z"
[extra]
url = "https://github.com/fermyon/spin/blob/main/docs/content/configuration.md"
---

Bartholomew can run inside of any CGI environment that supports directly executing Wasm modules. That basically means [Hippo](https://github.com/deislabs/hippo), [Wagi](https://github.com/deislabs/wagi), and [Wagi.net](https://github.com/deislabs/wagi-dotnet). (If you get it running in another environment, please tell us!)

Bartholomew requires that several directories are mounted into the Wasm module:

* `templates/` should contain Handlebars templates.
* `content/` should contain Markdown files.
* `scripts/` contains Rhai scripts that are available as template helpers.
* `config/site.toml` is the main site configuration file

By convention, we suggest putting all of your Wasm modules in a directory called `modules/`. However, there is no hard-coded reason why you need to do this.

A `modules.toml` might look something like this:

```toml
[[module]]
module = "target/wasm32-wasi/release/bartholomew.wasm"
route = "/..."
volumes = { "content/" = "content/" , "templates/" = "templates/", "scripts/" = "scripts/", "config/" = "config/"}
```

At the time of this writing, Bartholomew does not serve static files. Instead, use the [fileserver](https://github.com/deislabs/wagi-fileserver) for Wagi:

```toml
[[module]]
module = "modules/fileserver.gr.wasm"
route = "/static/..."
volumes = { "/" = "static/"}
```

Using the fileserver has the distinct security advantage that downloadable files are stored in isolation from executed files like templates.
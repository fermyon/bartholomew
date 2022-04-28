title = "Introducing Bartholomew"
template = "main"
date = "2022-04-24T21:51:00Z"
[extra]
url = "https://github.com/fermyon/bartholomew/blob/main/docs/content/index.md"
---

Bartholomew is an open-source micro-CMS for hosting lightweight websites with the power of [WebAssembly](https://webassembly.org/). Built into a lightweight WebAssembly module, it processes page requests individually on demand.

Bartholomew has the following features:

* Write your content in Markdown with a simple TOML header.
* Create custom templates using the popular Handlebars template language.
* Build custom template functions in the Rhai scripting language.
* Generate your first Bartholomew site from our easy site template
* Deliver static files with the Wagi fileserver.
* Get instant SEO optimization with dynamically generated (and easily customizable) Atom syndication, sitemap.xml, and robots.txt files.

### Overview

With all the buzz around cloud-native WebAssembly, the team at Fermyon was influenced to build a project that leveraged some of the cooler bits of Wasm. We ended up settling on the decision to build a CMS, which ended up turning into Bartholomew. The day we launched Fermyon, we decided that our first project would be to host our own website on our new WebAssembly stack. That gave us a feature set to target for Bartholomew’s first release.

The result: An adaptable WebAssembly CMS that uses a Functions-as-a-Service model for page rendering instead of requiring the entire site to be pre-generated to static HTML.

In the next section, we will [build our first site](/quickstart) with Bartholomew.

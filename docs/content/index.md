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

Bartholomew is built as a [Wagi](https://github.com/deislabs/wagi) application and will run inside of [Hippo](https://github.com/deislabs/hippo) or inside of [Krustlet](https://krustlet.dev/) if you have the Wagi extension. At Fermyon, we run ours using Hippo deploying to a [Nomad](https://www.nomadproject.io/) cluster.

Bartholomew offers a feature set that should be familiar to users of popular static site generators like Hugo. However, Bartholomew is not a static site generator. 

In the next section, we will [build our first site](/quickstart) with Bartholomew.

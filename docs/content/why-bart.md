title = "Why Bartholomew?"
template = "main"
date = "2022-04-27T00:22:56Z"
[extra]
url = "https://github.com/fermyon/bartholomew/blob/main/docs/content/why-bart.md"
---

The ecosystem for tooling that supports building "static" sites such as [Hugo](https://gohugo.io/) and other [Jamstack](https://jamstack.org/) platforms is turning into a packed space. While Bartholomew may share a lot of similarities with these tools, it is not actually a static site generator. Bartholomew websites are built into lightweight WebAssembly modules that _process page requests individually on demand_.

By leveraging this approach, Bartholomew is able to leverage the following features:

- Write your content in Markdown with a simple `toml` header.
- Create custom templates using the popular Handlebars template language.
- Build custom template functions in the Rhai scripting language.
- Get started quickly with built-in support for Bootstrap 5.
- Generate your first Bartholomew site from our easy Bartholomew site template.
- Deliver static files with the Wagi fileserver.
- Get instant SEO optimization with dynamically generated (and easily customizable) Atom syndication, sitemap.xml, and robots.txt files.

With all the buzz around cloud-native WebAssembly, the team at Fermyon was influenced to build a project that leveraged some of the cooler bits of Wasm. We ended up settling on the decision to build a CMS, which ended up turning into Bartholomew. The day we launched Fermyon, we decided that our first project would be to host our own website on our new WebAssembly stack. That gave us a feature set to target for Bartholomew’s first release.

The result: An adaptable WebAssembly CMS that uses a Functions-as-a-Service model for page rendering instead of requiring the entire site to be pre-generated to static HTML.
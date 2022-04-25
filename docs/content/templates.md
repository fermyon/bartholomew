title = "Templating Bartholomew Websites"
template = "main"
date = "2022-04-24T21:51:00Z"
[extra]
url = "https://github.com/fermyon/bartholomew/blob/main/docs/content/templates.md"
---

Bartholomew uses a [version of the Handlebars template language](https://crates.io/crates/handlebars). The template syntax is describe in the [Handlebars documentation](https://handlebarsjs.com/).

Every file in the `templates/` directory will be compiled to a template. The file is then accessible by its relative name, minus the extension. For example. `templates/main.hbs` will be accessible as main.

Note that Bartholomew expects to find a template named `main`. This template is used as a default when the content head does not contain a `template` directive. It is also used when an error occurs. You must have a main template.

## Accessing The Head (Front Matter) and the Body

The head is available in the template using the `{{ page.head }}` object. For example, to print the title, use `{{ page.head.title }}`. To access your custom [extra] field named foo, use `{{ page.head.extra.foo }}`.

The body is injected to the template converted to HTML. That is, the template does not have access to the Markdown version of the document.

To print the HTML body without having the output escaped, use `{{{ page.body }}}` (note the triple curly braces).
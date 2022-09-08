title = "Markdown guide"
date = "2022-05-08T14:05:02.118466Z"

[extra]
url = "https://github.com/fermyon/bartholomew/blob/main/docs/content/markdown.md"
---

To write content for a Bartholomew website, there are two steps:

1. Create the new page using the `bart new post` command,
2. Write the body.

## Creating a New File

To create a new page, run the `bart new post` command, with the path to where
the file should be created as the first argument (it should be in the root or in
a subdirectory of `content/`) and the file name as the second argument, making
sure the file has the `.md` extension.
For example, if the new post was created in `content/foo.md`, this file will be
rendered by Bartholomew as `<base_url>/foo`.

> The content directory can have subdirectories.

Bartholomew supports Markdown via the [Pulldown-Cmark](https://crates.io/crates/pulldown_cmark) library.

## Editing the Page Head

The first part of any Bartholomew document is the _head_. You can think of every piece of content as having a _head_ and a _body_. (Shoulders, knees, and toes will be added in a forthcoming release.)

The head is formatted as TOML, which for the most part is just names and values.

Here is an example head for a blog post:

```markdown
title = "A New Article"
description = "This article is really interesting and full of useful material."
date = "2021-12-23T15:05:19Z"
template = "post"
tags = ["news", "article"]
enable_shortcodes = false

[extra]
author = "Matt Butcher"
author_page = "/author/butcher"
---
```

Here is an example head for a web page:

```markdown
title = "The title"
description = "A short description"
date = "2021-12-23T23:20:57Z"
template = "main" # The default is `main`, which correlates to `templates/main.hbs`

[extra]
key = "your custom name value pairs go hear, but values MUST be strings"
---
```

### Markdown Body

Markdown support includes all the usual stuff plus fenced codeblocks. Image links are
supported, but you need to use the external [fileserver](https://github.com/fermyon/spin-fileserver)
library to display the images. If you are using your deployment of Spin and Bartholomew, you can [read more about how to install the fileserver](https://bartholomew.fermyon.dev/contributing) from source.

The last line of the example above is very important. The `---` tells Bartholomew that the head is done, and the body is coming up.

Every head must have a `title`. It is _strongly_ recommended that it also have a `date` as well because dates are tied to some of Bartholomew's features. There are several other defined fields. When you want to add your own fields, put them after the `[extra]` marker.
You can add your own fields as name/value pairs. Just make sure you quote these string values.

The following fields are defined for Bartholomew:

- `title`: The name of the piece of content. This is REQUIRED.
- `date`: The date that this post is published. This can be a future date (in which case Bartholomew will not publish it until that date). It MUST be in ISO 8601 format. It is STRONGLY RECOMMENDED.
- `description`: A short description of the content. This should be no more than a few sentences. It is RECOMMENDED.
- `template`: The name of the template that should be used to render this content. It is OPTIONAL and defaults to `main` (`templates/main.hbs`).
- `published`: A boolean (`published = true` or `published = false`, no quotes) that explicitly sets publication status. It is OPTIONAL and should be used sparingly.
- `tags`: A list of tags. The tags should be ranked most- to least relevant. OPTIONAL and defaults to an empty list.
- `content_type`: A media type for the document. For example, if you are generating XML, use `text/xml`. The default is HTML. OPTIONAL and should rarely be used.
    - NOTE: `content_type` has no impact on the formatting. That is, Markdown will still be rendered to HTML.
- `status`: (EXPERT) Status code and message for HTTP. E.g. "302 Found"
- `redirect`: (EXPERT) Send a redirect to the given fully qualified URL. The default redirect type is `301 Moved Permanently`. Use `status` to set another redirect type. When this is set, no body is sent to the client.
- `enable_shortcodes`: Allows the addition of shortcodes in the markdown content using rhai scripts. Defaults to false.
- `[extra]`: The section marker indicates that all content after it is user-defined.
    - Fields under extra MUST be in the form `name = "value"`. All values must be strings.
    - once the `[extra]` section is declared, you cannot declare top-level fields anymore.

After the `---`, you can write the content's body in Markdown.

## Writing a Markdown Body

**Please note:** When writing a blog post with more than one paragraph, the `<!-- break -->` syntax must be placed between the end of the first paragraph and the beginning of the second paragraph. The paragraph content above the `<!-- break -->` will be shown in the HTML as having a slightly larger font size (relative to the rest of the page's content). The first paragraph will also appear when viewing the list of all blog posts (from the blog index page). Here is an example:

```markdown
This is my first paragraph which introduces my blog post.

<!-- break -->

This is the second paragraph, note the line breaks used in this code block example.

This is the third paragraph, it is treated exactly like the second paragraph.
```

As you would have already seen, all documents in Bartholomew are written in Markdown.

[Markdown](https://www.markdownguide.org/) is a simple text format designed to make it easy (and very fast) to write the text that can then be converted into HTML.

For example, the Markdown `*hello*` is transformed into *hello*. And the Markdown `[example](http://example.com)` is transformed into the link [example](http://example.com).

To make a header, you just use hash marks: `#` for a title, `##` for a sub-header, and so on. Bullet lists are just plain text hyphen or asterisk lists.

For example:

```markdown
- This
- Is
- A
- List
```

The above produces:

- This
- Is
- A
- List

Make text italic by wrapping it in underscores: `_hello_` becomes _hello_. And use double asterisks for bold. `**goodbye**` becomes **goodbye**.

There are other Markdown goodies, but one you should know is how to make a link. Links are built by putting text in square brackets, and the URL in parentheses.

```markdown
Say [Hello](http://example.com)
```

Say [Hello](http://example.com).

If you want to reference an image, just add `!` in front of the square bracket. In that situation, the text in the square brackets becomes the image description.

## Why Is Markdown the Only Supported Format?

Some static site generators allow you to use other formats like asciidoc. Bartholomew only supports Markdown. The reason for this is pragmatic: We are trying to keep the binary size as small as we can to make Bartholomew faster.

## Can I Embed HTML?

Yes, you can embed HTML tags inside of your Markdown.

Bartholomew supports reusable content using snippets in the form of shortcodes, so let's take a look at the [shortcodes](/shortcodes) support in Bartholomew.

Lastly, let's take a look at how templating is used in Bartholomew.

## Templates

Bartholomew uses a [version of the Handlebars template language](https://crates.io/crates/handlebars).
The template syntax is described in the [Handlebars documentation](https://handlebarsjs.com/).

Every file in the `templates/` directory will be compiled into a template. The file is then accessible by its relative name, minus the extension. For example. `templates/main.hbs` will be accessible as `main`.

Note that Bartholomew _expects_ to find a template named `main`. This template is used as a default when the content head does not contain a `template` directive. It is also used when an error occurs. You must have a `main` template.

### Accessing The Head (Front Matter) and the Body

The head is available in the template using the `{{ page.head }}` object. For example, to print the title, use `{{ page.head.title }}`. To access your custom `[extra]` field named `foo`, use `{{ page.head.extra.foo }}`. The body is injected into the template and converted to HTML. That is, the template does not have access to the Markdown version of the document.

To print the HTML body without having the output escaped, use `{{{ page.body }}}` (note the triple curly braces).
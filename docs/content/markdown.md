title = "Markdown guide"
date = "2022-05-08T14:05:02.118466Z"

[extra]
url = "https://github.com/fermyon/bartholomew/blob/main/docs/content/markdown.md"
---

To write content for a Bartholomew website, there are two steps:

1. Create the new page using the `bart new post` command
2. Write the body

## Creating a new file

To create a new page, run the `bart new post` command, with the path to where
the file should be created as the first argument (it should be in the root or in
a subdirectory of `content/`) and the file name as the second argument, making
sure the file has the `.md` extension.
For example, if the new post was created in `content/foo.md`, this file will be
rendered by Bartholomew as `<base_url>/foo`.

> The content directory can have subdirectories.

## Editing the page head

The first part of any Bartholomew document is the _head_. This is formatted as TOML, which for the most part is just names and values.

Here is an example head:

```
title = "A New Article"
description = "This article is really interesting and full of useful material."
date = "2021-12-23T15:05:19Z"
template = "post"
tags = ["news", "article"]

[extra]
author = "Matt Butcher"
author_page = "/author/butcher"
---
```

The last line of the example above is very important. The `---` tells Bartholomew that the head is done, and the body is coming up.

Every head must have a `title`. It is _strongly_ recommended that it also have a `date` as well, because dates are tied to some of Bartholomew's features.
There are several other defined fields. When you want to add your own fields, put them after the `[extra]` marker.
You can add your own fields as name/value pairs. Just make sure you quote the values.

The following fields are defined for Bartholomew:

- `title`: The name of the piece of content. This is REQUIRED.
- `date`: The date that this post is published. This can be a future date (in which case Bartholomew will not publish it until that date). It MUST be in ISO 8601 format. It is STRONGLY RECOMMENDED.
- `description`: A short description of the content. This should be no more than a few sentences. It is RECOMMENDED.
- `template`: The name of the template that should be used to render this content. It is OPTIONAL and defaults to `main` (`templates/main.hbs`).
- `published`: A boolean (`published = true` or `published = false`, no quotes) that explicitly sets publication status. It is OPTIONAL and should be used sparingly.
- `tags`: A list of tags. The tags should be ranked most- to least-relevant. OPTIONAL and defaults to an empty list.
- `content_type`: A media type for the document. For example, if you are generating XML, use `text/xml`. The default is HTML. OPTIONAL and should rarely be used.
    - NOTE: `content_type` has no impact on the formatting. That is, Markdown will still be rendered to HTML.
- `status`: (EXPERT) Status code and message for HTTP. E.g. "302 Found"
- `redirect`: (EXPERT) Send a redirect to the given fully qualified URL. Default redirect type is `301 Moved Permanently`. Use `status` to set another redirect type. When this is set, no body is sent to the client.
- `[extra]`: The section marker that indicates that all content after it is user-defined.
    - Fields under extra MUST be in the form `name = "value"`. All values must be strings.
    - once the `[extra]` section is declared, you cannot declare top-level fields anymore.

After the `---`, you can write the content's body in Markdown.

## Writing a markdown body

All documents in Bartholomew are written in Markdown.
[Markdown](https://www.markdownguide.org/) is a simple text format designed to make it easy (and very fast)
to write text that can then be converted into HTML.

For example, the Markdown `*hello*` is transformed into *hello*. And the Markdown
`[example](http://example.com)` is transformed into the link [example](http://example.com).

To make a header, you just use hash marks: `#` for a title, `##` for a sub-header, and so on.
Bullet lists are just plain text hyphen or asterisk lists.

For example:

```
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

Make text italic by wrapping it in underscores: `_hello_` becomes _hello_. And use
double asterisks for bold. `**goodbye**` becomes **goodbye**.

There are other Markdown goodies, but one you should know is how to make a link.
Links are built by putting text in square brackets, and the URL in parentheses.

```
Say [Hello](http://example.com)
```

Say [Hello](http://example.com).

If you want to reference an image, just add `!` in front of the square bracket. In that
situation, the text in the square brackets becomes the image description.

## Why is markdown the only supported format?

Some static site generators allow you to use other formats like asciidoc.
Bartholomew only supports Markdown.
The reason for this is pragmatic: We are trying to keep the binary size as small as we can to make Bartholomew faster.

## Can I embed HTML?

Yes, you can embed HTML tags inside of your Markdown.

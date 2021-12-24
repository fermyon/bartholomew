title = "Writing Content in Markdown"
description = "Write your content in a simple text language"
date = "2021-12-23T17:05:19Z"
---

To create a new document, just drop a file into the `content/` folder, and make sure
that file has the extension `.md`. For example, `/content/foo.md` is a new document.
That document is then available to Bartholomew as `/foo`. So when a user types
`http://example.com/foo`, Bartholomew will load the file `/content/foo.md`.

> The content directory can have subdirectories.

All documents in Bartholomew are written in Markdown.
[Markdown]() is a simple text format designed to make it easy (and very fast)
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
asterisks for bold. `*goodbye*` becomes *goodbye*.

There are other Markdown goodies, but one you should know is how to make a link.
Links are built by putting text in square brackets, and the URL in parentheses.

```
Say [Hello](http://example.com)
```

Say [Hello](http://example.com).

If you want to reference an image, just add `!` in front of the square bracket. In that
situation, the text in the square brackets becomes the image description.
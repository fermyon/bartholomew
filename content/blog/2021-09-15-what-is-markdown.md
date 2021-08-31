title = "What Is Markdown?"
description = "Introducing an easy-to-write document format"
---
Markdown is one of the most prevalent document formats for writing content. If you use
Slack, you probably have written some Markdown. Every Slack message is a mini
Markdown document. GitHub, HackMD, and many other similar tools use Markdown as their
primary format for composing text. Why? Because it is very easy.

For starters, paragraphs are just plain-text paragraphs. No tags or markup need. To make
a header, you just use hash marks: `#` for a title, `##` for a sub-header, and so on.
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
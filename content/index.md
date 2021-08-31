title = "Welcome to Bartholomew"
description = "The Micro-CMS for Wagi"

[extra]
date = "Aug. 30, 2021"
author = "Matt Butcher"
author_page = "/author/butcher"
---
It might be hard to spell, but fortunately you won't have to spell it that often.

Bartholomew is a Micro-CMS for hosting Markdown content on a WebAssembly server.
It can run on any [Wagi-compliant runtime](https://github.com/deislabs/wagi), including
Wagi, Hippo, and Wagi.Net.

## Content is in Markdown

Write your content in Markdown, the simple text-based markup language.
Just drop your content somewhere in the `content/` folder, and you're ready to go.

## Templates are Handlebars

Handlebars is a popular template format similar to Mustache. All of the templates
are fully customizable. You can take a look in the `templates/` directory to start
customizing the look and feel of this site.

## Configuration Files

Configuration files are in `config`, and are simple TOML files. Probably the only one
you need is `config/site.toml`.

## Getting Started

To get started, edit this page. While Bartholomew doesn't make a lot of assumptions about
your site, it _does_ assume that requests to the root (`/`) should be directed to
`index.md`.
title = "Bartholomew configuration"
date = "2022-05-09T19:32:09.579913Z"

[extra]
url = "https://github.com/fermyon/bartholomew/blob/main/docs/content/configuration.md"
---

[TOML](https://toml.io/en/) is a simple configuration format.
Bartholomew uses TOML for [the `head` in Markdown documents](markdown) as well as
in the site configuration. In this chapter, we will focus on site configuration.

Your site's `config/` directory has one configuration file in it, called `site.toml`:

```
title = "Bartholomew"
# logo = "URL to logo"
base_url = "http://localhost:3000"
about = "This site is generated with Bartholomew, the Spin micro-CMS. And this message is in site.toml."
theme = "fermyon"
index_site_pages = ["main"]

[extra]
copyright = "The Site Authors"
github = "https://github.com/technosophos/bartholomew"
twitter = "https://twitter.com/technosophos"
```

You can think of this as "header for your site".

It has a few pre-defined fields:

- title: the title of your website
- logo: a URL or static path to your logo
- base_url: a base URL that templates can use to construct full URLs to content. This can be overridden by setting the `-e BASE_URL="https://example.com"` environment variable for Spin.
- about: a brief description of the site
- theme: the name of the theme for the website from the `/themes/` folder
- index_site_pages: A list of templates that require `site.pages` to be populated.

You can define your own fields in the `[extra]` section. Anything in `[extra]` is not
used by the system itself. But it's a useful way to pass information from one central
place to all of your templates. For example, a template can access the `copyright` value
using `{{site.info.extra.copyright}}`.

Let's take a look at how you can configure your site to use [themes](./themes.md).

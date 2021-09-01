title = "Editing the Configuration File"
description = "You can change site-wide settings in config.toml"
---
Your site's `config/` directory has one configuration file in it, called `site.toml`:

```
title = "Bartholomew"
# logo = "URL to logo"
base_url = "http://localhost:3000"
about = "This site is generated with Bartholomew, the Wagi micro-CMS. And this message is in site.toml."

[extra]
copyright = "The Site Authors"
github = "https://github.com/technosophos/bartholomew"
twitter = "https://twitter.com/technosophos"
```

You can think of this as "frontmatter for your site".

It has a few pre-defined fields:

- title: the title of your website
- logo: a URL or static path to your logo
- base_url: a base URL that templates can use to construct full URLs to content
- about: a brief description of the site

You can define your own fields in the `[extra]` section. Anything in `[extra]` is not
used by the system itself. But it's a useful way to pass information from one central
place to all of your templates. For example, a template can access the `copyright` value
using `{{site.info.extra.copyright}}`.
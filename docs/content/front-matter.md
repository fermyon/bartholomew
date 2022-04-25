title = "Bartholomew Headers"
template = "main"
date = "2022-04-24T21:51:00Z"
[extra]
url = "https://github.com/fermyon/bartholomew/blob/main/docs/content/front-matter.md"
---

Front matter is fairly basic, limited to a few predefined entries, and a map of custom name/value pairs. In Bartholomew, we refer to front matter as head (the header) for brevity. So you can think of every piece of content as having a head and a body. (Shoulders, knees, and toes will be added in a forthcoming release.)

A typical `head` looks like this:

```toml
title = "The title"
description = "A short description"
date = "2021-12-23T23:20:57Z"
template = "main" # The default is `main`, which correlates to `templates/main.hbs`

[extra]
key = "your custom name value pairs go hear, but values MUST be strings"
```

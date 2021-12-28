title = "The Goals of Bartholomew"
description = "We have plans. Big plans. Actually, they're small plans."
date = "2021-12-23T17:05:19Z"
[extra]
author = "Matt Butcher"
author_page = "/author/butcher"
---

Bartholomew is intended to be three things:

1. A simple-to-use CMS-like system that feels like Jekyll or other static site generators
2. A flexible system that is cheap to run, but easy to extend. (Cloud is expensive!)
3. An exhibition of the value of WebAssembly on the server-side

Static site generators are great for many things. But they are also limiting. To keep
a site feeling fresh, you have to keep building and publishing the site because all
elements are rendered at build time. If you want to write dynamic elements of any sort,
you have to resort to JavaScript.

But at the same time, these static site generators are super easy to use. Content is
just written in plain text, and is rendered into HTML. Menus and navigation are
built up automatically. And dealing with images and other files is as easy as dropping
a file in a directory. We love that experience. So we tried to combine the best of the
static site generator with a server-side technology.

Bartholomew works like PHP (the language Wordpress and Drupal are written in). Each
time the server receives a request, it starts up a new Bartholomew instance, which only
runs long enough to serve the request. In this model, you don't need to keep a
Bartholomew server running all the time.

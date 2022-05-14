title = "Introducing Bartholomew"
template = "main"
date = "2022-05-07T00:22:56Z"
[extra]
url = "https://github.com/fermyon/bartholomew/blob/main/docs/content/index.md"
---

Bartholomew is a simple CMS-like (Content Management System) tool for managing a
website. It is compiled to WebAssembly, and can run in any [Spin](https://spin.fermyon.dev)
environment.

At a glance, with Bartholomew you can:

- Write your content in [Markdown](https://www.markdownguide.org/), with a
simple [TOML](https://toml.io/en/) header.
- Create custom page templates using the popular [Handlebars](https://handlebarsjs.com/)
templates.
- Build custom functions using [the Rhai scripting language](https://rhai.rs/).
- Serve files using [the Spin static file server](https://github.com/fermyon/spin-fileserver).

## Overview

Bartholomew is built using as Functions-as-a-Service (FaaS) model, similar to
one you might find in AWS Lambda or Azure Functions. The CMS is only running when
it needs to handle incoming requests, reducing the load on the servers
running it.

Bartholomew is a [Spin](https://spin.fermyon.dev) component, and
websites built with Bartholomew are Spin applications that can run in any
environment that is capable of running Spin. At Fermyon, we run all of our
websites using Bartholomew and Spin, on our [Fermyon Platform, running on Nomad](https://www.fermyon.com/blog/spin-nomad).

Getting started with your new WebAssembly-powered website is as simple as:

```bash
# create a new site based on a template repository
$ bart new site --git https://github.com/fermyon/bartholomew-site-template website
# run the website using Spin
$ spin up --file website/spin.toml
Serving HTTP on address http://127.0.0.1:3000
Available Routes:
  bartholomew: http://127.0.0.1:3000 (wildcard)
  fileserver: http://127.0.0.1:3000/static (wildcard)
```

Now you can access your new website! Add your Markdown documents in the `content/`
directory, and Bartholomew will dynamically render the website pages for you on
each new request.

In the next page, we will [take Bartholomew for a spin](./quickstart.md).

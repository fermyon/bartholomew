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

Bartholomew is built using a Functions-as-a-Service (FaaS) model, similar to
one you might find in AWS Lambda or Azure Functions. The CMS is only running when
it needs to handle incoming requests, reducing the load on the servers
running it.

Bartholomew is a [Spin](https://spin.fermyon.dev) component, and
websites built with Bartholomew are Spin applications that can run in any
environment that is capable of running Spin. At Fermyon, we run all of our
websites using Bartholomew and Spin, on our [Fermyon Platform, running on Nomad](https://www.fermyon.com/blog/spin-nomad).

In the next page, we will [take Bartholomew for a spin](./quickstart.md).

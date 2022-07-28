# Bartholomew MicroCMS

Bartholomew is a simple CMS-like tool for hosting a website. It is compiled entirely
to WebAssembly, and can run in any Spin-capable system.

The screenshot below shows how Bartholomew is implemented as the official Fermyon website.

![Bartholomew screenshot](docs/static/image/bartholomew-screenshot.png)

## Bartholomew documentation

Check out [the docs](https://bartholomew.fermyon.dev/) and please raise any issues or ask any questions; we are here to help :)

## Getting Bartholomew

The easiest way to start is to get the [Bartholomew site template](https://github.com/fermyon/bartholomew-site-template) and then download the [latest release of Bartholomew](https://github.com/fermyon/bartholomew/releases).

### Building the Bartholomew Server from Source

Create (and change into) a new directory where you would like to install Bartholomew.

```bash
mkdir /home/my_bartholomew_installation
cd /home/my_bartholomew_installation
```

To build the Bartholomew Server from source, clone the source code from [the repository](https://github.com/fermyon/bartholomew), change into the newly created directory and build it.

```bash
cd /home/my_bartholomew_installation/bartholomew
make build
```

The `make build` command basically does a `cargo build --target wasm32-wasi --release`.

> `make build` also runs `wasm-opt`, which is part of the [Binaryen](https://webassembly.github.io/binaryen/) project.
> This reduces the size of the WebAssembly module by optimizing the bytecode.

Once you have used the aforementioned `make build` command you will be able to view some newly created files in your `/home/my_bartholomew_installation/bartholomew/target/wasm32-wasi/release` directory.

```
ls /home/my_bartholomew_installation/bartholomew/target/wasm32-wasi/release
bartholomew.d       bartholomew.wasm    build           deps            examples        incremental     libbartholomew.d    libbartholomew.rlib
```

### Building the Bartholomew Command Line Interface (CLI) tool

The `bart` CLI is something that you could/would use a lot i.e. it can be used to create a new blog post using a single line command.

To build the Bartholomew CLI from source, perform the following commands

```bash
cd /home/my_bartholomew_installation/bartholomew
make bart
```

Once built, you will find the very useful `bart` CLI executable in the `/home/my_bartholomew_installation/bartholomew/target/release` directory. 

For more information about how to use the CLI, please type `/home/my_bartholomew_installation/bartholomew/target/release --help`, as shown below.

```bash
/home/my_bartholomew_installation/bartholomew/target/release/bart --help    
bart 0.1.0
The Bartholomew CLI

USAGE:
    bart <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    calendar    Print the content calendar for a Bartholomew website
    check       Check content and identify errors or warnings
    help        Prints this message or the help of the given subcommand(s)
    new         Create a new page or website from a template
```

The Bartholomew CLI also has some other great features i.e. the `bart` command can automatically check content (i.e. parse your web page and/or blog post markdown files) and identify & report any issues. See example of test output below

```
✅ content/about.md
✅ content/atom.md
✅ content/code-of-conduct.md
✅ content/index.md
✅ content/open-source-promise.md
✅ content/platform.md
✅ content/privacy-policy.md
✅ content/robots.md
✅ content/sitemap.md
✅ content/tag.md
✅ content/thanks.md
✅ content/values.md
```

Please note that when you use the `bart` command to check content you must use the following procedure.

First, go to your actual CMS's location on the file system and change into the directory directly above where the `.md` content (to be analysed) is housed. Once at this location then call the `bart` command by explicitly mentioning the directory which houses the `.md` files. Here are two examples.

### Check web pages

```bash
# Note we are inside the actual CMS where we create our content for our site which we publish
cd /home/my_new_cms
# Note the "bart check" command is being run, as an absolute path, from where we installed bartholomew from source (notice how we are passing in the "content/*" as a parameter to bart's "check" subcommand).
/home/my_bartholomew_installation/bartholomew/target/release/bart check content/*
```

### Check blog posts

```bash
# Note that we are still inside the CMS, just one level deeper to that we can now check the blog md files instead
cd /home/my_new_cms/content
# Note the "bart check" command is still being run, as an absolute path, from where we installed bartholomew from source (notice how we are passing in the "blog/*" as a parameter to bart's "check" subcommand this time around).
/home/my_bartholomew_installation/bartholomew/target/release/bart check blog/*
```


## The relationship between Bartholomew and Spin

To run Bartholomew, you will need a Spin-capable runtime. For example, you can just download a recent release of [Spin](https://github.com/fermyon/spin) and put it on your `$PATH`.

Then the `make serve` command can start it all up for you.

### The relationship between Bartholomew and the Spin Fileserver

Bartholomew uses an external file server called [Spin-Fileserver](https://github.com/fermyon/spin-fileserver).

Build the WebAssembly module and copy it in the `modules/` directory. When you are done, you should see:

```console
$ ls modules                                 
README.md          spin_static_fs.wasm
```

## Running Bartholomew

With Spin:

```
$ spin up
```

With `make`:

```
$ make serve
```

For convenience, `make serve` builds the code, and then runs `spin up`.

### Preview Mode

By default, Bartholomew will not display content that is unpublished.
Content is unpublished if either:

- The article is marked `published = false` in its head
- The article has a publish date in the future

To view unpublished content, turn on `PREVIEW_MODE`.

Spin:

```
$ spin up -e PREVIEW_MODE=1
```

Make:

```
$ PREVIEW_MODE=1 make serve
```

## Configuring Bartholomew

Bartholomew can run inside of any Spin environment that supports directly executing
Wasm modules. That basically means Hippo and Spin. (If you get it running
in another environment, please tell us!)

Bartholomew requires that several directories are mounted into the Wasm module:

- `templates/` should contain Handlebars templates.
- `content/` should contain Markdown files.
- `scripts/` contains Rhai scripts that are available as template helpers.
- `config/site.toml` is the main site configuration file

By convention, we suggest putting all of your Wasm modules in a directory called `modules/`.
However, there is no hard-coded reason why you need to do this.

A `modules.toml` might look something like this:

```toml
[[module]]
module = "target/wasm32-wasi/release/bartholomew.wasm"
route = "/..."
volumes = { "content/" = "content/" , "templates/" = "templates/", "scripts/" = "scripts/", "config/" = "config/"}
```

At the time of this writing, Bartholomew does not serve static files. Instead, use
the [fileserver](https://github.com/fermyon/spin-fileserver) for Spin:

```
[[module]]
module = "modules/fileserver.gr.wasm"
route = "/static/..."
volumes = { "/" = "static/"}
```

Using the fileserver has the distinct security advantage that downloadable files are stored
in isolation from executed files like templates.

## Creating Content for Bartholomew

Bartholomew content consists of Markdown documents with TOML headers (aka _Front Matter_):

```
title = "This is the title"
description = "A quick description of the article"
date = "2021-12-23T23:20:57Z"

[extra]
info = "The [extra] section is for your own custom metadata fields. You can use them in templates."

# The three dashes mark the end of the header and the beginning of the content
---

# Hello

This is the markdown content

```

Bartholomew supports Markdown via the [Pulldown-Cmark](https://crates.io/crates/pulldown_cmark)
library.

### Head (Front Matter)

Front matter is fairly basic, limited to a few predefined entries, and a map of custom
name/value pairs. In Bartholomew, we refer to front matter as `head` (_the header_) for brevity.
So you can think of every piece of content as having a _head_ and a _body_.
(Shoulders, knees, and toes will be added in a forthcoming release.)

A typical `head` looks like this:

```
title = "The title"
description = "A short description"
date = "2021-12-23T23:20:57Z"
template = "main" # The default is `main`, which correlates to `templates/main.hbs`

[extra]
key = "your custom name value pairs go hear, but values MUST be strings"

```

### Markdown Body

Markdown support includes all the usual stuff plus fenced codeblocks. Image links are
supported, but you need to use the external [fileserver](https://github.com/fermyon/spin-fileserver)
library to display the images.

## Templates

Bartholomew uses a [version of the Handlebars template language](https://crates.io/crates/handlebars).
The template syntax is describe in the [Handlebars documentation](https://handlebarsjs.com/).

Every file in the `templates/` directory will be compiled to a template. The file is then
accessible by its relative name, minus the extension. For example. `templates/main.hbs`
will be accessible as `main`.

Note that Bartholomew _expects_ to find a template named `main`. This template is used as
a default when the content head does not contain a `template` directive. It is also
used when an error occurs. You must have a `main` template.

### Accessing The Head (Front Matter) and the Body

The head is available in the template using the `{{ page.head }}` object.
For example, to print the title, use `{{ page.head.title }}`. To access your custom
`[extra]` field named `foo`, use `{{ page.head.extra.foo }}`.

The body is injected to the template converted to HTML. That is, the template does not
have access to the Markdown version of the document.

To print the HTML body without having the output escaped, use `{{{ page.body }}}` (note the
triple curly braces).

## The Bartholomew CLI

This is a very early work in progress implementation for a CLI that simplifies
working with Bartholomew applications:

```
 ➜ bart calendar ./content
Wed, Dec 22, 2021 - 'Documentation' index.md
Thu, Dec 23, 2021 - 'First Post!' 2021-12-23-first-post.md
Thu, Dec 23, 2021 - 'What Is Markdown?' 2021-12-23-what-is-markdown.md
Thu, Dec 23, 2021 - 'Markdown examples' markdown.md

 ➜ bart new post content/blog --author "Radu Matei" --template abc --title "Writing a new post using Bart"
Wrote new post in file content/blog/untitled.md

 ➜ bart check content/* content/blog/*
✅ content/atom.md
✅ content/blog.md
✅ content/functions_example.md
✅ content/helpers_example.md
✅ content/index.md
✅ content/markdown.md
✅ content/redirect_example.md
✅ content/robots.md
✅ content/sitemap.md
✅ content/tag.md
❌ content/blog/2021-12-23-first-post.md        Could not parse file "content/blog/2021-12-23-first-post.md": TOML parsing error: trailing input for key `date` at line 3 column 8
✅ content/blog/2021-12-23-goals-of-bartholomew.md
✅ content/blog/2021-12-23-what-is-markdown.md
Error: One or more pieces of content are invalid
```

title = "Getting Started with Bartholomew"
template = "main"
date = "2022-04-24T21:51:00Z"
[extra]
url = "https://github.com/fermyon/bartholomew/blob/main/docs/content/quickstart.md"
---

## Building from source

To build Bartholomew from source, just run `make build`, which basically does a `cargo build --target wasm32-wasi --release`.

> `make build` also runs `wasm-opt`, which is part of the [Binaryen](https://webassembly.github.io/binaryen/) project. This reduces the size of the WebAssembly module by optimizing the bytecode.

To run Bartholomew, you will need a Wagi-capable runtime. For example, you can just download a recent release of `Wagi` and put it on your `$PATH`. Then the `make serve` command can start it all up for you.

## Install the Fileserver

Bartholomew uses an external file server called [Wagi-Fileserver](https://github.com/deislabs/wagi-fileserver/releases).

Download the latest release and put it in the `modules/` directory. When you are done, you should see:

```console
$ ls modules                                 
README.md          fileserver.gr.wasm
```

## Running Bartholomew

Bartholomew offers multiple ways to run a website as long as you have a wagi-capable runtime:

With Wagi:

```console
$ wagi -c modules.toml
```

With make:

```
$ make serve
```

With Hippo:

```console
$ hippo push
```

For convenience, `make serve` builds the code and then runs `wagi -c`.

## The Bart CLI

We have a very early work in progress implementation for a CLI that simplifies working with Bartholomew applications:

```console
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

You will have to install the binary at `/target/release/bart` onto your `$PATH` to use it. 

title = "Getting Started with Bartholomew"
template = "main"
date = "2022-04-24T21:51:00Z"
[extra]
url = "https://github.com/fermyon/bartholomew/blob/main/docs/content/quickstart.md"
---

## Building from source

To build Bartholomew from source, just run `make build`, which basically does a `cargo build --target wasm32-wasi --release`.

> `make build` also runs `wasm-opt`, which is part of the [Binaryen](https://webassembly.github.io/binaryen/) project. This reduces the size of the WebAssembly module by optimizing the bytecode.

To run Bartholomew, you will need a Spin-capable runtime. For example, you can just download a recent release of `Spin` and put it on your `$PATH`. Then the `make serve` command can start it all up for you.

## Install the Fileserver

Bartholomew uses an external file server called [Spin-Fileserver](https://github.com/fermyon/spin-fileserver).

Download the latest release and put it in the `modules/` directory. When you are done, you should see:

```console
$ ls modules                                 
README.md          spin_static_fs.wasm
```

## Running Bartholomew

Bartholomew offers multiple ways to run a website as long as you have a Spin-capable runtime:

With Spin:

```console
$ spin up
```

With make:

```
$ make serve
```

For convenience, `make serve` builds the code and then runs `spin up`.

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

<<<<<<< HEAD
To use the binary, you will have to build Bartholomew from source targeting the `bart/cargo.toml` manifest:

```
cargo build --release --manifest-file bart/Cargo.toml
```

The generated binary will show up in the `/target/release/` directory and you will then need to move this artifact onto your `$PATH`.cd 
=======
You will have to install the binary at `/target/release/bart` onto your `$PATH` to use it. 
>>>>>>> 7902aa97122cf88bea01316ab1172bf678b508a2

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

For convenience, `make serve` builds the code, and then runs `wagi -c`.
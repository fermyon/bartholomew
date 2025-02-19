# Bartholomew MicroCMS

Bartholomew is a simple CMS-like tool for hosting a website. It is compiled entirely
to WebAssembly, and can run in any Spin-capable system.

The screenshot below shows how Bartholomew is implemented as the official Fermyon website.

![Bartholomew screenshot](docs/static/image/bartholomew-screenshot.png)

## Bartholomew Documentation

Check out [the official documentation](https://developer.fermyon.com/bartholomew/index) and please raise any issues or ask any questions; we are here to help :)

## Prerequisites

To run the Bartholomew CMS, you'll need [Spin](https://developer.fermyon.com/spin/install).

To build Bartholomew or the `bart` command line tool from source, you'll need [Rust](https://www.rust-lang.org/tools/install).

## Basic Authentication

Bartholomew can enforce basic authentication. To enable basic authentication, update the environment for your Spin component and provide the following variables:

```toml
environment = { BASIC_AUTH_USERNAME = "bob", BASIC_AUTH_PASSWORD = "foobar" }
```

Optionally, you can configure the `realm` value of the `WWW-Authenticate` header sent when requests do not contain an `Authorization` header:

```toml
environment = { BASIC_AUTH_USERNAME = "bob", BASIC_AUTH_PASSWORD = "foobar", BASIC_AUTH_REALM = "my-realm" }
```

## Building Bartholomew locally

To build bartholomew on your local machine, you must have rust and the `wasm32-wasi` target installed. Run the following command to build it:

```bash
cargo build --target wasm32-wasip1 --release
```

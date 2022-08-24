title = "Taking Bartholomew for a spin"
date = "2022-05-08T14:05:02.118466Z"

[extra]
url = "https://github.com/fermyon/bartholomew/blob/main/docs/content/quickstart.md"
---

This is a quickstart example of using Spin to deploy a Bartholomew CMS instance, locally on your machine. In this quickstart session, we use a Bartholomew website template which contains some pre-built parts (for your convenience). If you would like to build all of the parts separately from source (Spin, Bartholomew, File Server), please see the [contributing section](https://bartholomew.fermyon.dev/contributing) which dives a lot deeper than this quickstart example.

## Getting the `bart` and `spin` Binaries

First, you need the `bart` and `spin` binaries configured in your path.

### Spin

For Spin, follow [the Spin quickstart guide](https://spin.fermyon.dev/quickstart) which details how to either:
- download the latest Spin binary release,
- clone and install Spin using cargo, or
- clone and build Spin from source.

### Bartholomew CLI

For the `bart` CLI, there are two options:
- download the latest `bart` binary [release](https://github.com/fermyon/bartholomew/releases/), or
- clone and install `bart` from source, using the following commands:

```bash
$ git clone https://github.com/fermyon/bartholomew.git
$ cd bartholomew
$ make bartholomew
```

## Spin Templates

This quickstart method uses a Bartholomew website template. So while we do require `spin` and `bart` CLI (as per the details above) everything else we need (to launch our Bartholomew CMS website) is packaged up in the Bartholomew website template. Including the `bartholomew.wasm` and the `spin_static_fs.wasm` files which take care of Bartholomew's business logic and [Spin's file server](https://github.com/fermyon/spin-fileserver) needs, respectively. We will start working with the template in the next section.

## Create Your First Website With Spin

Now that you have the `spin` in your path, you can use it to install the Bartholomew site template, which we spoke about earlier:

```bash
$ spin templates install --git https://github.com/fermyon/bartholomew-site-template
```

Once installed, you can then create your first website using the installed template:

```bash
$ spin new bartholomew-site my-site
```

The following command will run your new website locally:

```bash
$ cd my-site && spin up --follow-all
```

When you navigate to `http://localhost:3000`, you should see the website running.

## Create Your First Blog Post

You are now ready to start adding content to your new website. You will recall that we installed the `bart` CLI in a previous step. We can use this CLI to create a new blog post page. 

Note how we:
- set the location where the post will be created i.e. `content/blog`,
- set the name of the blog file `protons.md`, and
- specify the `author`, `template` style and `title`.

Feel free to change these values when you run the command on your system:

```bash
$ mkdir content/blog
$ bart new post content/blog protons.md --author "Enrico Fermi" --template "blog" --title "On the Recombination of Neutrons and Protons"
```

If you would like to check the validity of your content, you can use the following `bart check` command. Notice how we specify the location of the content we want to check. In this case, we are checking all of the Markdown files in the blog directory:

```bash
bart check blog/*
```

If your syntax in the `.md` file is **correct**, you will receive an output similar to the following. Otherwise, you will receive an informative error message and a cross:

```bash
✅ content/blog/protons.md
```

## Viewing Your Changes

Running the `spin up` command from above (again) will:

- add the newly created post to the blogs index page at `http://localhost:3000/blog`
- render the post content at `http://localhost:3000/blog/protons`

![Post content rendered by Bartholomew based on the blog template](../static/image/docs/bart-new-post.png)

Next, let's explore how to [build custom templates for our new website](./templates.md).
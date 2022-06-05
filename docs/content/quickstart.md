title = "Taking Bartholomew for a spin"
date = "2022-05-08T14:05:02.118466Z"

[extra]
url = "https://github.com/fermyon/bartholomew/blob/main/docs/content/quickstart.md"
---

## Getting the `bart` and `spin` binaries

First, you need the `bart` and `spin` binaries configured in your path.
For Spin, follow [the Spin quickstart guide](https://spin.fermyon.dev/quickstart)
or head over to GitHub and download [the latest canary release](https://github.com/fermyon/spin/releases/tag/canary).

For `bart`, the Bartholomew CLI, either [follow the contribution guide](./contributing.md)
and build it from source, or download the pre-built binary from [the latest
GitHub release](https://github.com/fermyon/bartholomew/releases/).

## Creating your first website with Bartholomew

Now that you have the `bart` CLI in your path, you can use it to create your
first website from a template:

```bash
# create a new site in the `website/` directory, based on the default Bartholomew site template
$ bart new site --git https://github.com/fermyon/bartholomew-site-template website
Successfully created new Bartholomew website in directory website.
Run spin up --file website/spin.toml to start your website locally.
```

This command will pull the git template locally, and you can start Spin and run
your new website locally:

```bash
$ cd website
$ spin up --follow-all
```

Navigating to `http://localhost:3000`, you should see the website running.

You are now ready to start adding content to your new website. We can use the
`bart new post` command to add a new page. We can specify the directory, file
(which will give us the URL), author, template, and the title:

```bash
$ mkdir content/blog
$ bart new post content/blog protons.md --author "Enrico Fermi" --template "blog" --title "On the recombination of neutrons and protons"
```

Running `spin up` again, this will:

- add the newly created post to the blogs index page at `http://localhost:3000/blog`
- render the post content at `http://localhost:3000/blog/protons`

![Post content rendered by Bartholomew based on the blog template](../static/image/docs/bart-new-post.png)

Next, let's explore how to [build custom templates for our new website](./templates.md).

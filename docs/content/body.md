title = "The Markdown Body"
template = "main"
date = "2022-03-14T00:22:56Z"
[extra]
url = "https://github.com/fermyon/bartholomew/blob/main/docs/content/body.md"
---

Along with front matter, Bartholomew supports generic Markdown support, including usual features like fenced codeblocks. Image links are also supported but require the use of the [Wagi fileserver](https://github.com/deislabs/wagi-fileserver) library to display the images.

A typical content file would look like this, including the header:

```
title = "Awesome piece of content"
template = "main"
date = "2022-03-14T00:22:56Z"
[extra]
url = "https://github.com/fermyon/spin/blob/main/docs/content/redis-trigger.md"
---

Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.

```
title = "Displaying unpublished content"
template = "main"
date = "2022-03-14T00:22:56Z"
[extra]
url = "https://github.com/fermyon/bartholomew/blob/main/docs/content/preview-mode.md"
---

By default, Bartholomew will not display content that is unpublished. Content is unpublished if either:

* The article is marked `published = false` in its head
* The article has a publish date in the future

To view unpublished content, turn on `PREVIEW_MODE`.

Wagi:

```console
$ wagi -c modules.toml -e PREVIEW_MODE=1
```

Make:

```console
$ PREVIEW_MODE=1 make serve
```
Hippo:

Add the environment variable `PREVIEW_MODE=1` to the desired channel.
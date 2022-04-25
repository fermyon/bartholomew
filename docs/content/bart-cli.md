title = "The Bart CLI"
template = "main"
date = "2022-04-24T22:56:00Z"
[extra]
url = "https://github.com/fermyon/bartholomew/blob/main/docs/content/bart-cli.md"
---

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
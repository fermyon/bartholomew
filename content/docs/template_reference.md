title = "Template Quick Reference"
description = "A catalog of values and functions for your templates"
date = "2022-01-27T02:57:20Z"
---

This page provides a quick reference for things you can use in your templates, like values and functions.

## Values Reference

The following values are available in the template. This is formatted in YAML for readability. The three top-level objects are:

- *page:* The content for the current page
- *site:* Info about the entire site
- *env:* Information from the browser or the request, presented as environment variables.

To reference a particular value, use dotted notation. For example, `page` has a `head`, which has the page's `title`.
To access the title, use `{{ page.head.title }}`.


```yaml
# Page holds data specific to the page that matches the URI. For example, the URI /foo loads page data from /content/foo.md
page:
    # The info from the TOML head:
    head:
        date: "2022-01-11T20:08:47Z" # In templates, use `date_format "%B %m, %Y" page.head.date` to format
        title: "WebAssembly Language Support Matrix" # The H1 title
        description: "Tracking the programming languages that compile to WebAssembly (Wasm)."
        template: "page"
        tags: ["webassembly", "programming languages", "javascript", "python", "rust", "dotnet", "ruby"]

        extra: # Remember that everything in extra is optional, so wrap in #if
            author: "Fermyon Staff"
            author_page: "/author/"
            image: "/static/some/thing.png"
            last_modified: "2022-01-11T20:08:47Z" # Only if the page is modified
    body: "The HTML content"
    published: true # Set to false if either manually disabled by author or if `date` is in the future

# Site holds sitewide data
site:
    # Info keeps all of the information from `site.toml`
    info:
        title: "The site title"
        base_url: "https://something" # Note no trailing slash
        about: "Pithy site summary"
        extra:
            your_defined_field: "Value"

    pages: # Metadata for every page on the site. Note that there is no guarantee that `body` will be set.
        "/foo":
            head:
                date: "2022-01-11T20:08:47Z"
                title: "Some title"
                # These are all optional
                description: "Some Description (if set)"
                tags: ["tags", "if", "set"]
                template: "template-if-set"
                # user-defined extras go here
                extras:
                    any: "extras go here"
            published: true
        "/bar":
            head:
                # These fields will always be set.
                date: "2022-01-11T20:08:47Z"
                title: "Some other title"
            published: true

env: # Information about the environment
    PATH_INFO: "/foo" # The absolute path to this page, from the URI
    SERVER_PROTOCOL: "HTTP/1.1"
    REQUEST_METHOD: GET
    REMOTE_HOST: 172.31.28.227
    HTTP_HOST: example.com
    SERVER_NAME: internal.example.com
    SERVER_PORT: 3000
    SCRIPT_NAME: /
    GATEWAY_INTERFACE: CGI/1.1
    HTTP_X_FORWARDED_PROTO: https
    REMOTE_ADDR: 172.31.28.227
    HTTP_X_FORWARDED_PORT: 443
    PATH_TRANSLATED: "/foo"
    X_FULL_URL: http:/example.com:3000/
    HTTP_ACCEPT_LANGUAGE: en-US,en;q=0.9
    SERVER_SOFTWARE: WAGI/1
    X_MATCHED_ROUTE: /
    HTTP_ACCEPT: "text/html"
    QUERY_STRING: ""
    HTTP_USER_AGENT: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/95.0.4638.69 Safari/537.36"
    CONTENT_LENGTH: 0
    CONTENT_TYPE: ""
```

Given the above, for example, you can write a template to create a link to the current page:

```handlebars
<a href="{{site.info.base_url}}{{env.PATH_INFO}}">{{page.head.title}}</a>
```

The above will output something like:

```html
<a href="https://example.com/in-action">WebAssembly In Action</a>
```

## Template Functions and Rhai Scripts

The following template functions are built into Bartholomew:
- `upper STRING`: Given a string, return an uppercase version (`upper "foo"` prints `FOO`)
- `lower STRING`: Given a string, return a lowercase version (`lower "Hello World"` prints `hello world`)
- `now`: Return the current time in the format used by the `date` field
- `date_format STRING DATE`: Given a [format string](https://docs.rs/chrono/latest/chrono/format/strftime/index.html#specifiers) and a date, format the date. (`date_format "%B %d, %Y" "2022-01-11T20:08:47Z"` produces `January 11, 2022`)
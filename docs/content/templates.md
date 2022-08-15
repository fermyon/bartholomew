title = "Bartholomew templates"
date = "2022-05-09T19:23:07.490818Z"

[extra]
url = "https://github.com/fermyon/bartholomew/blob/main/docs/content/templates.md"
---

In Bartholomew, layout is handled via templates. All templates are in the
`templates/` directory.

[Handlebars](https://handlebarsjs.com) is a simple template language well-tuned to HTML.
While the version of Handlebars used in Bartholomew is written in Rust, not JavaScript,
it works almost identically.

## A simple template

Here is a simple HTML template with Handlebars:

```
<!DOCTYPE html>
<html>

<head>
    <title>{{page.head.title}}</title>
</head>

<body>
    {{{page.body}}}
</body>

</html>
```

The above sets the HTML document's title to whatever is in `page.head.title`, and
then fills in the `body` with the value of `page.body`.

Let's take a brief look at the `page` object to understand what is happening here.

### The `page` object

In JSON, the `page` object looks like this:

```
{
    head: {
        title: "Some title",
        description: "Some description",
        template: "an optional template rather than using main.hbs"
        extra: {
            "key": "value",
            "description": "whatever is in the [extra] section of your Markdown doc's header"
        }
    },
    body: "<p>Some rendered Markdown content</p>",
    published: true
}
```

To access a part, you simply use a dotted path notation. So to get the value of `key` in
the `extra` section, we use `{{ page.head.extra.key }}`.

### The `site` object

In addition to the `page` object, there is also a `site` object. `site.pages` contains the `head` section and content of every page in the site. `site.pages` is only populated for templates included in `index_site_pages` in `site.toml` as described in the [configuration section](/configuration.md) 

```
{
    info: {
        title: "site title"
        about: "Site about information"
        base_url: "http://localhost:3000"
        extra: {
            copyright: "site-wide copyring (this is not required, since it's in extra)"
        }
    },
    pages: [
        {...},
        {...}
    ]
}
```

Note that the `site.pages` array has access to every single document in the `content` folder.
This part of the API may change in the future, as it does not scale terribly well. 

### The `env` object

The third top-level object is `env`, which holds all of the environment data.

The `env` object is a set of keys and values:

```
{
    PREVIEW_MODE: "0"
    ...
}
```

You can dump the entire contents of `env` using a template like this:

```
<ul>
    {{#each env}}
    <li><code>{{@key}}</code>: <code>"{{this}}"</code></li>
    {{/each}}
</ul>
```

### The `request` object

The fourth top-level object is `request`, which holds all the details about
the HTTP request, the path of this resource, and other Spin information.

The `request` object is a set of keys and values:

```
{
    spin-full-url: "http://localhost:3000/test"
    ...
}
```

You can dump the entire contents of `request` using a template like this:

```
<ul>
    {{#each request}}
    <li><code>{{@key}}</code>: <code>"{{this}}"</code></li>
    {{/each}}
</ul>
```

## Including a template

It is possible to include a template into another template.
For example, if we want to include the `navbar.hbs` template, we use a "partial" include
like this:

```
{{> navbar }}
```

Note that we drop the `.hbs` suffix when including this way.

## Calling template helpers

There are a few template helpers defined in Bartholomew.

For example, to change a piece of text to all-caps, use the `upper` helper:

```
{{ upper "hello" }}
```

The above will render `HELLO`.

Note that you can create custom template helpers using [Rhai scripts](./scripting.md).

### Defined helper functions

The following helper functions are provided with Bartholomew

- `upper STRING`: converts the given string to uppercase
- `lower STRING`: converts the given string to lowercase
- `date_format STRING DATE`: Formats a date using the given string. `date "%Y" page.head.date`. Use [strftime format](https://docs.rs/chrono/latest/chrono/format/strftime/index.html#specifiers).
- `now STRING`: Formats the current date using the given string. This uses the same format as above. 
- `trim STRING`: Remove whitespace from beginning and end of string.
- `trunc UINT STRING`: Truncate the string to length `UINT`.
- `abbrev UINT STRING`: Shorten the string to UINT with elipses. For example `abbrev 8 "Functions Example"` returns `Funct...`.
- `plural INT SINGULAR_STRING PLURAL_STRING`: If `INT` is `1`, return the singular form. Otherwise return plural: `plural 1 "apple" "apples"` returns `apple`. But if we change `1` to `2` (or `0`) it will return `apples`.

### Values reference

The following values are available in the template. This is formatted in YAML for readability. The four top-level objects are:

- *page:* The content for the current page
- *site:* Info about the entire site
- *env:* Information about the environment variables.
- *request* - Information about the browser, HTTP request and other spin information.

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

env: # Environment variables set in the Bartholomew Wasm module

request: # HTTP request data along with spin information.
```

Given the above, for example, you can write a template to create a link to the current page:

```html
<a href="{{site.info.base_url}}{{env.PATH_INFO}}">{{page.head.title}}</a>
```

The above will output something like:

```html
<a href="https://example.com/in-action">WebAssembly In Action</a>
```

### Template Functions and Rhai Scripts

The following template functions are built into Bartholomew:
- `upper STRING`: Given a string, return an uppercase version (`upper "foo"` prints `FOO`)
- `lower STRING`: Given a string, return a lowercase version (`lower "Hello World"` prints `hello world`)
- `now`: Return the current time in the format used by the `date` field
- `date_format STRING DATE`: Given a [format string](https://docs.rs/chrono/latest/chrono/format/strftime/index.html#specifiers) and a date, format the date. (`date_format "%B %d, %Y" "2022-01-11T20:08:47Z"` produces `January 11, 2022`)

Next, let's dive into [how site configuration works](./configuration.md).

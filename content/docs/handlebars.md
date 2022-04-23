title = "Templating with Handlebars"
description = "Use Handlebars to customize the look and feel of your site"
date = "2021-12-23T17:05:19Z"
---
In Bartholomew, layout is handled via templates. All templates are in the
`templates/` directory.

[Handlebars](https://handlebarsjs.com) is a simple template language well-tuned to HTML.
While the version of Handlebars used in Bartholomew is written in Rust, not JavaScript,
it works almost identically.

## A Simple Template

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

### The Page Object

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

### The Site Object

In addition to the `page` object, there is also a `site` object:

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

### The Env Object

The third top-level object is `env`, which holds all of the environment data, including details about
the HTTP request, the path of this resource, and other Spin information.

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

Note that very little data validation is done on incoming environment variables, so you should validate or scrub any values before showing them to the user.

## Including A Template

It is possible to include a template into another template.
For example, if we want to include the `navbar.hbs` template, we use a "partial" include
like this:

```
{{> navbar }}
```

Note that we drop the `.hbs` suffix when including this way.

## Calling Template Helpers

There are a few template helpers define in Bartholomew.

For example, to change a piece of text to all-caps, use the `upper` helper:

```
{{ upper "hello" }}
```

The above will render `HELLO`.

Note that you can create custom template helpers using [Rhai scripts](/docs/rhai).

### Defined Helper Functions

The following helper functions are provided with Bartholomew

- `upper STRING`: converts the given string to uppercase
- `lower STRING`: converts the given string to lowercase
- `date_format STRING DATE`: Formats a date using the given string. `date "%Y" page.head.date`. Use [strftime format](https://docs.rs/chrono/latest/chrono/format/strftime/index.html#specifiers).
- `now STRING`: Formats the current date using the given string. This uses the same format as above. 
- `trim STRING`: Remove whitespace from beginning and end of string.
- `trunc UINT STRING`: Truncate the string to length `UINT`.
- `abbrev UINT STRING`: Shorten the string to UINT with elipses. For example `abbrev 8 "Functions Example"` returns `Funct...`.
- `plural INT SINGULAR_STRING PLURAL_STRING`: If `INT` is `1`, return the singular form. Otherwise return plural: `plural 1 "apple" "apples"` returns `apple`. But if we change `1` to `2` (or `0`) it will return `apples`.

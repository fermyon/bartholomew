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
title = "Scripting guide"
date = "2022-05-08T14:05:02.118466Z"

[extra]
url = "https://github.com/fermyon/bartholomew/blob/main/docs/content/scripting.md"
---

Sometimes you want to do something special in your templates. Perhaps it's some
fancy formatting or iterating through some content and finding specific information.
Bartholomew provides the Rhai scripting language for this.

[Rhai](https://rhai.rs/) is a simple scripting language that has many stylistic similarities to
Rust, Go, and Python. The integration with Bartholomew is easy to work with.

## The basic

1. Create a file in `scripts/` with the extension `.rhai`
2. Put some Rhai code in there
3. Have the code return the value you care about
4. Use it in a template.

For example, let's take a look at the `scripts/echo.rhai` script. Note that because
it is named `echo.rhai`, it will be accessible inside of templates as `echo` (just remove
the `.rhai`).

```rust
let msg = params[0];

"hello " + msg;
```

On the first line, we accept the first parameter from the template (`params[0]`) and
assign it to the variable named `msg`.

> Variables are mutable in Rhai.

The second line returns the string `"hello" ` concatenated with the value of `msg`.
That's all there is to a simple Rhai script.

From a template, we can then use this script like this:

```
{{ echo "world" }}
```

When we run the template, we will see:

```
hello world
```

## A more complicated example.

Let's take a look at `scripts/blogs.rhai`, which is called in a template as `blogs`.
This script makes a list of all of the blog posts for the site.

```rust
// Param 1 should be `site.pages`
let pages = params[0];

// Loop through them and return all of the page objects that are in
// the blog path. We want the results in an array to preserve ordering.
let blog_pages = [];

// Get each blog post, assigning it to {path: object}.
let keys = pages.keys();
for item in keys {
    if item.index_of("/content/blog/") == 0 {
        // Remove /content and .md
        let path = item.sub_string(8);
        path = path.sub_string(0, path.index_of(".md"));

        // Build an object that has `uri` and `page` keys.
        blog_pages.push(#{
            uri: path,
            page: pages[item],
        });
        //blog_pages[path] = pages[item];
    }
   
}
// Newest to oldest, assuming you put the date in the URI
blog_pages.reverse();

// Return the array of blog pages
blog_pages
```

Again note that this is one long procedural script that starts by fetching a parameter
from the `params` array, and ends by sending back the output of the last line, `blog_pages`.

The script returns a more complex data type, so let's see how this one is used in the
`content_sidebar.hbs` template:

```html
<div class="p-4">
    <h4 class="fst-italic">Recent Posts</h4>
    <ol class="list-unstyled mb-0">
        {{#each (blogs site.pages)}}<li><a href="{{uri}}">{{page.head.title}}</a></li>
        {{/each }}
    </ol>
</div>
```

The code `{{#each (blogs site.pages)}}` calls `blogs` with `param[0]` set to `site.pages`.
Then the `each` loops through the results.

The value of `this` within the `#each` loop is the object that we created in Rhai:

```rust
#{
    uri: "/some/path"
    page: #{head: #{...}, body: "some html" }
}
```

So `<a href="{{uri}}">{{page.head.title}}</a>` will use `this.uri`, and the `title`
from the `page` object.

That's how you can use Rhai to add custom formatters to the site.

No site would be complete without quality content, so let's take a look at the [markdown guide](./markdown.md) which will help you create and format your awesome content.
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

## Building on the example

When a site has a lot of blog posts, it would make sense to implement pagination for the content. This will introduce the idea of working with objects and nesting function calls. 

Firstly, we will need need a script to parse the URL to obtain the page number to generate the pagination list.
```rust
let full_url = params[0];
let query_params = #{page: 0};
let query_string;
let query_param;

if full_url.contains("?") {
    // Get the query string
    query_string = full_url.sub_string(full_url.index_of("?") + 1);
    do {
        // Seperate by parameters
        if query_string.contains("&") {
            query_param = query_string.sub_string(0, query_string.index_of("&"));
            query_string = query_string.sub_string(query_string.index_of("&") + 1);
        } else {
            query_param = query_string;
            query_string.clear();
        }
        // set the page parameter to the value from the url
        if query_param.sub_string(0, query_param.index_of("=")) == "page" {
            try {
            query_params.page = parse_int(query_param.sub_string(query_param.index_of("=") + 1));
            } catch {
                query_params.page = 0;
            }
        }
    } while ( query_string.len() > 0)
}

// Return the query_params
query_params
```
The script above currently return an object containing only the page parameter. It can be easily extended to include other parameters as required. To access the object inside handlebars template, the following can be used.

```html
{{#with (url_query_params request.spin-full-url)}}
    <span> Page number: {{page}} </span> 
{{/with}}
```

The `#with` keyword brings the parameters of the object into the context of the template part allowing to be directly used.

The next step to create pagination would be to create the logic for pagination.

```rust
let arr = params[0];
let index = params[1];
let offset = params[2];

let pagination = #{
    prev: "",
    next: "",
    subarr: []
};

pagination.subarr = arr.extract(index * offset, offset);

if (arr.len > (index*offset) + offset ) {
    pagination.next = index + 1;
}

if index > 0 {
    if (arr.len < index*offset) {
        pagination.prev = arr.len/offset;
    }
    else {
        pagination.prev = index - 1;
    }
}
pagination
```

This script allows takes in 3 arguments, one for the array to paginate, one for the index and the last one for the offset. It returns a pagination object with the sub array along with the index to the next and previous index. The pagination object can be used to create the listing along with the navigation, where each page has 2 objects.

```html
{{#with (url_query_params request.spin-full-url)}}
    {{#with (pagination (blog site.pages) page 2)}}
        <div class="p-4">
            <h4 class="fst-italic">Posts</h4>
            <ol class="list-unstyled mb-0">
                {{#each subarray}}<li><a href="{{uri}}">{{page.head.title}}</a></li>
                {{/each }}
            </ol>
            {{#if prev}}
                <a href="/blog?page={{prev}}">Previous</a>
            {{/if}}
            {{#if next}}
                <a href="/blog?page={{next}}">Next</a>
            {{/if}}
        </div>
    {{/with}}
{{/with}}
```

No site would be complete without quality content, so let's take a look at the [markdown guide](./markdown.md) which will help you create and format your awesome content.

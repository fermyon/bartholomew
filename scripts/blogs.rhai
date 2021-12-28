// This function lists all of the blog posts, subject to the constraints.
//
// It returns an array of objects of the form:
//  [
//    #{ uri: "path/to/page", page: PageObject }
// ]

// Param 1 should be `site.pages`
let pages = params[0];

// Loop through them and return all of the page objects that are in
// the blog path.

let blog_pages = [];

// Get each blog post, assigning it to {path: object}.
let keys = pages.keys();
for item in keys {
    if item.index_of("/content/blog/") == 0 {
        // Remove /content and .md
        let path = item.sub_string(8);
        let page = pages[item];

        path = path.sub_string(0, path.index_of(".md"));
        blog_pages.push(#{
            uri: path,
            page: page,
        });
        //blog_pages[path] = pages[item];
    }
   
}

// Return the blogs sorted newest to oldest
fn sort_by_date(a, b) {
    if a.page.head.date < b.page.head.date {
        1
    } else {
        -1
    }
}

// Sort by the value of the page date.
blog_pages.sort(Fn("sort_by_date"));

blog_pages
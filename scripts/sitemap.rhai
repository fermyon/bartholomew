// This function lists all of the posts, filtering a few.
//
// It returns an array of objects of the form:
//  [
//    #{ uri: "path/to/page", page: PageObject }
// ]

// These should be skipped.
let disallow = [
    "/sitemap", // Don't list self.
    // "/tag", // tag will list all of the tags on a site. If you prefer this not be indexed, uncomment this line.
    "/index", // This is a duplicate of /
    "/atom",
    "/robots",
];

// Param 1 should be `site.pages`
let pages = params[0];

let site_pages = [];
let keys = pages.keys();
for item in keys {
    let path = item.sub_string(8);
    let page = pages[item];

    path = path.sub_string(0, path.index_of(".md"));
    if !disallow.contains(path) {
        site_pages.push(#{
            uri: path,
            page: page,
            priority: prioritize(path),
            frequency: "weekly",
        });
    }
   
}

// This is an example of how we could prioritize based on information about the page.
//
// Specifically, here we use path to boost docs and blogs while reducing the priority
// of author pages and features.
fn prioritize(path) {
    let boost = ["/blog/", "/docs/"];
    for sub in boost {
        if path.contains(sub) {
            return 0.8
        }
    }
    let nerf = ["/author/", "/features/"];
    for sub in nerf {
        if path.contains(sub) {
            return 0.3
        }
    }
    0.5
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
site_pages.sort(Fn("sort_by_date"));

site_pages
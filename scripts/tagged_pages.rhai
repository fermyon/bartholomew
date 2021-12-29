// This function lists all pages that are tagged with the given tag.
//
// It returns an array of objects of the form:
//  [
//    #{ uri: "path/to/page", page: PageObject, weight: int }
// ]

// Param 1 should be `site.pages`
let pages = params[0];

// Param 2 should be the tag name
let tag = params[1];

// Loop through them and return all of the page objects that are in
// the blog path.
let tagged_pages = [];

// Get each blog post, assigning it to {path: object}.
let keys = pages.keys();
for item in keys {
    // Remove /content
    let path = item.sub_string(8);
    let page = pages[item];

    if page.head.tags.len == 0 {
        continue;
    }

    let tag_index = page.head.tags.index_of(tag);
    if tag_index >= 0 {
        // Remove .md
        path = path.sub_string(0, path.index_of(".md"));
        tagged_pages.push(#{
            uri: path,
            page: page,
            weight: tag_index,
        });
    }
}

// Return the matches, sorted by weight
fn sort_by_weight(a, b) {
    if a.weight < b.weight {
        -1
    } else {
        1
    }
}

// Sort by how close to the front of the tag list a given tag was.
tagged_pages.sort(Fn("sort_by_weight"));

tagged_pages
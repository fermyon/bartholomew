// Param 1 should be `site.pages`
let pages = params[0];

let tags = [];
let keys = pages.keys();
for item in keys {
    let path = item.sub_string(8);
    let page = pages[item];
    for tag in page.head.tags {
        if !tags.contains(tag) {
            tags.push(tag);
        }
    }
}

tags
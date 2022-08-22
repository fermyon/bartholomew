title = "Search Engine Optimization (SEO)"
date = "2022-08-09T14:05:02.118466Z"

[extra]
url = "https://github.com/fermyon/bartholomew/blob/main/docs/content/seo.md"
---

## Google Verification using Bartholomew

Your Bartholomew implementation can be <a href="https://support.google.com/webmasters/answer/9008080?hl=en" target="_blank">officially verified with Google</a>. This is an important first step because the verification process provides you with access to the <a href="https://g.co/kgs/UcBemE" target="_blank">Google Search Console</a>. The Google Search Console (formerly Google Webmaster Tools) allows webmasters to check indexing status, search queries, crawling errors and also optimize visibility of their site.

Let's take a look at how the verification process is accomplished using Bartholomew. 

### Markdown

The first step in the Google verification process is where Google provides you (the owner) of the specific website with a specially named file i.e. `abcdefg.html`. Google now wants you, the owner, to make this file available on your site, so that Google can fetch it as proof that you are the site's owner which has access control to the site. This is a really simple task. First you create a Markdown file (in Bartholomew's `content` directory) called `abcdefg.md` (the name of this file just has to match the name of the file which Google provided). **Note: We are creating an `.md` file here not an `.html` file**.

Bartholomew uses templating so you just have to be explicit about a couple of things inside that new `.md` file. Specifically,  Make sure that there is a template name (we will create the template next) and that the content type (of this file) is rendered as `text/html`. This is shown in the source code of the new `abcdefg.md` file below.

```
title = "Google Verification"
description = "Google verification file which provides us with access to Google Search Console"
date = "2022-07-11T00:01:01Z"
template = "google_verification"
content_type = "text/html"
---

This is the abcdefg.html file that Google can see openly at the root of the website.
```

### Template

Bartholomew uses <a href="https://handlebarsjs.com/" target="_blank">Handlebars</a> templating. Therefore the next step is for you to go ahead and create a new `google_verification.hbs` file in Bartholomew's template directory. Once the file is created, populate it with the content which Google requested, below is just an example.

```
{{! 
For info on what can be placed here, see https://support.google.com/webmasters/answer/9008080#html_verification&zippy=%2Chtml-file-upload
}}
google-site-verification: abcdefg.html
```

At this point the verification button in the Google dashboard can be pressed; Google fetches the file from your site and the verification is complete!

## Search Engine Optimization (SEO) using Bartholomew

In addition to just verifying the ownership of a site/domain, You can see that there are <a href="https://developers.google.com/search/docs/beginner/seo-starter-guide" target="_blank">specific SEO requirements</a> in relation to how the <a href="https://g.co/kgs/p6qtQs" target="_blank">Googlebot</a> indexes content. Googlebot is the web crawler software used by Google.

Let's take a look at how the SEO compliance (i.e. sitemap and robots.txt) is accomplished using Bartholomew.

### Generating a Sitemap

Google <a href="https://developers.google.com/search/docs/advanced/sitemaps/build-sitemap" target="_blank">expects the standard sitemap protocol to be implemented</a>. Thankfully, Bartholomew automatically builds a sitemap file based on the entire set of content in the CMS. The heavy lifting of the work is performed using the <a href="https://rhai.rs/" target="_blank">Rhai</a> scripting language. Here is an example of the `sitemap.rhai` file that you would store in Bartholomew's scripts directory.

```
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
```

When Bartholomew sees an incoming request for the `sitemap.xml` URL, it will look inside the scripts directory for a Rhai file named `sitemap.rhai` (as shown above) and execute the script on demand.

In conjunction to the above scripting, the aforementioned <a href="https://handlebarsjs.com/" target="_blank">Handlebars</a> templating assists in this work being performed dynamically (using variables common between the script and the template); as shown in the `sitemap.hbs` file's contents below.

```
<?xml version="1.0" encoding="UTF-8" ?>
{{!
    For sitemap.xml, see https://www.sitemaps.org/protocol.html
    For date/time format, see https://www.w3.org/TR/NOTE-datetime
}}
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    <url>
        <loc>{{site.info.base_url}}/</loc>
        <changefreq>daily</changefreq>
        <priority>0.8</priority>
    </url>
    {{#each (sitemap site.pages) }}
    <url>
        <loc>{{../site.info.base_url}}{{uri}}</loc>
        {{#if page.head.date }}<lastmod>{{date_format "%Y-%m-%dT%H:%M:%SZ" page.head.date}}</lastmod>{{/if}}
        <changefreq>{{frequency}}</changefreq>
        <priority>{{priority}}</priority>
    </url>
    {{/each}}
</urlset>
```

**If you ever need assistance with any of the scripting, templating or Markdown mentioned here, please go ahead and jump into our <a href="https://discord.gg/AAFNfS7NGf" target="_blank">Discord</a> server. We are here to assist and would love to see what you are building. Alternatively, place [an issue in GitHub](https://github.com/fermyon/bartholomew/issues) and ask for help.**

From a display point of view we again just use Markdown (create a `sitemap.md` file in the site's content directory, correctly reference the name of the template (`sitemap`) and then ensure that the content type is set to `text/xml`). The above process will generate an XML sitemap called `sitemap.xml` at the root of the site. Perfect!

```
title = "Sitemap XML file"
description = "This is the sitemap.xml file"
date = "2021-12-29T22:36:33Z"
template = "sitemap"
content_type = "text/xml"
---

This is the autogenerated sitemap. Note that the suffix .xml is replaced with .md by Bartholomew.
```

### Creating a Robots File

You can actually control the Googlebot and tell it which files it may access on the site. This is done via the use of <a href="https://developers.google.com/search/docs/advanced/robots/create-robots-txt" target="_blank">a robots.txt file</a>.

Similarly to the process above, you create a `robots.md` Markdown file in the content directory and also a `robots.hbs` in the template directory. These are shown below (in that order).

```
title = "Robots"
description = "This is the robots.txt file"
date = "2021-12-30T03:17:26Z"
template = "robots"
content_type = "text/plain"
---

This is the robots.txt file. It is autogenerated.
```

```
{{! 
For info on what can be placed here, see http://www.robotstxt.org/
See also: https://developers.google.com/search/docs/advanced/robots/intro
}}
User-agent: *
Sitemap: {{site.info.base_url}}/sitemap.xml
Disallow: /index
```

## Google Search Console

The above steps of a) verifying and b) complying to the SEO requirements will give you great control over what is indexed by the Googlebot, and other Web Crawlers also. From a Google Search Console perspective specifically, you can now enjoy specific features and benefits such as on-demand page indexing i.e. allowing Google to go ahead and index specific content (like a new blog post) and much more.

## Google Search Analytics

<a href="https://g.co/kgs/xZqj9L" target="_blank">Google Analytics</a> tracks and reports web site traffic. Showing not only where users are visiting from but how long they are staying and which pages they are reading and so forth. Once Google Analytics is configured you can even see how many users are on the site in real time. It is recommended that Google Analytics be used in conjunction with the above SEO.

## Rich Results

Implementing rich results will make your listings stand out from other listings. Events, job listings, COVID-19 announcements and recipes all qualify to use rich results (a structured data markup approach to content). Whilst rich results can make your content more visually appealing, implementing rich results can also help search engines to crawl and rank your content which may result in higher rankings.

### Structured Data

Rich results are created using structured data inside your content. For example, <a href="https://www.w3.org/TR/json-ld11/" target="_blank">JSON-LD</a>, a JSON-based serialization for linked data.

#### Video

Let's go ahead and create a JSON-LD code snippet for a video. In this case, we go straight to <a href="https://developers.google.com/search/docs/advanced/structured-data/video" target="_blank">Google Search Console Documentation</a>, specifically the <a href="https://developers.google.com/search/docs/advanced/structured-data/video#video-object" target="_blank">JSON-LD Video Object</a> section and read about the properties i.e. `name`, `description`, `contentUrl` and so forth.

>> Please note: That there are other online JSON-LD resources, including this <a href="https://json-ld.org/playground/" target="_blank">JSON-LD Playground</a> which lets you choose from pre-made JSON-LD examples to get started. Another great resource is the <a href="https://jsonld.com/" target="_blank">jsonld.com</a> site which is intended for webmasters seeking pre-made and validated JSON-LD markup. Also, Google has a <a href="https://codelabs.developers.google.com/codelabs/structured-data/index.html#0" target="_blank"> structured data code lab</a> that walks you through adding several structured data examples to a page (which takes approximately 1/2 hour to complete).

The following is an example of the JSON-LD that we used (in a particular blog post). 

>> Note: Whilst you can see URLs to the actual video content, this rich results data is just metadata. In addition to this data, the actual video embedding takes place elsewhere in the page's content.

```javascript
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "VideoObject",
  "name": "Fermyon Technologies, Inc. are attending Open Source Summit Europe 2022 Dublin Ireland.",
  "description": "Fermyon Technologies, Inc. are thrilled to be joining the Open Source Summit Europe 2022 in Dublin Ireland; from September 13th to 16th.",
  "thumbnailUrl": "https://www.fermyon.com/static/image/icon/mstile-150x150.png",
  "uploadDate": "2022-08-13T08:00:00+08:00",
  "duration": "PT0M24S",
  "contentUrl": "https://youtu.be/UUzscJY6j2A",
  "embedUrl": "https://www.youtube.com/embed/UUzscJY6j2A"
}
</script>
```

Once the rich results code is created we can simply embed it into our Bartholomew blog post's Markdown file.

#### Testing Rich Results

When you are finished adding the rich media, you can go ahead and make sure that the content passes the "rich results test" [rich results test](https://search.google.com/test/rich-results). This is a free online tool, which allows you to test both public URLs to content and also pasted-in HTML source code.

Here is an example of how the code block from above is rendering in the site's HTML and therefore passing the rich results test.

![Rich results test](/static/image/rich-results-01.png)

The test only takes a couple of seconds, and as you can see we have successfully added rich result video data to our blog post.

![Rich results test](/static/image/rich-results-02.png)

#### Monitoring Rich Results

Once deployed be sure to check your rich result [status reports](https://support.google.com/webmasters/answer/7552505). The status reports essentially monitor the health of your existing pages, on an ongoing basis. A great way to quickly check that everything is still working as time goes by.

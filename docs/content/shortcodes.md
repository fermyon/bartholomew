title = "Bartholomew shortcodes"
date = "2022-05-09T19:23:07.490818Z"
enable_shortcodes = true

[extra]
url = "https://github.com/fermyon/bartholomew/blob/main/docs/content/shortcodes.md"
---

Shortcodes are simple reusable snippets can be used inside the markdown content.

Bartholomew supports shortcodes simplify sharing content between different markdown files. 

## The basic

1. Create a file in shortcodes/ with the extension `.rhai`
2. Put some Rhai code in there
3. Make the code return the required value
4. Enable shortcodes in the markdown document
5. Use it in the content

## Creating a shortcode

An example of a shortcode is in `shortcodes/alert.rhai`. The shortcode is available in the markdown file as `alert` just like for scripts.

```rust
let type = params[0];
let msg = params[1];


let colors = #{
  primary:`alert-primary`,
  success:`alert-success`,
  warning: `alert-warning`,
  danger: `alert-danger`,
};

let icons = #{
  primary:`#info-fill`,
  success:`#check-circle-fill`,
  warning: `#exclamation-triangle-fill`,
  danger: `#exclamation-triangle-fill`,
};
`<svg xmlns="http://www.w3.org/2000/svg" style="display: none;">
  <symbol id="check-circle-fill" fill="currentColor" viewBox="0 0 16 16">
    <path d="M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 
    0zm-3.97-3.03a.75.75 0 0 0-1.08.022L7.477 9.417 5.384 7.323a.75.75
     0 0 0-1.06 1.06L6.97 11.03a.75.75 0 0 0 1.079-.02l3.992-4.99a.75.75 0 0 0-.01-1.05z"/>
  </symbol>
  <symbol id="info-fill" fill="currentColor" viewBox="0 0 16 16">
    <path d="M8 16A8 8 0 1 0 8 0a8 8 0 0 0 0 16zm.93-9.412-1 4.705c-.07.34.029.533.304.533.194
     0 .487-.07.686-.246l-.088.416c-.287.346-.92.598-1.465.598-.703
      0-1.002-.422-.808-1.319l.738-3.468c.064-.293.006-.399-.287-.47l-.451-.081.082-.381
       2.29-.287zM8 5.5a1 1 0 1 1 0-2 1 1 0 0 1 0 2z"/>
  </symbol>
  <symbol id="exclamation-triangle-fill" fill="currentColor" viewBox="0 0 16 16">
    <path d="M8.982 1.566a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98
     1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566zM8 5c.535 0 .954.462.9.995l-.35
      3.507a.552.552 0 0 1-1.1 0L7.1 5.995A.905.905 0 0 1 8 5zm.002 6a1 1 0 1 1 0 2 1 1 0 0 1 0-2z"/>
  </symbol>
</svg>

<div class="alert ` + colors[type] + ` d-flex align-items-center" role="alert">
  <svg class="bi flex-shrink-0 me-2" width="24" height="24" role="img" aria-label="Info:"><use xlink:href="` + icons[type]  +`"/></svg>
  <div>
` + msg + `
  </div>
</div>
`


```

## Enabling shortcodes

To enable shortcodes support for a particular document, the value of `enable_shortcodes = true` must be set in the page head.

```
title = "A New Article"
description = "This article is really interesting and full of useful material."
date = "2021-12-23T15:05:19Z"
...
...
enable_shortcodes = true
---
```

## Using shortcodes

Now the shortcode can be used in the markdown file by calling it with the required arguments. For the alerts script, this is the type of alert and the message to be displayed.

```
\{{ alert "warning" "Bartholomew is a work in progress" }}
```
Which renders as the following.

{{ alert "warning" "Bartholomew is a work in progress" }}

## Note while using shortcodes

An important note to be considered while using shortcodes is that the `\{{ }}` must be escaped if they are not meant to be executed. This is only required in the content files where shortcodes are enabled.

To escape the code, the `\` is used like 
{{{{raw}}}}
`\\{{ alert "warning" "Bartholomew is a work in progress" }}`
{{{{/raw}}}}
which will render `\{{ alert "warning" "Bartholomew is a work in progress" }}` instead of running the shortcode.



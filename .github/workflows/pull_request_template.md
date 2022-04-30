Content must go through a pre-merge checklist.

## Pre-Merge Content Checklist

This post has been checked to ensure that:

- [ ] The `title`, `date`, and `url` are all set
- [ ] File does not use CRLF, but uses plain LF
- [ ] Has been manually tested by running in Spin/Bartholomew
- [ ] Handlebars template URL's are all in sync

- Twitter card
    - [ ] Is added in `/static/image`
    - [ ] Is referenced in `[extra]` with `image` and `twitter_card_type`
    - [ ] *OR* will not be created for this post

For non-content merges (code/image/template, etc), you may delete the checklist.
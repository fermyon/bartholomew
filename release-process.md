# Cutting a Bartholomew Release

To cut a release of Bartholomew, you will need to do the following:

1. Create a pull request that changes the version number for your new version (e.g. 1.2.2 becomes 1.2.3)
    - `Cargo.toml` and `bart/Cargo.toml` are the most important places to make this change
    - Check the docs for hard-coded version strings
1. Merge the PR created in #1 (Such PRs are still required to get approvals, so make sure you get signoff on the PR)
1. Create a new tag with a `v` and then the version number (`v1.2.3`)
1. Push the tag to origin (eg `git push origin v1.2.3`)
    - This will trigger a release build
    - The release build will attach the `bartholomew.wasm`, `bart` CLI and checksums file to the GitHub release
    - The release notes will be auto-generated, but you can edit as needed, for instance to alert about breaking changes or other notable items
1. Update [bartholomew-site-template](https://github.com/fermyon/bartholomew-site-template/tree/main) with the latest `wasm` module.

At this point, you can just verify that all things are good.
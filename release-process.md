# Cutting a Bartholomew Release

To cut a release of Bartholomew, you will need to do the following:

1. Create a pull request that changes the version number for your new version (e.g. 1.2.2 becomes 1.2.3)
    - `Cargo.toml` and `bart/Cargo.toml` are the most important places to make this change
    - Check the docs for hard-coded version strings
2. Merge the PR created in #1 (Such PRs are still required to get approvals, so make sure you get signoff on the PR)
3. Create a new tag with a `v` and then the version number (`v1.2.3`)
4. Push the tag up to `main` on GitHub
    - This will trigger a release build
5. Build WebAssembly binary, `batholomew.wasm`
6. Generate SHAs with `shasum -a 256 bartholomew.wasm ` or a similar command
7. Go to the GitHub [tags page](https://github.com/fermyon/bartholomew/releases) and create a release, adding release notes, and uploading `bartholomew.wasm`. The SHAs should go in the release notes.

At this point, you can just verify that all things are good.
# Contributing

## Prerequisites

- Camsense-X1 LiDAR sensor
- Rust 1.86+
- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)

## Release Process

- Make sure CI passes.
- Update version number.
- Update changelog following the [Common Changelog](https://common-changelog.org) style guide
  (as much as possible, of course).
- Push any remaining changes.
- Create tag for latest commit (v[0-9]+\.[0-9]+\.[0-9]+): 

  ```shell
  git tag -a <tag> -m "<tag message e.g. current date>"
  ```
  
  As an example, for version `1.2.3` on the `01.01.2031` you would do:

  ```shell
  git tag -a v1.2.3 -m "Version v1.2.3 - 2031.01.01"
  ```

- Push the tag:
  
  ```shell
  git push --tags
  ```

- Let CI handle the rest.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.


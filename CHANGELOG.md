# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.1.1] - 2017-06-16

### Fixed

- `Ref`'s `Copy` and `Clone` implementation. Previously `Copy` was only
  implemented for `Ref<'a, T>` where `T: Copy`; now it's implemented for any
  `T`. The same thing happened to its `Clone` implementation.

## v0.1.0 - 2017-04-20

- Initial version

[Unreleased]: https://github.com/japaric/static-ref/compare/v0.1.1...HEAD
[v0.1.1]: https://github.com/japaric/static-ref/compare/v0.1.0...v0.1.1

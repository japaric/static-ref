# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.2.0] - 2017-06-27

### Changed

- [breaking-change] `Ref` and `RefMut` have been dropped in favor of the
  `Static` newtype. `&'a Static<T>` is equivalent to `Ref<'a, T>` and `&'a mut
  Static<T>` is equivalent to `RefMut<'a, T>`. `Static<T>` supports dynamically
  sized types (`T: ?Sized`)

### Removed

- The `StaticRef` trait

## [v0.1.1] - 2017-06-16

### Fixed

- `Ref`'s `Copy` and `Clone` implementation. Previously `Copy` was only
  implemented for `Ref<'a, T>` where `T: Copy`; now it's implemented for any
  `T`. The same thing happened to its `Clone` implementation.

## v0.1.0 - 2017-04-20

- Initial version

[Unreleased]: https://github.com/japaric/static-ref/compare/v0.2.0...HEAD
[v0.2.0]: https://github.com/japaric/static-ref/compare/v0.1.1...v0.2.0
[v0.1.1]: https://github.com/japaric/static-ref/compare/v0.1.0...v0.1.1

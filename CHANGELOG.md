# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.1

### Added

- "Partial" constructors for all `Vectorlike` types:
  - `Vector::from_x()`, `Vector::from_y()`
  - `Point::from_x()`, `Point::from_y()`
  - `Size::from_width()`, `Size::from_height()`
- `From<Size>` implementations for `Rect` and `SizedRect`.

## v0.1.0

- Initial release.

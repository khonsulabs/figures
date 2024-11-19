# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Breaking Changes

- `wgpu` has been updated to `23.0.0`.

## v0.4.3 (2024-11-19)

### Added

- Trait `StdNumOps` defines saturating math operations. All unit types, `Size`,
  and `Point` implement this trait.
- `Rect::saturating_extents` returns the extents of a rectangle using saturating
  math.

## v0.4.2 (2024-10-20)

### Added

- `Duration` now implements `Ranged`.

## v0.4.1 (2024-10-07)

### Changed

- `Fraction::new` now contains a `debug_assert!` ensuring the denominator is
  non-zero.

### Added

- `Rect` now implements `FloatConversion` when its unit type implements it.

## v0.4.0 (2024-07-22)

### Breaking Changes

- `wgpu` has been updated to `v22.0.0`.

## v0.3.0 (2024-05-12)

### Breaking Changes

- `Px`, `UPx`, and `Lp` no longer directly have an associated constant `ZERO`.
  This constant is still available via the `Zero` trait.
- This crate's `wgpu` version is now `0.20.0`.
- This crate's `winit` version is now `0.30.0`.

## v0.2.2 (2024-01-04)

### Added

- `Size` and `Point` now implement both to/from winit's
  PhysicalSize/PhysicalPosition for both `i32`/`Px` and `u32`/`UPx`.

## v0.2.1 (2021-12-28)

### Added

- `Px`, `UPx`, and `Lp` now can be multiplied by `Fraction`.
- `Point::rotate_by` and `Point::rotate_around` allow rotating points by an
  `Angle`.

## v0.2.0 (2023-12-18)

This version is a complete rewrite to be primarily integer driven. No attempt at
keeping a compatible API has been made.

## v0.1.2 (2021-09-02)

### Added

- `From<(T,T)>` for `Size`, `Point`, and `Vector`.

## v0.1.1 (2021-08-29)

### Added

- "Partial" constructors for all `Vectorlike` types:
  - `Vector::from_x()`, `Vector::from_y()`
  - `Point::from_x()`, `Point::from_y()`
  - `Size::from_width()`, `Size::from_height()`
- `From<Size>` implementations for `Rect` and `SizedRect`.

## v0.1.0 (2021-08-22)

- Initial release.

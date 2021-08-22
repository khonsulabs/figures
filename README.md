# figures

[![crate version](https://img.shields.io/crates/v/gooey.svg)](https://crates.io/crates/figures)
[![Live Build Status](https://img.shields.io/github/workflow/status/khonsulabs/figures/Tests/main)](https://github.com/khonsulabs/figures/actions?query=workflow:Tests)
[![HTML Coverage Report for `main` branch](https://khonsulabs.github.io/figures/coverage/badge.svg)](https://khonsulabs.github.io/figures/coverage/)
[![Documentation for `main` branch](https://img.shields.io/badge/docs-main-informational)](https://khonsulabs.github.io/figures/main/figures/)

A minimalist crate for 2d math focused on what is needed to build user
interfaces. Written for [`Gooey`][gooey] and [`Kludgine`][kludgine].

`figures` utilizes the Rust type system to prevent performing math on numbers
with different units. The core types in `figures` all accept arbitrary unit
types, allowing you to safely manage numbers in a wide variety of use cases
while allowing the Rust compiler to help you prevent easy mistakes.

While the core types support any arbitrary units, `figures` defines three built in units:

* [`Pixels`](https://khonsulabs.github.io/figures/main/figures/enum.Pixels.html): Represents physical pixels on a display.
* [`Points`](https://khonsulabs.github.io/figures/main/figures/enum.Points.html): Represents a DPI-scaled resolution, as configured in the operating system.
* [`Scaled`](https://khonsulabs.github.io/figures/main/figures/enum.Scaled.html): Represents an arbitrarily scaled resolution, enabling applications to add a "user interface scale" option to their applications easily.

To ease converting between these three units, the [`Displayable`](https://khonsulabs.github.io/figures/main/figures/trait.Displayable.html) trait is implemented by all measurement types in this library.

## Inspiration

This library is born of a difference of opinions from the excellent library
[`euclid`](https://crates.io/crates/euclid). Ultimately, I disagree on some
fundamental design decisions. One of the most prevalent is that in `euclid`
you are prevented from adding a Point and a Size together. In `figures`, you
are able to interoperate with more types freely.

The second major change from `euclid` is embracing the `Length`/[`Figure`]
type throughout the types. APIs will often favor returning a `Figure`
instead of the raw type.

This library is hyper-focused on being a minimalistic math library for
[`Gooey`][gooey] and [`Kludgine`][kludgine]. `euclid` has more functionality.
While we may accept PRs for some additional functionality, if you're looking for
a more complete library, `euclid` should be your choice.

## About

This is being developed by and for [Khonsu Labs](https://khonsulabs.com/) for
[Cosmic Verge](https://github.com/khonsulabs/cosmicverge). I hope it will be
useful to others as well.

This code is dual-licensed under the [MIT License](./LICENSE-MIT) and [Apache
License 2.0](./LICENSE-APACHE). Fonts in this repository are not included by
default, and are [solely licensed under the Apache License
2.0](./fonts/README.md).

[gooey]: https://github.com/khonsulabs/gooey
[kludgine]: https://github.com/khonsulabs/kludgine

# Clutter

_Rust bindings for [Clutter]._

[Clutter]: https://developer.gnome.org/clutter/
[![Build Status](https://travis-ci.org/passcod/rust-clutter.svg?branch=master)](https://travis-ci.org/passcod/rust-clutter)

The bulk of the bindings were original pulled from [clutter.rs], but were then
manually fixed to work with the latest Rust nightly. However, this binding
isn't just that: the full API documentation has been copied into the relevant
places, and altered in some places to fit with the way Rust works (e.g. no NULL
pointers!), and the `cargo doc`-generated files should be all one needs to create
an application using Clutter. The documentation is also [available online][docs].

Due to the source of these bindings, however, there is a portion of the Clutter
API which isn't available. It is functional and useable nonetheless, and Pull
Requests are very welcome!

Stability levels are also taken from the upstream documentation and applied with
`#[stable]`, `#[deprecated]`, etc tags. The binding is set as stable globally,
so reading through the code, one should assume that unmarked methods are stable.

The version number in `Cargo.toml` track the upstream version number. The
version of the bindings can only be tracked with git SHA-1s at the moment.

[clutter.rs]: https://github.com/jensnockert/clutter.rs
[docs]: http://www.rust-ci.org/passcod/rust-clutter/doc/clutter/

## Legal

- Clutter is licensed under the LGPL, and thus these bindings are, too.
- The clutter documentation, and thus the rustdoc comments which contain text
  copied or modified from [the clutter documentation][Clutter], is licensed
  under the GNU Free Documentation License.
- Anything that is not covered by either of the above is released in the
  [Public Domain](https://passcod.name/PUBLIC.txt) unless specified.

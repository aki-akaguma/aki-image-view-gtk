# aki-image-view-gtk

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]

image view gtk gui application.

`aki-image-view-gtk` command is gtk image viewer.

- minimum support gtk v3_24
- minimum support rustc 1.56.1 (59eed8a2a 2021-11-01)

## Command help

```
aki-image-view-gtk --help
```

```
Usage:
  aki-image-view-gtk [options]

image view gtk gui application

Options:
  -c, --config <file>   config file path

  -H, --help        display this help and exit
  -V, --version     display version information and exit
  -X <x-options>    x options. try -X help
```

### Quick install

1. you can install this into cargo bin path:

```
cargo install aki-image-view-gtk
```

2. you can build debian package:

```
cargo deb
```

and install **.deb** into your local repository of debian package.

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/aki-image-view-gtk/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/aki-image-view-gtk.svg
[crate-link]: https://crates.io/crates/aki-image-view-gtk
[docs-image]: https://docs.rs/aki-image-view-gtk/badge.svg
[docs-link]: https://docs.rs/aki-image-view-gtk/
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg

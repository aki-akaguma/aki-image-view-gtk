# aki-image-view-gtk

image view gtk gui application.

`aki-image-view-gtk` command is gtk image viewer.

* minimum support gtk v3_24
* minimum support rustc 1.51.0 (2fd73fabe 2021-03-23)

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

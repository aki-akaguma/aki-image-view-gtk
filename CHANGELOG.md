# Changelog: aki-image-view-gtk

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] *
### Added
* badges into `README.tpl`

### Changed
* reformat `CHANGELOG.md`
* update depends: flood-tide(0.2.8), flood-tide-gen(0.1.19), memx-cdy(0.1.10)
* update depends: anyhow(1.0.68), toml(0.5.10)
* update depends: serde(1.0.152), serde_derive(1.0.152)
* upgrade crates: gtk(0.16)
* update depends: cairo-rs(0.16.7), gdk(0.16.2), gdk-pixbuf(0.16.7)
* update depends: gio(0.16.7), glib(0.16.7)

### Fixed
* clippy: uninlined_format_args


## [0.2.9] (2022-06-25)
### Changed
* update depends: gtk(0.15.5)

## [0.2.8] (2022-06-24)
### Changed
* update depends: gtk(0.14.3)
* changes to edition 2021

## [0.2.7] (2021-09-13)
### Added
* action: `app.reload`
* action: `app.about`

### Changed
* update depends: anyhow(1.0.44), flood-tide(0.2.3), flood-tide-gen(0.1.14), memx-cdy(0.1.6)

## [0.2.6] (2021-09-10)
### Changed
* refactoring source code

## [0.2.5] (2021-09-02)
### Added
* gio action support
* main menu.

### Changed
* the menu ui was refactored.
* buttons of ui to stock buttons.

## [0.2.4] (2021-08-22)
### Changed
* refactoring source code

## [0.2.3] (2021-08-21)
### Changed
* separate ImVm.glade to Dialog.glade

### Fixed
* a render thread loop bug

## [0.2.2] (2021-08-19)
### Added
* a support of a image path into command arguments
* a file chooser dialog

## [0.2.1] (2021-08-16)
### Fixed
* deb's ui/Menu.glade

## [0.2.0] (2021-08-16)
### Changed
* a lot.

## [0.1.0] (2021-07-09)
* first commit

[Unreleased]: https://github.com/aki-akaguma/aki-image-view-gtk/compare/v0.2.9..HEAD
[0.2.9]: https://github.com/aki-akaguma/aki-image-view-gtk/compare/v0.2.8..v0.2.9
[0.2.8]: https://github.com/aki-akaguma/aki-image-view-gtk/compare/v0.2.7..v0.2.8
[0.2.7]: https://github.com/aki-akaguma/aki-image-view-gtk/compare/v0.2.6..v0.2.7
[0.2.6]: https://github.com/aki-akaguma/aki-image-view-gtk/compare/v0.2.5..v0.2.6
[0.2.5]: https://github.com/aki-akaguma/aki-image-view-gtk/compare/v0.2.4..v0.2.5
[0.2.4]: https://github.com/aki-akaguma/aki-image-view-gtk/compare/v0.2.3..v0.2.4
[0.2.3]: https://github.com/aki-akaguma/aki-image-view-gtk/compare/v0.2.2..v0.2.3
[0.2.2]: https://github.com/aki-akaguma/aki-image-view-gtk/compare/v0.2.1..v0.2.2
[0.2.1]: https://github.com/aki-akaguma/aki-image-view-gtk/compare/v0.2.0..v0.2.1
[0.2.0]: https://github.com/aki-akaguma/aki-image-view-gtk/compare/v0.1.0..v0.2.0
[0.1.0]: https://github.com/aki-akaguma/aki-image-view-gtk/releases/tag/v0.1.0

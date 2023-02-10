# flood-tide-gen

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test ubu][test-ubuntu-image]][test-ubuntu-link]
[![Test mac][test-windows-image]][test-windows-link]
[![Test win][test-macos-image]][test-macos-link]

generating the source codes that is used by [flood-tide](https://crates.io/crates/flood-tide).

## Features

- generate tables that is used by [flood-tide](https://crates.io/crates/flood-tide)
- easy to use by xtask
- parsing text define format file that has command options like help text
- minimum support rustc 1.57.0 (f1edd0429 2021-11-29)

## Todos

- [ ] adding test codes
- [ ] other define format support  (like yaml,json)

## Supports

- [flood-tide](https://crates.io/crates/flood-tide) - command option parsing
- [aki-gsub](https://crates.io/crates/aki-gsub) - the sample used *flood-tide*


# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/flood-tide-gen/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/flood-tide-gen.svg
[crate-link]: https://crates.io/crates/flood-tide-gen
[docs-image]: https://docs.rs/flood-tide-gen/badge.svg
[docs-link]: https://docs.rs/flood-tide-gen/
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-ubuntu-image]: https://github.com/aki-akaguma/flood-tide-gen/actions/workflows/test-ubuntu.yml/badge.svg
[test-ubuntu-link]: https://github.com/aki-akaguma/flood-tide-gen/actions/workflows/test-ubuntu.yml
[test-macos-image]: https://github.com/aki-akaguma/flood-tide-gen/actions/workflows/test-macos.yml/badge.svg
[test-macos-link]: https://github.com/aki-akaguma/flood-tide-gen/actions/workflows/test-macos.yml
[test-windows-image]: https://github.com/aki-akaguma/flood-tide-gen/actions/workflows/test-windows.yml/badge.svg
[test-windows-link]: https://github.com/aki-akaguma/flood-tide-gen/actions/workflows/test-windows.yml

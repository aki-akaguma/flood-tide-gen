# Changelog: flood-tide-gen

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.2] - 2026-05-17
### Fixed
- Restore the fallen `CmdOp::to()`

## [0.2.1] - 2026-05-17
### Changed
- Replace the `case` crate with `heck`
- Refactor `OptStr::to_enum` and `OptStr::to_field` to use `heck`
- Refactor `MetaType::as_type_string` to use `heck`
- Rename internal functions and structs to remove the `0` suffix
- Replace `unsafe { std::mem::transmute(value) }` with a safe `match` statement in generated code
- Consolidate multiple type-specific `value_to` functions into a single generic `value_to_type<T>` function in generated code
- Improve parsing robustness by replacing `unreachable!` and `eprintln!` with structured error returns using `anyhow`
- Rust-version: "1.68.0"
- Update crate: regex(1.12)

### Fixed
- `clippy::derivable_impls`

### Removed
- Unnecessary test file `tests/basic.rs`

## [0.2.0] - 2025-09-24
### Added
- `specs`
- More tests
- `gen::MetaType::Path`

## [0.1.22] - 2024-06-09
### Changed
- Refactor test codes

## [0.1.21] - 2023-02-12
### Added
- `.github/workflows/test-ubuntu.yml`
- `.github/workflows/test-macos.yml`
- `.github/workflows/test-windows.yml`
- Test status badges into `README.tpl`
- `MIRIFLAGS=-Zmiri-disable-isolation` on `cargo miri`

### Changed
- Refactor `Makefile`

### Removed
- `COPYING`

### Fixed
- `LICENSE-APACHE`, `LICENSE-MIT`
- Rust-version: "1.57.0" into "1.56.0"

## [0.1.20] - 2023-01-28
### Added
- `.github/workflows/test.yml`
- Test status badges into `README.tpl`

### Fixed
- Makefile: rustc version `1.66.0` to `1.66.1`
- Clippy: `bool_assert_comparison`
- Bug: test on windows, return of `compare_file()` was error
- `LICENSE` files

## [0.1.19] - 2023-01-10
### Added
- Version difference link into `CHANGELOG.md`
- Rust-version = "1.57.0" into Cargo.toml
- `all-test-version` target into Makefile
- Badges into README.tpl

## [0.1.18] - 2023-01-05
### Changed
- Reformat `CHANGELOG.md`
- Update crates: regex(1.7.0)

## [0.1.17] - 2022-08-21
### Fixed
- Gen src: you are deriving `PartialEq` and can implement `Eq`

## [0.1.16] - 2022-06-13
### Changed
- Change to edition 2021

## [0.1.15] - 2021-11-14
### Added
- More documents

### Changed
- Clean source codes

## [0.1.14] - 2021-09-10
### Changed
- Update crates: anyhow(1.0.43)

## [0.1.13] - 2021-05-09
### Changed
- Update depends: regex(1.5.4)

## [0.1.12] - 2021-04-23
### Changed
- Change to nv.opt.lon_or_sho(): argument of OptParseError::xxx() 

## [0.1.11] - 2021-04-22
### Added
- `MetaType::Other(string)` of gen_src_value_to()

## [0.1.10] - 2021-04-18
### Added
- `fn do_gen_src<>()` for more simple

### Changed
- Many deprecated
- Refactor source code

## [0.1.9] - 2021-04-14
### Added
- Test

### Changed
- Separate source code: gen_buffer, gen_src_help, gen_src_match

## [0.1.8] - 2021-04-06
### Added
- Support the single only option: -X <option>

## [0.1.7] - 2021-04-03
### Added
- `cmd_opt_conf_has_subcmd` and `subcmd_opt_conf` into SrcHelpFlags

### Changed
- Change param type: `fn update_file(sss: &str, file_path: &str)`
- Update depends

## [0.1.6] - 2021-03-02
### Added
- `OptStr::is_opt` for supporting Option<T> field.

## [0.1.5] - 2021-02-28
### Added
- `MetaType::Other(String)`

### Changed
- Change `MetaType.as_str()` to `MetaType.as_type_string()`

## [0.1.4] - 2021-02-27
### Added
- Support some "ptions:" line in cmd.txt

## [0.1.3] - 2021-02-21
### Changed
- Rename enum: `CmdOP` to `CmdOp` of output in `gen_src_help()`, it is clippy friends.
- Rename field: `opt_program` to `prog_name` of struct `CmdOptConf` in `gen_src_help()`

## [0.1.2] - 2021-02-07
### Added
- `out_flags: SrcHelpFlags` to `gen_src_help()`.

## [0.1.1] - 2021-02-05
### Added
- Support trait `HelpVersion`

### Changed
- Modify `README.md`

## [0.1.0] - 2021-01-17
### Added
- Initial release

[Unreleased]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.2.2..HEAD
[0.2.2]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.2.1..v0.2.2
[0.2.1]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.2.0..v0.2.1
[0.2.0]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.22..v0.2.0
[0.1.22]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.21..v0.1.22
[0.1.21]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.20..v0.1.21
[0.1.20]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.19..v0.1.20
[0.1.19]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.18..v0.1.19
[0.1.18]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.17..v0.1.18
[0.1.17]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.16..v0.1.17
[0.1.16]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.15..v0.1.16
[0.1.15]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.14..v0.1.15
[0.1.14]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.13..v0.1.14
[0.1.13]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.12..v0.1.13
[0.1.12]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.11..v0.1.12
[0.1.11]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.10..v0.1.11
[0.1.10]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.9..v0.1.10
[0.1.9]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.8..v0.1.9
[0.1.8]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.7..v0.1.8
[0.1.7]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.6..v0.1.7
[0.1.6]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.5..v0.1.6
[0.1.5]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.4..v0.1.5
[0.1.4]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/flood-tide-gen/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/flood-tide-gen/releases/tag/v0.1.0

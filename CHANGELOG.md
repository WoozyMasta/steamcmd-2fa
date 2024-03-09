# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1] - 2023-03-09

### Fixed

- `--code-only` no longer requires a login

## [0.2.0] - 2023-03-09

### Added

- `--code-only` flag for retrieve key only
- default value `+@ShutdownOnFailedCommand 1 +quit` for `--args` flag
- `--before-args` key for positional args like `+force_install_dir`
- environment variables for pass credentials
- one more default linux path for steamcmd `/usr/bin/steamcmd`
- POSIX spliting for quoted args

### Changed

- `--args` flag can split multiple arguments
- build optimization

### Fixed

- `--args` processed normally and allows the steamcmd client to close

[unreleased]: https://github.com/olivierlacan/keep-a-changelog/compare/0.2.0...HEAD
[0.2.1]: https://github.com/WoozyMasta/steamcmd-2fa/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/WoozyMasta/steamcmd-2fa/compare/e2781444666eca8f77d50ca7b63844a51706aef4...0.2.0

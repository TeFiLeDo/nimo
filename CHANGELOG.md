# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
The changelog format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), with
these deviations:

- All releases (except 0.1.0) will also have name.
- All releases will have one paragraph describing the release as a whole.
- Anchor targets are at the end of their release, as opposed to the end of their file.

## [Unreleased]

### Added

- [Started a changelog at `CHANGELOG.md`](https://github.com/TeFiLeDo/nimo/pull/1)
- [The `speed-test` subcommand now automatically retries up to five times if a test results in an error](https://github.com/TeFiLeDo/nimo/pull/4)

### Changed

- [Moved the `completion` subcommand into the `emit` subcommand](https://github.com/TeFiLeDo/nimo/pull/2)
- [Subcommands that don't change the data will now run without write access to the data file](https://github.com/TeFiLeDo/nimo/pull/3)

[unreleased]: https://github.com/TeFiLeDo/nimo/compare/v0.1.0...HEAD

## [0.1.0]: Initial release - 2020-04-22

This is the initial release of this program.

[0.1.0]: https://github.com/TeFiLeDo/nimo/releases/tag/v0.1.0

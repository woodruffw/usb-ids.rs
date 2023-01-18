# Changelog

All notable changes to `usb-ids` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

<!-- @next-header@ -->

## [Unreleased] - ReleaseDate

* The date-style versioning used by this crate is now
  (`v1.YYYY.SEQUENCE`), where `1` indicates the current API version,
  `YYYY` is the current year, and `SEQUENCE` is an increasing sequence number
  for the year. This was changed to prevent versions that would otherwise
  be illegal in the crate ecosystem, like `v1.2023.0117`.

## [1.2022.1224] - 2022-12-25

### Changed

* The date-style versioning used by this crate is now
  (`v1.YYYY.MMDD`), where `1` indicates the current API version
  and `YYYY.MMDD` is the date of release.

## [2022.12.24] - 2022-12-25

**YANKED**.

### Changed

* This crate now uses date-style versioning (`vYYYY.MM.DD`)
  and is considered stable. This release contains the usb.ids
  database as of `2022-12-15 20:34:08`.

## [0.2.5] - 2022-11-29

## [0.2.4] - 2022-09-09

## [0.2.3] - 2022-09-09

## [0.2.2] - 2022-03-29

### Changed

* DB: Updates to the USB DB (+146, -15)

## [0.2.1] - 2021-07-03

## [0.2.0] - 2021-06-24

## [0.1.0] - 2021-06-24

## [0.0.3] - 2021-01-31

### Added

* Routine DB update (2021-01-29 20:34:11)

## [0.0.2] - 2020-12-26

### Added

* Added `Device::from_vid_pid`.

## [0.0.1] - 2020-12-26

### Added

* This is the initial release of `usb-ids`.

<!-- @next-url@ -->
[Unreleased]: https://github.com/woodruffw/kbs2/compare/v1.2022.1224...HEAD
[1.2022.1224]: https://github.com/woodruffw/kbs2/compare/v2022.12.24...v1.2022.1224
[2022.12.24]: https://github.com/woodruffw/kbs2/compare/v0.2.5...v2022.12.24
[0.2.5]: https://github.com/woodruffw/kbs2/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/woodruffw/kbs2/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/woodruffw/kbs2/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/woodruffw/usb-ids.rs/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/woodruffw/usb-ids.rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/woodruffw/usb-ids.rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/woodruffw/usb-ids.rs/compare/v0.0.3...v0.1.0
[0.0.3]: https://github.com/woodruffw/usb-ids.rs/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/woodruffw/usb-ids.rs/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/woodruffw/usb-ids.rs/releases/tag/v0.0.1

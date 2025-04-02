# Changelog

All notable changes to `usb-ids` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

<!-- @next-header@ -->

## [Unreleased] - ReleaseDate

## [1.2025.2] - 2025-04-02

## [1.2025.1] - 2025-01-15

## [1.2024.5] - 2024-12-09

## [1.2024.4] - 2024-07-08

## [1.2024.3] - 2024-04-26

## [1.2024.2] - 2024-01-31

## [1.2024.1] - 2024-01-21

## [1.2023.7] - 2023-11-19

* Support for additional identities in the USB ID repository has been
  expanded significantly; see the docs for additional new APIs.
  [#50](https://github.com/woodruffw/usb-ids.rs/pull/50)

## [1.2023.6] - 2023-10-18

## [1.2023.5] - 2023-08-25

## [1.2023.4] - 2023-08-19

## [1.2023.3] - 2023-08-05

## [1.2023.2] - 2023-04-30

## [1.2023.1] - 2023-04-24

## [1.2023.0] - 2023-01-18

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
[Unreleased]: https://github.com/woodruffw/usb-ids.rs/compare/v1.2025.2...HEAD
[1.2025.2]: https://github.com/woodruffw/usb-ids.rs/compare/v1.2025.1...v1.2025.2
[1.2025.1]: https://github.com/woodruffw/usb-ids.rs/compare/v1.2024.5...v1.2025.1
[1.2024.5]: https://github.com/woodruffw/usb-ids.rs/compare/v1.2024.4...v1.2024.5
[1.2024.4]: https://github.com/woodruffw/usb-ids.rs/compare/v1.2024.3...v1.2024.4
[1.2024.3]: https://github.com/woodruffw/usb-ids.rs/compare/v1.2024.2...v1.2024.3
[1.2024.2]: https://github.com/woodruffw/usb-ids.rs/compare/v1.2024.1...v1.2024.2
[1.2024.1]: https://github.com/woodruffw/usb-ids.rs/compare/v1.2023.7...v1.2024.1
[1.2023.7]: https://github.com/woodruffw/usb-ids.rs/compare/v1.2023.6...v1.2023.7
[1.2023.6]: https://github.com/woodruffw/usb-ids.rs/compare/v1.2023.5...v1.2023.6
[1.2023.5]: https://github.com/woodruffw/usb-ids.rs/compare/v1.2023.4...v1.2023.5
[1.2023.4]: https://github.com/woodruffw/usb-ids.rs/compare/v1.2023.3...v1.2023.4
[1.2023.3]: https://github.com/woodruffw/usb-ids.rs/compare/v1.2023.2...v1.2023.3
[1.2023.2]: https://github.com/woodruffw/usb-ids.rs/compare/v1.2023.1...v1.2023.2
[1.2023.1]: https://github.com/woodruffw/usb-ids.rs/compare/v1.2023.0...v1.2023.1
[1.2023.0]: https://github.com/woodruffw/usb-ids.rs/compare/v1.2022.1224...v1.2023.0
[1.2022.1224]: https://github.com/woodruffw/usb-ids.rs/compare/v2022.12.24...v1.2022.1224
[2022.12.24]: https://github.com/woodruffw/usb-ids.rs/compare/v0.2.5...v2022.12.24
[0.2.5]: https://github.com/woodruffw/usb-ids.rs/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/woodruffw/usb-ids.rs/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/woodruffw/usb-ids.rs/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/woodruffw/usb-ids.rs/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/woodruffw/usb-ids.rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/woodruffw/usb-ids.rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/woodruffw/usb-ids.rs/compare/v0.0.3...v0.1.0
[0.0.3]: https://github.com/woodruffw/usb-ids.rs/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/woodruffw/usb-ids.rs/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/woodruffw/usb-ids.rs/releases/tag/v0.0.1

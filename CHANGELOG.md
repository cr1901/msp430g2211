# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]
This crate was regenerated three times to take advantage of svd2rust commit 26baf2c,
svd2rust 0.18.0, and then svd2rust 0.20.0. Non-breaking changes include:

- The newer versions of msp430_svd did not cause significant changes to this
  crate (some fields of the SVD were moved around).

Breaking changes include:

- New versions of svd2rust caused backwards-incompatible changes.
  See commits 287e48a, 68e45a9. 0.20.0 breakage is likely minimal.
- Update msp430 and msp430-rt to version 0.3.0 and bare-metal to 1.0.0.

## [v0.2.1] - 2021-04-16
- Put msp430_svd version information into the SVD file using
  `vendorExtensions`.

- Leverage the work of `svdtools` to fill in missing register fields.
  These _appear_ to all be backward-compatible changes.

## [v0.2.0] - 2020-01-15
- Regenerate crate with svd2rust v0.17.0 plus msp-fix commits.
  - Remove empty enumerations which violate SVD spec.

- Update msp430 and msp430-rt to version 0.2.0.

## [v0.1.4] - 2018-06-18
- Updated msp430-rt to version 0.1.3.

## [v0.1.3] - 2018-06-18
Version `v0.1.2` was never released due to an oversight.

- Updated msp430-rt to version 0.1.2.

- Fix for macro_reexport feature.

## [v0.1.1] - 2018-02-02
- Updated msp430-rt to version 0.1.1.

## v0.1.0 - 2017-07-24

Initial release

[Unreleased]: https://github.com/cr1901/msp430g2211/compare/v0.2.1...HEAD
[v0.2.1]: https://github.com/cr1901/msp430g2211/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/cr1901/msp430g2211/compare/v0.1.4...v0.2.0
[v0.1.4]: https://github.com/cr1901/msp430g2211/compare/v0.1.3...v0.1.4
[v0.1.3]: https://github.com/cr1901/msp430g2211/compare/v0.1.1...v0.1.3
[v0.1.1]: https://github.com/cr1901/msp430g2211/compare/v0.1.0...v0.1.1

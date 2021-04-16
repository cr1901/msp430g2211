# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

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

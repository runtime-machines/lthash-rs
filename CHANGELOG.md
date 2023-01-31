# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 31-01-2023

### Added

- Provide generic version of `LtHash32`.
- Other operations for both versions:
  - union function (also via `|` operator);
  - difference function (also via `-` operator);
  - checksum as hex string.
  
## [0.0.1] - 25-01-2023

### Added

- Basic version of `LtHash16`:
  - `LtHash` trait;
  - sequential `insert` and `remove` operation;
  - support any hash function implementing `digest::ExtendableOutput` and `Default`.

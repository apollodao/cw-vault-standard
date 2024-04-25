# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Changes type of `VaultStandardInfoResponse::version` from `u16` to `String`
- Add `base_token` and `vault_token` fields to `VaultContract` helper struct.
- Remove faulty usage of `to_binary` in `VaultContract` query helper functions.
- Replace usage of deprecated `to_binary` with `to_json_binary`.

## [0.3.3] - 2023-09-27

### Added

- Export `const VERSION`.
- Adds helper module with helper structs `VaultContract` and `VaultContractUnchecked`.

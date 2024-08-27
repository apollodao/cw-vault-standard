# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.1-rc.2] - 2024-08-27

### Added

- Added `base_token` and `vault_token` fields to `VaultContract` helper struct.
- Added `VaultStandardQueryMsg::VaultTokenExchangeRate` query message.

### Changed

- Changes type of `VaultStandardInfoResponse::version` from `u16` to `String`
- Remove faulty usage of `to_binary` in `VaultContract` query helper functions.
- Replace usage of deprecated `to_binary` with `to_json_binary`.
- Removes `VaultStandardQueryMsg::PreviewDeposit` and `VaultStandardQueryMsg::PreviewRedeem`.
  - These queries turned out to be too difficult to implement in most cases. We recommend to use transaction simulation from non-contract clients such as frontends. These queries will be removed in the next major version.
- Removes CW4626 Extension now that TokenFactory module is standard across all major Cosmos chains. The extension will be removed in the next major version.
- Removes `amount` fields in `VaultStandardExecuteMsg::Deposit` and `VaultStandardExecuteMsg::Redeem` as well as in `LockupExecuteMsg::Unlock` and `ForceUnlockExecuteMsg::ForceRedeem`. These fields will be removed in a future version and the amount from the sent funds will be used instead.

## [0.3.3] - 2023-09-27

### Added

- Export `const VERSION`.
- Adds helper module with helper structs `VaultContract` and `VaultContractUnchecked`.

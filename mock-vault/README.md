# Mock Vault

This is a mock implementation of a vault contract that adheres to the [CosmWasm Vault Standard](../cw-vault-standard/README.md).

This is useful for testing and for reference. This vault simply increases the ratio of `base_token` to `vault_token` when base tokens are donated to it via a normal BankMsg transfer rather than calling the deposit function.

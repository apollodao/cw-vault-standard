use cosmwasm_std::Uint128;
pub use cw_storage_plus::Item;

use crate::msg::Config;

pub const CONFIG: Item<Config> = Item::new("config");

pub const VAULT_TOKEN_SUPPLY: Item<Uint128> = Item::new("vault_token_supply");

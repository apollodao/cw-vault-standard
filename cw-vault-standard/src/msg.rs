#[cfg(feature = "force-unlock")]
use crate::extensions::force_unlock::ForceUnlockExecuteMsg;
#[cfg(feature = "keeper")]
use crate::extensions::keeper::{KeeperExecuteMsg, KeeperQueryMsg};
#[cfg(feature = "lockup")]
use crate::extensions::lockup::{LockupExecuteMsg, LockupQueryMsg};

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_json_binary, Coin, CosmosMsg, Empty, StdResult, Uint128, WasmMsg};
use schemars::JsonSchema;

/// The default ExecuteMsg variants that all vaults must implement.
/// This enum can be extended with additional variants by defining an extension
/// enum and then passing it as the generic argument `T` to this enum.
#[cw_serde]
pub enum VaultStandardExecuteMsg<T = ExtensionExecuteMsg> {
    /// Called to deposit into the vault. Native assets are passed in the funds
    /// parameter.
    Deposit {
        /// The optional recipient of the vault token. If not set, the caller
        /// address will be used instead.
        recipient: Option<String>,
    },

    /// Called to redeem vault tokens and receive assets back from the vault.
    /// The native vault token must be passed in the funds parameter, unless the
    /// lockup extension is called, in which case the vault token has already
    /// been passed to ExecuteMsg::Unlock.
    Redeem {
        /// An optional field containing which address should receive the
        /// withdrawn assets. If not set, the caller address will be
        /// used instead.
        recipient: Option<String>,
        /// The denoms of the assets to be withdrawn. If not set, the return
        /// asset or assets will be determined by the vault. Note that
        /// the vault may not support all assets, and may return an
        /// error if the requested assets are not supported.
        redeem_into: Option<Vec<String>>,
    },

    /// Called to execute functionality of any enabled extensions.
    VaultExtension(T),
}

impl VaultStandardExecuteMsg {
    /// Convert a [`VaultStandardExecuteMsg`] into a [`CosmosMsg`].
    pub fn into_cosmos_msg(self, contract_addr: String, funds: Vec<Coin>) -> StdResult<CosmosMsg> {
        Ok(WasmMsg::Execute {
            contract_addr,
            msg: to_json_binary(&self)?,
            funds,
        }
        .into())
    }
}

/// Contains ExecuteMsgs of all enabled extensions. To enable extensions defined
/// outside of this crate, you can define your own `ExtensionExecuteMsg` type
/// in your contract crate and pass it in as the generic parameter to ExecuteMsg
#[cw_serde]
pub enum ExtensionExecuteMsg {
    #[cfg(feature = "keeper")]
    Keeper(KeeperExecuteMsg),
    #[cfg(feature = "lockup")]
    Lockup(LockupExecuteMsg),
    #[cfg(feature = "force-unlock")]
    ForceUnlock(ForceUnlockExecuteMsg),
}

/// The default QueryMsg variants that all vaults must implement.
/// This enum can be extended with additional variants by defining an extension
/// enum and then passing it as the generic argument `T` to this enum.
#[cw_serde]
#[derive(QueryResponses)]
pub enum VaultStandardQueryMsg<T = ExtensionQueryMsg>
where
    T: JsonSchema,
{
    /// Returns `VaultStandardInfoResponse` with information on the version of
    /// the vault standard used as well as any enabled extensions.
    #[returns(VaultStandardInfoResponse)]
    VaultStandardInfo {},

    /// Returns `VaultInfoResponse` representing vault requirements, lockup, &
    /// vault token denom.
    #[returns(VaultInfoResponse)]
    Info {},

    /// Returns the amount of assets managed by the vault as a `Vec<Coin>`.
    /// Useful for display purposes, such as calculating vault TVL.
    #[returns(Vec<Coin>)]
    TotalAssets {},

    /// Returns `Uint128` total amount of vault tokens in circulation.
    #[returns(Uint128)]
    TotalVaultTokenSupply {},

    /// Returns the exchange rate of vault tokens quoted in terms of the
    /// supplied quote_denom. Returns a `Decimal` containing the amount of
    /// `quote_denom` assets that can be exchanged for 1 unit of vault
    /// tokens.
    ///
    /// May return an error if the quote denom is not supported by the vault.
    #[returns(cosmwasm_std::Decimal)]
    VaultTokenExchangeRate {
        /// The quote denom to quote the exchange rate in.
        quote_denom: String,
    },

    /// The amount of vault tokens that the vault would exchange for the amount
    /// of assets provided, in an ideal scenario where all the conditions
    /// are met.
    ///
    /// Useful for display purposes and does not have to confer the exact amount
    /// of vault tokens returned by the vault if the passed in assets were
    /// deposited. This calculation should not reflect the "per-user"
    /// price-per-share, and instead should reflect the "average-user’s"
    /// price-per-share, meaning what the average user should expect to see
    /// when exchanging to and from.
    #[returns(Uint128)]
    ConvertToShares {
        /// The assets to convert to vault tokens.
        assets: Vec<Coin>,
    },

    /// Returns the amount of assets that the vault would exchange for
    /// the `amount` of vault tokens provided, in an ideal scenario where all
    /// the conditions are met.
    ///
    /// Useful for display purposes and does not have to confer the exact amount
    /// of assets returned by the vault if the passed in vault tokens were
    /// redeemed. This calculation should not reflect the "per-user"
    /// price-per-share, and instead should reflect the "average-user’s"
    /// price-per-share, meaning what the average user should expect to see
    /// when exchanging to and from.
    #[returns(Vec<Coin>)]
    ConvertToAssets {
        /// The amount of vault tokens to convert to assets.
        amount: Uint128,
        /// The assets to convert the specified amount of vault tokens to.
        assets: Vec<String>,
    },

    /// Handle queries of any enabled extensions.
    #[returns(Empty)]
    VaultExtension(T),
}

/// Contains QueryMsgs of all enabled extensions. To enable extensions defined
/// outside of this crate, you can define your own `ExtensionQueryMsg` type
/// in your contract crate and pass it in as the generic parameter to QueryMsg
#[cw_serde]
pub enum ExtensionQueryMsg {
    #[cfg(feature = "keeper")]
    Keeper(KeeperQueryMsg),
    #[cfg(feature = "lockup")]
    Lockup(LockupQueryMsg),
}

/// Struct returned from QueryMsg::VaultStandardInfo with information about the
/// used version of the vault standard and any extensions used.
///
/// This struct should be stored as an Item under the `vault_standard_info` key,
/// so that other contracts can do a RawQuery and read it directly from storage
/// instead of needing to do a costly SmartQuery.
#[cw_serde]
pub struct VaultStandardInfoResponse {
    /// The version of the vault standard used by the vault as a semver
    /// compliant string. E.g. "1.0.0" or "1.2.3-alpha.1"
    pub version: String,
    /// A list of vault standard extensions used by the vault.
    /// E.g. ["lockup", "keeper"]
    pub extensions: Vec<String>,
}

/// Returned by QueryMsg::Info and contains information about this vault
#[cw_serde]
pub struct VaultInfoResponse {
    /// The token that is used for accounting in the vault. This should be the
    /// denom if it is a native token. It is optional as some vaults do not
    /// account in a specific token and instead may use some kind of virtual
    /// shares or other type of accounting.
    pub base_token: Option<String>,
    /// The denom of the vault token.
    pub vault_token: String,
}

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
        /// The amount of base tokens to deposit.
        #[deprecated(
            since = "0.4.1",
            note = "This field will be removed in the next version. The amount \
            of deposited assets should instead be read from the actual sent funds."
        )]
        amount: Uint128,
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
        /// withdrawn base tokens. If not set, the caller address will be
        /// used instead.
        recipient: Option<String>,
        /// The amount of vault tokens sent to the contract. In the case that
        /// the vault token is a Cosmos native denom, we of course have this
        /// information in info.funds, but if the vault implements the
        /// Cw4626 API, then we need this argument. We figured it's
        /// better to have one API for both types of vaults, so we
        /// require this argument.
        #[deprecated(
            since = "0.4.1",
            note = "This field will be removed in the next version. The amount \
            of vault tokens should instead be read from the actual amount of sent vault tokens."
        )]
        amount: Uint128,
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

    /// Returns `Uint128` amount of vault tokens that will be returned for the
    /// passed in `amount` of base tokens.
    ///
    /// Allows an on-chain or off-chain user to simulate the effects of their
    /// deposit at the current block, given current on-chain conditions.
    ///
    /// Must return as close to and no more than the exact amount of vault
    /// tokens that would be minted in a deposit call in the same transaction.
    /// I.e. Deposit should return the same or more vault tokens as
    /// PreviewDeposit if called in the same transaction.
    #[deprecated(
        since = "0.4.1",
        note = "PreviewDeposit and PreviewRedeem turned out to be too difficult to implement in most cases. We recommend to use transaction simulation from non-contract clients such as frontends."
    )]
    #[returns(Uint128)]
    PreviewDeposit {
        /// The amount of base tokens to preview depositing.
        amount: Uint128,
    },

    /// Returns `Uint128` amount of base tokens that would be withdrawn in
    /// exchange for redeeming `amount` of vault tokens.
    ///
    /// Allows an on-chain or off-chain user to simulate the effects of their
    /// redeem at the current block, given current on-chain conditions.
    ///
    /// Must return as close to and no more than the exact amount of base tokens
    /// that would be withdrawn in a redeem call in the same transaction.
    #[deprecated(
        since = "0.4.1",
        note = "PreviewDeposit and PreviewRedeem turned out to be too difficult to implement in most cases. We recommend to use transaction simulation from non-contract clients such as frontends."
    )]
    #[returns(Uint128)]
    PreviewRedeem {
        /// The amount of vault tokens to preview redeeming.
        amount: Uint128,
    },

    /// Returns the amount of assets managed by the vault denominated in base
    /// tokens. Useful for display purposes, and does not have to confer the
    /// exact amount of base tokens.
    #[returns(Uint128)]
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
        /// The amount of base tokens to convert to vault tokens.
        amount: Uint128,
    },

    /// Returns the amount of base tokens that the Vault would exchange for
    /// the `amount` of vault tokens provided, in an ideal scenario where all
    /// the conditions are met.
    ///
    /// Useful for display purposes and does not have to confer the exact amount
    /// of assets returned by the vault if the passed in vault tokens were
    /// redeemed. This calculation should not reflect the "per-user"
    /// price-per-share, and instead should reflect the "average-user’s"
    /// price-per-share, meaning what the average user should expect to see
    /// when exchanging to and from.
    #[returns(Uint128)]
    ConvertToAssets {
        /// The amount of vault tokens to convert to base tokens.
        amount: Uint128,
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
    /// The token that is accepted for deposits, withdrawals and used for
    /// accounting in the vault. The denom if it is a native token and the
    /// contract address if it is a cw20 token.
    pub base_token: String,
    /// Vault token. The denom if it is a native token and the contract address
    /// if it is a cw20 token.
    pub vault_token: String,
}

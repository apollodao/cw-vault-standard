use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "cw20s")] {
use cw_asset::{Asset, AssetInfo};
use std::convert::TryFrom;
use cw20::Cw20Coin;
    }
}

#[cfg(feature = "lockup")]
use crate::extensions::lockup::{LockupExecuteMsg, LockupQueryMsg};

#[cfg(feature = "keeper")]
use crate::extensions::keeper::{KeeperExecuteMsg, KeeperQueryMsg};

use cosmwasm_std::{Binary, Decimal, StdError, StdResult, Uint128};
use cosmwasm_std::{Coin, Empty};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg<T = ExtensionExecuteMsg, S = Empty> {
    /// Called to deposit into the vault. Native assets are passed in the funds
    /// parameter.
    Deposit {
        /// With the cw20 feature, it is allowed to deposit CW20 tokens. These
        /// must be passed in with the cw20_assets and have allowance pre-approved.
        #[cfg(feature = "cw20s")]
        cw20_assets: Option<Vec<Cw20Coin>>,
        /// The optional receiver of the vault token. If not set, the caller
        /// address will be used instead.
        receiver: Option<String>,
    },

    /// Called to withdraw from the vault. The native vault token must be passed
    /// in the funds parameter, unless the lockup extension is called, in which
    /// case the vault token has already been passed to ExecuteMsg::Unlock
    Withdraw {
        /// An optional field containing which address should receive the
        /// withdrawn underlying assets.
        receiver: Option<String>,
        // An optional field containing a binary encoded CosmosMsg. If set, the
        // vault will return the underlying assets to receiver and assume that
        // receiver is a contract and try to execute the binary encoded
        // ExecuteMsg on the contract.
        //
        // TODO: Keep this? Figure out best Receiver API.
        // contract_msg: Option<Binary>,
    },

    /// Custom callback functions defined by the vault.
    Callback(S),

    /// Support for custom extensions
    VaultExtension(T),
}

/// Contains ExecuteMsgs of all enabled extensions. To enable extensions defined
/// outside of this create, you can define your own `ExtensionExecuteMsg` type
/// in your contract crate and pass it in as the generic parameter to ExecuteMsg
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionExecuteMsg {
    #[cfg(feature = "keeper")]
    Keeper(KeeperExecuteMsg),
    #[cfg(feature = "lockup")]
    Lockup(LockupExecuteMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg<T = ExtensionQueryMsg> {
    /// Returns `VaultStandardInfo` with information on the version of the vault
    /// standard used as well as any enabled extensions.
    VaultStandardInfo,

    /// Returns `VaultInfo` representing vault requirements, lockup, & vault
    /// token denom.
    Info,

    /// Returns `Uint128` amount of vault tokens that will be returned for the
    /// passed in assets.
    ///
    /// Allows an on-chain or off-chain user to simulate the effects of their
    /// deposit at the current block, given current on-chain conditions.
    ///
    /// MUST return as close to and no more than the exact amount of Vault
    /// shares that would be minted in a deposit call in the same transaction.
    /// I.e. deposit should return the same or more shares as previewDeposit if
    /// called in the same transaction.
    ///
    /// MUST NOT account for deposit limits like those returned from maxDeposit
    /// and should always act as though the deposit would be accepted,
    /// regardless if the user has enough tokens approved, etc.
    ///
    /// MUST be inclusive of deposit fees. Integrators should be aware of the
    /// existence of deposit fees.
    PreviewDeposit {
        coins: Vec<Coin>,
        #[cfg(feature = "cw20s")]
        cw20s: Vec<Cw20Coin>,
    },

    /// Returns `Uint128` vault tokens needed to withdraw the passed in assets.
    ///
    /// TODO: Keep this? See discussion above. If removed PreviewRedeem could be
    /// renamed to PreviewWithdraw.
    PreviewWithdraw {
        coins: Vec<Coin>,
        #[cfg(feature = "cw20ss")]
        cw20s: Vec<Cw20Coin>,
    },

    /// Returns `AssetsResponse` assets needed to mint `shares` number of
    /// vault tokens.
    ///
    /// TODO: Keep this? Could maybe be useful, but we have no Mint ExecuteMsg
    /// since we are using native vault tokens. If callbacks are added to TokenFactory and
    /// the MintMsg on TokenFactory is openened up, anyone could potentially send a MsgMint on
    /// to the TokenFactory, but how would the assets be passed to the contract? Could they
    /// be passed to the TokenFactory and be forwarded to the vault's callback?
    PreviewMint {
        shares: Uint128,
    },

    /// Returns `AssetsResponse` representing all the assets that would be redeemed for in exchange for
    /// vault tokens. Used by Rover to calculate vault position values.
    PreviewRedeem {
        shares: Uint128,
    },

    /// Returns `Option<AssetsResponse>` maximum amount of assets that can be
    /// deposited into the Vault for the `receiver`, through a call to Deposit.
    ///
    /// MUST return the maximum amount of assets deposit would allow to be
    /// deposited for `receiver` and not cause a revert, which MUST NOT be higher
    /// than the actual maximum that would be accepted (it should underestimate
    /// if necessary). This assumes that the user has infinite assets, i.e.
    /// MUST NOT rely on the asset balances of `receiver`.
    ///
    /// MUST factor in both global and user-specific limits, like if deposits
    /// are entirely disabled (even temporarily) it MUST return 0.
    MaxDeposit {
        receiver: String,
    },

    /// Returns `Option<Uint128>` maximum amount of vault shares that can be minted upon
    /// a Deposit call.
    ///
    /// TODO: Keep this? We don't have a Mint function. Could be combined with
    /// MaxDeposit to return a struct containing both.
    MaxMint {
        receiver: String,
    },

    /// Returns `Option<AssetsResponse>` maximum amount of assets that can be
    /// withdrawn from the owner balance in the Vault, through a withdraw call.
    ///
    /// MUST return the maximum amount of assets that could be transferred from
    /// owner through withdraw and not cause a revert, which MUST NOT be higher
    /// than the actual maximum that would be accepted (it should underestimate
    /// if necessary). This assumes that the user has infinite vault shares, i.e.
    /// MUST NOT rely on the vault token balances of `owner`.
    ///
    /// MUST factor in both global and user-specific limits, like if withdrawals
    /// are entirely disabled (even temporarily) it MUST return 0.
    MaxWithdraw {
        owner: String,
    },

    /// Returns `Option<Uint128>` maximum amount of Vault shares that can be redeemed
    /// from the owner balance in the Vault, through a call to Withdraw
    ///
    /// TODO: Keep this? Could potentially be combined with MaxWithdraw to return
    /// a MaxWithdrawResponse type that includes both max assets that can be
    /// withdrawn as well as max vault shares that can be withdrawn in exchange
    /// for assets.
    MaxRedeem {
        owner: String,
    },

    /// Returns `AssetsResponse` assets managed by vault.
    /// Useful for display purposes, and does not have to confer the exact
    /// amount of underlying assets.
    TotalAssets,

    /// The amount of shares that the vault would exchange for the amount of
    /// assets provided, in an ideal scenario where all the conditions are met.
    ///
    /// Useful for display purposes and does not have to confer the exact amount
    /// of shares returned by the vault if the passed in assets were deposited.
    /// This calculation may not reflect the “per-user” price-per-share, and
    /// instead should reflect the “average-user’s” price-per-share, meaning
    /// what the average user should expect to see when exchanging to and from.
    ConvertToShares {
        coins: Vec<Coin>,
        #[cfg(feature = "cw20s")]
        cw20s: Vec<Cw20Coin>,
    },

    /// Returns `AssetsResponse` assets that the Vault would exchange for
    /// the amount of shares provided, in an ideal scenario where all the
    /// conditions are met.
    ///
    /// Useful for display purposes and does not have to confer the exact amount
    /// of assets returned by the vault if the passed in shares were withdrawn.
    /// This calculation may not reflect the “per-user” price-per-share, and
    /// instead should reflect the “average-user’s” price-per-share, meaning
    /// what the average user should expect to see when exchanging to and from.
    ConvertToAssets {
        shares: Uint128,
    },

    VaultExtension(T),
}

/// Contains QueryMsgs of all enabled extensions. To enable extensions defined
/// outside of this create, you can define your own `ExtensionQueryMsg` type
/// in your contract crate and pass it in as the generic parameter to QueryMsg
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VaultStandardInfo {
    /// The version of the vault standard used. A number, e.g. 1, 2, etc.
    pub version: u16,
    /// A list of vault standard extensions used by the vault.
    /// E.g. ["cw20", "lockup", "keeper"]
    pub extensions: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AssetsResponse {
    pub coins: Vec<Coin>,
    #[cfg(feature = "cw20s")]
    pub cw20s: Vec<Cw20Coin>,
}

#[cfg(feature = "cw20s")]
impl TryFrom<Vec<Asset>> for AssetsResponse {
    type Error = StdError;

    fn try_from(assets: Vec<Asset>) -> StdResult<Self> {
        let mut coins = vec![];
        let mut cw20s = vec![];

        for asset in assets {
            match &asset.info {
                AssetInfo::Native(token) => coins.push(Coin {
                    denom: token.to_string(),
                    amount: asset.amount,
                }),
                AssetInfo::Cw20(addr) => cw20s.push(Cw20Coin {
                    address: addr.to_string(),
                    amount: asset.amount,
                }),
                _ => return Err(StdError::generic_err("unsupported asset type")),
            }
        }

        Ok(AssetsResponse { coins, cw20s })
    }
}

/// Returned by QueryMsg::Info and contains information about this vault
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VaultInfo {
    /// Coins required to enter vault.
    /// Amount will be proportional to the share of which it should occupy in the group
    /// (e.g. { denom: osmo, amount: 1 }, { denom: atom, amount: 1 } indicate a 50-50 split)
    pub deposit_coins: Vec<Coin>,
    #[cfg(feature = "cw20s")]
    pub deposit_cw20s: Vec<Cw20Coin>,
    /// Denom of vault token
    pub vault_token_denom: String,
}

pub struct VaultReceiveMsg {
    pub sender: String,
    pub amount: Uint128,
    pub msg: Binary,
}

/// Zapper that does not need to know Vault API
pub enum GeneralizedZapperExecuteMsg {
    Zap {
        /// If cw20 assets are sent, they must be listed here and have pre-approved
        /// allowance set.
        assets: Option<Vec<Coin>>,
        /// The asset the caller wishes to receive
        receive_asset: String,
        /// The recipient of the converted assets
        recipient: String,
        /// If set will try to call the binary encoded ExecuteMsg on recipient
        contract_msg: Option<Binary>,
        /// The slippage tolerance to use when converting. The zapper will use
        /// some internal oracle to value the input and output assets.
        slippage_tolerance: Option<Decimal>,
    },
}

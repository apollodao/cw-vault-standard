use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_json_binary, Coin, CosmosMsg, StdResult, WasmMsg};

use crate::{ExtensionExecuteMsg, VaultStandardExecuteMsg};

/// Additional ExecuteMsg variants for vaults that enable the ForceUnlock
/// extension.
#[cw_serde]
pub enum ForceUnlockExecuteMsg {
    /// Can be called by whitelisted addresses to bypass the lockup and
    /// immediately return the assets. Used in the event of
    /// liquidation. The caller must pass the native vault tokens in the funds
    /// field.
    ForceRedeem {
        /// The address which should receive the withdrawn assets. If not set,
        /// the caller address will be used instead.
        recipient: Option<String>,
        /// The denoms of the assets to be withdrawn. If not set, the return
        /// asset or assets will be determined by the vault. Note that
        /// the vault may not support all assets, and may return an
        /// error if the requested assets are not supported.
        redeem_into: Option<Vec<String>>,
    },

    /// Force withdraw from a position that is already unlocking (Unlock has
    /// already been called).
    ForceWithdrawUnlocking {
        /// The ID of the unlocking position from which to force withdraw
        lockup_id: u64,
        /// Optional amount of assets to be force withdrawn.
        /// If None is passed, the entire position will be force withdrawn.
        amounts: Option<Vec<Coin>>,
        /// The address which should receive the withdrawn assets. If not set,
        /// the assets will be sent to the caller.
        recipient: Option<String>,
    },

    /// Update the whitelist of addresses that can call ForceRedeem and
    /// ForceWithdrawUnlocking.
    UpdateForceWithdrawWhitelist {
        /// Addresses to add to the whitelist.
        add_addresses: Vec<String>,
        /// Addresses to remove from the whitelist.
        remove_addresses: Vec<String>,
    },
}

impl ForceUnlockExecuteMsg {
    /// Convert a [`ForceUnlockExecuteMsg`] into a [`CosmosMsg`].
    pub fn into_cosmos_msg(self, contract_addr: String, funds: Vec<Coin>) -> StdResult<CosmosMsg> {
        Ok(WasmMsg::Execute {
            contract_addr,
            msg: to_json_binary(&VaultStandardExecuteMsg::VaultExtension(
                ExtensionExecuteMsg::ForceUnlock(self),
            ))?,
            funds,
        }
        .into())
    }
}

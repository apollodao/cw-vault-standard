use cosmwasm_std::{Addr, Binary, Uint128};
use cw_utils::Expiration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LockupExecuteMsg {
    /// Withdraw an unlocking position that has finished unlocking.
    WithdrawUnlocked {
        /// An optional field containing which address should receive the
        /// withdrawn underlying assets.
        receiver: Option<String>,
        /// An optional field containing a binary encoded CosmosMsg. If set, the
        /// vault will return the underlying assets to receiver and assume that
        /// receiver is a contract and try to execute the binary encoded
        /// ExecuteMsg on the contract.
        contract_msg: Option<Binary>,
        /// The ID of the expired lockup to withdraw from.
        /// If None is passed, the vault will attempt to withdraw all expired
        /// lockup positions. Note that this can fail if there are too many
        /// lockup positions and the `max_contract_gas` limit is hit.
        lockup_id: Option<u64>,
    },

    /// Unlock is called to initiate unlocking a locked position held by the
    /// vault.
    /// The caller must pass the native vault tokens in the funds parameter.
    /// Emits an Unlock event with `amount` attribute containing an u64 lockup_id.
    /// Also encodes the u64 lockup ID as binary and returns it in the Response's
    /// data field, so that it can be read by SubMsg replies.
    Unlock,

    /// Can be called by whitelisted addresses to bypass the lockup and
    /// immediately return the underlying assets. Used in the event of
    /// liquidation.
    /// TBD: how will this be whitelisted?
    ForceWithdraw { receiver: Option<String> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LockupQueryMsg {
    /// Returns a `Vec<Lockup>` containing all the currently unclaimed lockup
    /// positions for the `owner`.
    Lockups {
        /// The address of the owner of the lockup
        owner: String,
        /// Return results only after this lockup_id
        start_after: Option<u64>,
        /// Max amount of results to return
        limit: Option<u32>,
    },

    /// Returns `Lockup` info about a specific lockup, by owner and ID.
    Lockup { owner: String, lockup_id: u64 },

    /// Returns `u64` duration of the lockup.
    LockupDuration,
}

/// Info about a currenly unlocking position.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Lockup {
    pub owner: Addr,
    pub id: u64,
    pub release_at: Expiration,
    pub amount: Uint128,
}

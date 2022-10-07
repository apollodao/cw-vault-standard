use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Uint128};
use cw_utils::{Duration, Expiration};

#[cw_serde]
pub enum LockupExecuteMsg {
    /// Withdraw an unlocking position that has finished unlocking.
    WithdrawUnlocked {
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
        //
        /// The ID of the expired lockup to withdraw from.
        /// If None is passed, the vault will attempt to withdraw all expired
        /// lockup positions. Note that this can fail if there are too many
        /// lockup positions and the `max_contract_gas` limit is hit.
        lockup_id: Option<u64>,
    },

    /// Unlock is called to initiate unlocking a locked position held by the
    /// vault.
    /// The caller must pass the native vault tokens in the funds field.
    /// Emits an Unlock event with `amount` attribute containing an u64 lockup_id.
    /// Also encodes the u64 lockup ID as binary and returns it in the Response's
    /// data field, so that it can be read by SubMsg replies.
    Unlock,

    /// Can be called by whitelisted addresses to bypass the lockup and
    /// immediately return the underlying assets. Used in the event of
    /// liquidation. The caller must pass the native vault tokens in the funds
    /// field.
    ForceWithdraw {
        /// The address which should receive the withdrawn assets.
        recipient: Option<String>,
    },

    /// Force withdraw from a position that is already unlocking (Unlock has
    /// already been called).
    ForceWithdrawUnlocking {
        /// The address of the owner of the position.
        owner: String,
        /// The ID of the unlocking position from which to force withdraw
        lockup_id: u64,
        /// Optional amounts of each underlying asset to be force withdrawn.
        /// If None is passed, the entire position will be force withdrawn.
        /// Vaults MAY require the ratio of assets to be the same as the ratio
        /// in the `deposit_assets` field returned by the `VaultInfo` query.
        amounts: Option<Vec<Coin>>,
        #[cfg(feature = "cw20")]
        cw20s_amounts: Option<Vec<Cw20Coin>>,
        /// The address which should receive the withdrawn assets. If not set,
        /// the assets will be sent to the caller.
        recipient: Option<String>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum LockupQueryMsg {
    /// Returns a `Vec<Lockup>` containing all the currently unclaimed lockup
    /// positions for the `owner`.
    #[returns(Vec<Lockup>)]
    Lockups {
        /// The address of the owner of the lockup
        owner: String,
        /// Return results only after this lockup_id
        start_after: Option<u64>,
        /// Max amount of results to return
        limit: Option<u32>,
    },

    /// Returns `Lockup` info about a specific lockup, by owner and ID.
    #[returns(Lockup)]
    Lockup { owner: String, lockup_id: u64 },

    /// Returns `cw_utils::Duration` duration of the lockup.
    #[returns(Duration)]
    LockupDuration,
}

/// Info about a currenly unlocking position.
#[cw_serde]
pub struct Lockup {
    pub owner: Addr,
    pub id: u64,
    pub release_at: Expiration,
    pub amount: Uint128,
}

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub enum ForceUnlockExecuteMsg {
    /// Can be called by whitelisted addresses to bypass the lockup and
    /// immediately return the base tokens. Used in the event of
    /// liquidation. The caller must pass the native vault tokens in the funds
    /// field.
    ForceRedeem {
        /// The address which should receive the withdrawn assets. If not set,
        /// the caller address will be used instead.
        recipient: Option<String>,
        /// The amount of vault tokens to force redeem.
        amount: Uint128,
    },

    /// Force withdraw from a position that is already unlocking (Unlock has
    /// already been called).
    ForceWithdrawUnlocking {
        /// The ID of the unlocking position from which to force withdraw
        lockup_id: u64,
        /// Optional amount of base tokens to be force withdrawn.
        /// If None is passed, the entire position will be force withdrawn.
        amount: Option<Uint128>,
        /// The address which should receive the withdrawn assets. If not set,
        /// the assets will be sent to the caller.
        recipient: Option<String>,
    },

    /// Update the whitelist of addresses that can call ForceRedeem and
    /// ForceWithdrawUnlocking.
    UpdateForceWithdrawWhitelist {
        add_addresses: Vec<String>,
        remove_addresses: Vec<String>,
    },
}

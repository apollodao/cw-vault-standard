use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_binary, Addr, Binary, Coin, CosmosMsg, StdResult, Uint128, WasmMsg};
use cw_utils::{Duration, Expiration};

use crate::{ExtensionExecuteMsg, VaultStandardExecuteMsg};

/// Type for the unlocking position created event emitted on call to `Unlock`.
pub const UNLOCKING_POSITION_CREATED_EVENT_TYPE: &str = "unlocking_position_created";
/// Key for the lockup id attribute in the "unlocking position created" event
/// that is emitted on call to `Unlock`.
pub const UNLOCKING_POSITION_ATTR_KEY: &str = "lockup_id";

/// The message that will be sent to the contract when calling `WithdrawUnlockedToContract`.
#[cw_serde]
pub struct WithdrawUnlockedMsg {
    /// The amount of base tokens that were withdrawn.
    pub amount: Uint128,
    /// The native denom or cw20 contract address of the base token that was withdrawn.
    pub base_token: String,
    /// The custom message that will be sent to the contract.
    pub msg: Binary,
}

impl WithdrawUnlockedMsg {
    /// serializes the message
    pub fn into_binary(self) -> StdResult<Binary> {
        let msg = ReceiverExecuteMsg::ReceiveWithdrawn(self);
        to_binary(&msg)
    }

    /// creates a cosmos_msg sending this struct to the named contract
    pub fn into_cosmos_msg<T: Into<String>>(self, contract_addr: T) -> StdResult<CosmosMsg> {
        let msg = self.into_binary()?;
        let execute = WasmMsg::Execute {
            contract_addr: contract_addr.into(),
            msg,
            funds: vec![],
        };
        Ok(execute.into())
    }
}

/// The ExecuteMsg variant that must exist on the contract that is called by
/// `WithdrawUnlockedToContract`.
#[cw_serde]
pub enum ReceiverExecuteMsg {
    ReceiveWithdrawn(WithdrawUnlockedMsg),
}

/// Additional ExecuteMsg variants for vaults that enable the Lockup extension.
#[cw_serde]
pub enum LockupExecuteMsg {
    /// Unlock is called to initiate unlocking a locked position held by the
    /// vault.
    /// The caller must pass the native vault tokens in the funds field.
    /// Emits an event with type `UNLOCKING_POSITION_CREATED_EVENT_TYPE` with
    /// an attribute with key `UNLOCKING_POSITION_ATTR_KEY` containing an u64
    /// lockup_id.
    ///
    /// Like Redeem, this takes an amount so that the same API can be used for
    /// CW4626 and native tokens.
    Unlock {
        /// The amount of vault tokens to unlock.
        amount: Uint128,
    },

    /// EmergencyUnlock is called to initiate unlocking a locked position held
    /// by the vault.
    /// This call should simply unlock `amount` of vault tokens, without performing
    /// any other side effects that might cause the transaction to fail. Such
    /// as for example compoundning rewards for an LP position.
    EmergencyUnlock {
        /// The amount of vault tokens to unlock.
        amount: Uint128,
    },

    /// Withdraw an unlocking position that has finished unlocking.
    WithdrawUnlocked {
        /// An optional field containing which address should receive the
        /// withdrawn base tokens. If not set, the caller address will be
        /// used instead.
        recipient: Option<String>,
        /// The ID of the expired lockup to withdraw from.
        lockup_id: u64,
    },

    /// Withdraw an unlocking position that has finished unlocking, send the
    /// base tokens to a contract, and execute a message on that contract.
    WithdrawUnlockedToContract {
        /// The ID of the expired lockup to withdraw from.
        lockup_id: u64,
        /// The contract address to send the withdrawn base tokens to.
        contract: String,
        /// The custom message to pass to the recipient contract.
        msg: Binary,
    },
}

impl LockupExecuteMsg {
    /// Convert a [`LockupExecuteMsg`] into a [`CosmosMsg`].
    pub fn into_cosmos_msg(self, contract_addr: String, funds: Vec<Coin>) -> StdResult<CosmosMsg> {
        Ok(WasmMsg::Execute {
            contract_addr,
            msg: to_binary(&VaultStandardExecuteMsg::VaultExtension(
                ExtensionExecuteMsg::Lockup(self),
            ))?,
            funds,
        }
        .into())
    }
}

/// Additional QueryMsg variants for vaults that enable the Lockup extension.
#[cw_serde]
#[derive(QueryResponses)]
pub enum LockupQueryMsg {
    /// Returns a `Vec<UnlockingPosition>` containing all the currently
    /// unclaimed lockup positions for the `owner`.
    #[returns(Vec<UnlockingPosition>)]
    UnlockingPositions {
        /// The address of the owner of the lockup
        owner: String,
        /// Return results only after this lockup_id
        start_after: Option<u64>,
        /// Max amount of results to return
        limit: Option<u32>,
    },

    /// Returns an `UnlockingPosition` info about a specific lockup, by owner
    /// and ID.
    #[returns(UnlockingPosition)]
    UnlockingPosition {
        /// The ID of the lockup to query
        lockup_id: u64,
    },

    /// Returns `cw_utils::Duration` duration of the lockup of the vault.
    #[returns(Duration)]
    LockupDuration {},
}

/// Info about a currenly unlocking position.
#[cw_serde]
pub struct UnlockingPosition {
    /// The ID of the lockup.
    pub id: u64,
    /// The address of the owner of the lockup.
    pub owner: Addr,
    /// A `cw_utils::Expiration` containing information about when the position
    /// completes unlocking.
    pub release_at: Expiration,
    /// The amount of base tokens that are being unlocked.
    pub base_token_amount: Uint128,
}

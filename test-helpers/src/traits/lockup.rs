use cosmwasm_std::{coin, Coin, Uint128};
use cw_it::helpers::Unwrap;
use cw_it::test_tube::{Account, Runner, SigningAccount};

use cw_utils::Duration;
use cw_vault_standard::extensions::lockup::{LockupExecuteMsg, LockupQueryMsg, UnlockingPosition};
use cw_vault_standard::msg::VaultStandardExecuteMsg as ExecuteMsg;
use cw_vault_standard::{ExtensionExecuteMsg, ExtensionQueryMsg, VaultStandardQueryMsg};

use super::CwVaultStandardRobot;

pub trait LockedVaultRobot<'a, R: Runner<'a> + 'a>: CwVaultStandardRobot<'a, R> {
    /// Calls `ExecuteMsg::Unlock` with the given funds.
    fn unlock_with_funds(
        &self,
        funds: &[Coin],
        unwrap_choice: Unwrap,
        signer: &SigningAccount,
    ) -> &Self {
        unwrap_choice.unwrap(self.wasm().execute(
            &self.vault_addr(),
            &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Lockup(LockupExecuteMsg::Unlock {})),
            funds,
            signer,
        ));
        self
    }

    /// Calls `ExecuteMsg::Unlock` with the given amount and the correct native coins in the funds field.
    fn unlock(
        &self,
        amount: impl Into<Uint128>,
        unwrap_choice: Unwrap,
        signer: &SigningAccount,
    ) -> &Self {
        let info = self.query_info();
        let amount: Uint128 = amount.into();
        self.unlock_with_funds(
            &[coin(amount.u128(), info.vault_token)],
            unwrap_choice,
            signer,
        )
    }

    /// Calls `ExecuteMsg::Unlock` with the all of the account's vault tokens.
    fn unlock_all(&self, unwrap_choice: Unwrap, signer: &SigningAccount) -> &Self {
        let amount = self.query_vault_token_balance(signer.address());
        self.unlock(amount, unwrap_choice, signer)
    }

    /// Calls `ExecuteMsg::WithdrawUnlocked` to withdraw tokens from a lockup position.
    fn withdraw_unlocked(
        &self,
        lockup_id: u64,
        recipient: Option<String>,
        unwrap_choice: Unwrap,
        signer: &SigningAccount,
    ) -> &Self {
        unwrap_choice.unwrap(self.wasm().execute(
            &self.vault_addr(),
            &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Lockup(
                LockupExecuteMsg::WithdrawUnlocked {
                    lockup_id,
                    recipient,
                },
            )),
            &[],
            signer,
        ));
        self
    }

    /// Queries the vault for all unlocking positions of the given address (with optional pagination).
    fn query_unlocking_positions(
        &self,
        address: impl Into<String>,
        start_after: Option<u64>,
        limit: Option<u32>,
    ) -> Vec<UnlockingPosition> {
        self.wasm()
            .query(
                &self.vault_addr(),
                &VaultStandardQueryMsg::VaultExtension(ExtensionQueryMsg::Lockup(
                    LockupQueryMsg::UnlockingPositions {
                        owner: address.into(),
                        start_after,
                        limit,
                    },
                )),
            )
            .unwrap()
    }

    /// Queries the vault for a single unlocking position.
    fn query_unlocking_position(&self, lockup_id: u64) -> UnlockingPosition {
        self.wasm()
            .query(
                &self.vault_addr(),
                &VaultStandardQueryMsg::VaultExtension(ExtensionQueryMsg::Lockup(
                    LockupQueryMsg::UnlockingPosition { lockup_id },
                )),
            )
            .unwrap()
    }

    /// Queries the vault for the lockup duration.
    fn query_lockup_duration(&self) -> Duration {
        self.wasm()
            .query(
                &self.vault_addr(),
                &VaultStandardQueryMsg::VaultExtension(ExtensionQueryMsg::Lockup(
                    LockupQueryMsg::LockupDuration {},
                )),
            )
            .unwrap()
    }

    /// Asserts that the number of unlocking positions in the vault is equal to the given value.
    fn assert_number_of_unlocking_positions(
        &self,
        address: impl Into<String>,
        expected: usize,
    ) -> &Self {
        let positions = self.query_unlocking_positions(address, None, None);
        assert_eq!(positions.len(), expected);

        self
    }

    /// Asserts that an unlocking position at the given id is equal to the given value.
    fn assert_unlocking_position_eq(&self, lockup_id: u64, expected: UnlockingPosition) -> &Self {
        let position = self.query_unlocking_position(lockup_id);
        assert_eq!(position, expected);

        self
    }
}

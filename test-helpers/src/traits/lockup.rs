use cosmwasm_std::{coin, Coin, Uint128};
use cw_it::test_tube::{Runner, SigningAccount};

use cw_utils::Duration;
use cw_vault_standard::extensions::lockup::{LockupExecuteMsg, LockupQueryMsg, UnlockingPosition};
use cw_vault_standard::msg::VaultStandardExecuteMsg as ExecuteMsg;
use cw_vault_standard::{ExtensionExecuteMsg, ExtensionQueryMsg, VaultStandardQueryMsg};

use super::CwVaultStandardRobot;

pub trait LockedVaultRobot<'a, R: Runner<'a> + 'a>: CwVaultStandardRobot<'a, R> {
    fn unlock_with_funds(
        &self,
        amount: impl Into<Uint128>,
        signer: &SigningAccount,
        funds: &[Coin],
    ) -> &Self {
        self.wasm()
            .execute(
                &self.vault_addr(),
                &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Lockup(
                    LockupExecuteMsg::Unlock {
                        amount: amount.into(),
                    },
                )),
                funds,
                signer,
            )
            .unwrap();
        self
    }

    fn unlock(&self, amount: impl Into<Uint128>, signer: &SigningAccount) -> &Self {
        let info = self.query_info();
        let amount: Uint128 = amount.into();
        self.unlock_with_funds(
            amount.clone(),
            signer,
            &[coin(amount.u128(), info.vault_token)],
        )
    }

    fn withdraw_unlocked(
        &self,
        lockup_id: u64,
        recipient: Option<String>,
        signer: &SigningAccount,
    ) -> &Self {
        self.wasm()
            .execute(
                &self.vault_addr(),
                &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Lockup(
                    LockupExecuteMsg::WithdrawUnlocked {
                        lockup_id,
                        recipient,
                    },
                )),
                &[],
                signer,
            )
            .unwrap();
        self
    }

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

    fn assert_number_of_unlocking_positions(
        &self,
        address: impl Into<String>,
        expected: usize,
    ) -> &Self {
        let positions = self.query_unlocking_positions(address, None, None);
        assert_eq!(positions.len(), expected);

        self
    }

    fn assert_unlocking_position_eq(&self, lockup_id: u64, expected: UnlockingPosition) -> &Self {
        let position = self.query_unlocking_position(lockup_id);
        assert_eq!(position, expected);

        self
    }
}

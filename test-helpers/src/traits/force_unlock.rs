use cosmwasm_std::Uint128;
use cw_it::test_tube::{Runner, SigningAccount};

use cw_vault_standard::extensions::force_unlock::ForceUnlockExecuteMsg;
use cw_vault_standard::msg::VaultStandardExecuteMsg as ExecuteMsg;
use cw_vault_standard::ExtensionExecuteMsg;

use super::CwVaultStandardRobot;

pub trait ForceUnlockVaultRobot<'a, R: Runner<'a> + 'a>: CwVaultStandardRobot<'a, R> {
    fn force_redeem(
        &self,
        amount: impl Into<Uint128>,
        recipient: Option<String>,
        signer: &SigningAccount,
    ) -> &Self {
        self.wasm()
            .execute(
                &self.vault_addr(),
                &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::ForceUnlock(
                    ForceUnlockExecuteMsg::ForceRedeem {
                        recipient,
                        amount: amount.into(),
                    },
                )),
                &[],
                signer,
            )
            .unwrap();
        self
    }

    fn force_withdraw_unlocking(
        &self,
        lockup_id: u64,
        amount: Option<impl Into<Uint128>>,
        recipient: Option<String>,
        signer: &SigningAccount,
    ) -> &Self {
        self.wasm()
            .execute(
                &self.vault_addr(),
                &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::ForceUnlock(
                    ForceUnlockExecuteMsg::ForceWithdrawUnlocking {
                        amount: amount.map(Into::into),
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

    fn update_force_withdraw_whitelist(
        &self,
        signer: &SigningAccount,
        add_addresses: Vec<String>,
        remove_addresses: Vec<String>,
    ) -> &Self {
        self.wasm()
            .execute(
                &self.vault_addr(),
                &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::ForceUnlock(
                    ForceUnlockExecuteMsg::UpdateForceWithdrawWhitelist {
                        add_addresses,
                        remove_addresses,
                    },
                )),
                &[],
                signer,
            )
            .unwrap();
        self
    }
}

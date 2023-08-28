use cosmwasm_std::{coin, Coin, Uint128};
use cw_it::helpers::Unwrap;
use cw_it::test_tube::{Account, Runner, SigningAccount};

use cw_vault_standard::extensions::force_unlock::ForceUnlockExecuteMsg;
use cw_vault_standard::msg::VaultStandardExecuteMsg as ExecuteMsg;
use cw_vault_standard::ExtensionExecuteMsg;

use super::CwVaultStandardRobot;

pub trait ForceUnlockVaultRobot<'a, R: Runner<'a> + 'a>: CwVaultStandardRobot<'a, R> {
    /// Calls `ExecuteMsg::ForceRedeem` with the given amount and funds.
    fn force_redeem_with_funds(
        &self,
        amount: impl Into<Uint128>,
        recipient: Option<String>,
        funds: &[Coin],
        unwrap_choice: Unwrap,
        signer: &SigningAccount,
    ) -> &Self {
        unwrap_choice.unwrap(self.wasm().execute(
            &self.vault_addr(),
            &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::ForceUnlock(
                ForceUnlockExecuteMsg::ForceRedeem {
                    recipient,
                    amount: amount.into(),
                },
            )),
            funds,
            signer,
        ));
        self
    }

    /// Calls `ExecuteMsg::ForceRedeem` with the given amount and the correct native coins in the funds field.
    fn force_redeem(
        &self,
        amount: impl Into<Uint128>,
        recipient: Option<String>,
        unwrap_choice: Unwrap,
        signer: &SigningAccount,
    ) -> &Self {
        let amount: Uint128 = amount.into();
        self.force_redeem_with_funds(
            amount,
            recipient,
            &[coin(amount.u128(), self.vault_token())],
            unwrap_choice,
            signer,
        )
    }

    /// Calls `ExecuteMsg::ForceRedeem` with the all of the account's vault tokens.
    fn force_redeem_all(
        &self,
        recipient: Option<String>,
        unwrap_choice: Unwrap,
        signer: &SigningAccount,
    ) -> &Self {
        let amount = self.query_vault_token_balance(signer.address());
        self.force_redeem(amount, recipient, unwrap_choice, signer)
    }

    /// Calls `ExecuteMsg::ForceWithdrawUnlocking` to withdraw tokens from a lockup position before
    /// the lockup has matured.
    fn force_withdraw_unlocking(
        &self,
        lockup_id: u64,
        amount: Option<impl Into<Uint128>>,
        recipient: Option<String>,
        unwrap_choice: Unwrap,
        signer: &SigningAccount,
    ) -> &Self {
        unwrap_choice.unwrap(self.wasm().execute(
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
        ));
        self
    }

    /// Updates the force withdraw whitelist.
    fn update_force_withdraw_whitelist(
        &self,
        add_addresses: Vec<String>,
        remove_addresses: Vec<String>,
        unwrap_choice: Unwrap,
        signer: &SigningAccount,
    ) -> &Self {
        unwrap_choice.unwrap(self.wasm().execute(
            &self.vault_addr(),
            &ExecuteMsg::VaultExtension(ExtensionExecuteMsg::ForceUnlock(
                ForceUnlockExecuteMsg::UpdateForceWithdrawWhitelist {
                    add_addresses,
                    remove_addresses,
                },
            )),
            &[],
            signer,
        ));
        self
    }
}

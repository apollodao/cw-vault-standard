#[cfg(feature = "lockup")]
pub mod lockup;

#[cfg(feature = "force-unlock")]
pub mod force_unlock;

use cosmwasm_std::{coin, Coin, Empty, Uint128};
use cw_it::robot::TestRobot;
use cw_it::test_tube::{Account, Runner, SigningAccount};

use cw_vault_standard::msg::{
    VaultStandardExecuteMsg as ExecuteMsg, VaultStandardQueryMsg as QueryMsg,
};
use cw_vault_standard::VaultInfoResponse;

pub trait CwVaultStandardRobot<'a, R: Runner<'a> + 'a>: TestRobot<'a, R> {
    fn vault_addr(&self) -> String;

    fn query_info(&self) -> VaultInfoResponse {
        self.wasm()
            .query(&self.vault_addr(), &QueryMsg::<Empty>::Info {})
            .unwrap()
    }

    /// Returns the base token.
    fn base_token(&self) -> String {
        self.query_info().base_token
    }

    /// Returns the vault token.
    fn vault_token(&self) -> String {
        self.query_info().vault_token
    }

    fn deposit(
        &self,
        amount: impl Into<Uint128>,
        recipient: Option<String>,
        funds: &[Coin],
        signer: &SigningAccount,
    ) -> &Self {
        let amount: Uint128 = amount.into();
        self.wasm()
            .execute(
                &self.vault_addr(),
                &ExecuteMsg::<Empty>::Deposit { amount, recipient },
                funds,
                signer,
            )
            .unwrap();
        self
    }

    fn deposit_all(&self, recipient: Option<String>, signer: &SigningAccount) -> &Self {
        let base_token_denom = self.query_info().base_token;
        let amount = self.query_native_token_balance(&signer.address(), &base_token_denom);

        self.deposit(
            amount,
            recipient,
            &[Coin::new(amount.u128(), base_token_denom)],
            signer,
        )
    }

    fn query_base_token_balance(&self, address: impl Into<String>) -> Uint128;

    fn assert_base_token_balance_eq(
        &self,
        address: impl Into<String>,
        amount: impl Into<Uint128>,
    ) -> &Self {
        let amount: Uint128 = amount.into();
        let balance = self.query_base_token_balance(address);
        assert_eq!(balance, amount);
        self
    }

    fn query_vault_token_balance(&self, address: impl Into<String>) -> Uint128 {
        let info = self.query_info();
        self.query_native_token_balance(address, &info.vault_token)
    }

    fn assert_vault_token_balance_eq(
        &self,
        address: impl Into<String>,
        amount: impl Into<Uint128>,
    ) -> &Self {
        let amount: Uint128 = amount.into();
        let balance = self.query_vault_token_balance(address);
        assert_eq!(balance, amount);
        self
    }

    fn redeem_with_funds(
        &self,
        amount: Uint128,
        recipient: Option<String>,
        funds: &[Coin],
        signer: &SigningAccount,
    ) -> &Self {
        self.wasm()
            .execute(
                &self.vault_addr(),
                &ExecuteMsg::<Empty>::Redeem { amount, recipient },
                funds,
                signer,
            )
            .unwrap();
        self
    }

    fn redeem(&self, amount: Uint128, recipient: Option<String>, signer: &SigningAccount) -> &Self {
        self.redeem_with_funds(
            amount,
            recipient,
            &[coin(amount.u128(), self.vault_token())],
            signer,
        )
    }

    fn redeem_all(&self, recipient: Option<String>, signer: &SigningAccount) -> &Self {
        let amount =
            self.query_native_token_balance(signer.address(), &self.query_info().vault_token);
        self.redeem(amount, recipient, signer)
    }
}

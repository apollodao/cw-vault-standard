#[cfg(feature = "lockup")]
pub mod lockup;

#[cfg(feature = "force-unlock")]
pub mod force_unlock;

use cosmwasm_std::{coin, Coin, Empty, Uint128};
use cw_it::helpers::Unwrap;
use cw_it::robot::TestRobot;
use cw_it::test_tube::{Account, Runner, SigningAccount};

use cw_vault_standard::msg::{
    VaultStandardExecuteMsg as ExecuteMsg, VaultStandardQueryMsg as QueryMsg,
};
use cw_vault_standard::VaultInfoResponse;

pub trait CwVaultStandardRobot<'a, R: Runner<'a> + 'a>: TestRobot<'a, R> {
    /// Returns the vault address.
    fn vault_addr(&self) -> String;

    /// Returns the base token and vault tokens.
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

    /// Calls `ExecuteMsg::Deposit` with the given amount and funds.
    fn deposit_with_funds(
        &self,
        amount: impl Into<Uint128>,
        recipient: Option<String>,
        funds: &[Coin],
        unwrap_choice: Unwrap,
        signer: &SigningAccount,
    ) -> &Self {
        let amount: Uint128 = amount.into();
        unwrap_choice.unwrap(self.wasm().execute(
            &self.vault_addr(),
            &ExecuteMsg::<Empty>::Deposit { amount, recipient },
            funds,
            signer,
        ));
        self
    }

    /// Calls `ExecuteMsg::Deposit` with the given amount and the correct native coins in the funds field.
    fn deposit(
        &self,
        amount: impl Into<Uint128>,
        recipient: Option<String>,
        unwrap_choice: Unwrap,
        signer: &SigningAccount,
    ) -> &Self {
        let amount = amount.into();
        self.deposit_with_funds(
            amount,
            recipient,
            &[coin(amount.u128(), self.base_token())],
            unwrap_choice,
            signer,
        )
    }

    /// Calls `ExecuteMsg::Deposit` with the all of the account's native tokens.
    fn deposit_all(
        &self,
        recipient: Option<String>,
        unwrap_choice: Unwrap,
        signer: &SigningAccount,
    ) -> &Self {
        let base_token_denom = self.query_info().base_token;
        let amount = self.query_native_token_balance(signer.address(), base_token_denom);

        self.deposit(amount, recipient, unwrap_choice, signer)
    }

    /// Calls `ExecuteMsg::Redeem` with the given amount and funds.
    fn redeem_with_funds(
        &self,
        amount: Uint128,
        recipient: Option<String>,
        funds: &[Coin],
        unwrap_choice: Unwrap,
        signer: &SigningAccount,
    ) -> &Self {
        unwrap_choice.unwrap(self.wasm().execute(
            &self.vault_addr(),
            &ExecuteMsg::<Empty>::Redeem { amount, recipient },
            funds,
            signer,
        ));
        self
    }

    /// Calls `ExecuteMsg::Redeem` with the given amount and the correct native coins in the funds field.
    fn redeem(
        &self,
        amount: Uint128,
        recipient: Option<String>,
        unwrap_choice: Unwrap,
        signer: &SigningAccount,
    ) -> &Self {
        self.redeem_with_funds(
            amount,
            recipient,
            &[coin(amount.u128(), self.vault_token())],
            unwrap_choice,
            signer,
        )
    }

    /// Calls `ExecuteMsg::Redeem` with the all of the account's vault tokens.
    fn redeem_all(
        &self,
        recipient: Option<String>,
        unwrap_choice: Unwrap,
        signer: &SigningAccount,
    ) -> &Self {
        let amount =
            self.query_native_token_balance(signer.address(), self.query_info().vault_token);
        self.redeem(amount, recipient, unwrap_choice, signer)
    }

    /////// QUERIES ///////

    /// Queries the base token balance of the given address.
    fn query_base_token_balance(&self, address: impl Into<String>) -> Uint128;

    /// Queries the native token balance of the given address.
    fn query_vault_token_balance(&self, address: impl Into<String>) -> Uint128 {
        let info = self.query_info();
        self.query_native_token_balance(address, info.vault_token)
    }

    /////// ASSERTIONS ///////

    /// Asserts that the base token balance of the given address is equal to the given amount.
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

    /// Asserts that the base token balance of the given address is greater than the given amount.
    fn assert_base_token_balance_gt(
        &self,
        address: impl Into<String>,
        amount: impl Into<Uint128>,
    ) -> &Self {
        let amount: Uint128 = amount.into();
        let balance = self.query_base_token_balance(address);
        assert!(balance > amount);
        self
    }

    /// Asserts that the base token balance of the given address is less than the given amount.
    fn assert_base_token_balance_lt(
        &self,
        address: impl Into<String>,
        amount: impl Into<Uint128>,
    ) -> &Self {
        let amount: Uint128 = amount.into();
        let balance = self.query_base_token_balance(address);
        assert!(balance < amount);
        self
    }

    /// Asserts that the vault token balance of the given address is equal to the given amount.
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

    /// Asserts that the vault token balance of the given address is greater than the given amount.
    fn assert_vault_token_balance_gt(
        &self,
        address: impl Into<String>,
        amount: impl Into<Uint128>,
    ) -> &Self {
        let amount: Uint128 = amount.into();
        let balance = self.query_vault_token_balance(address);
        assert!(balance > amount);
        self
    }

    /// Asserts that the vault token balance of the given address is less than the given amount.
    fn assert_vault_token_balance_lt(
        &self,
        address: impl Into<String>,
        amount: impl Into<Uint128>,
    ) -> &Self {
        let amount: Uint128 = amount.into();
        let balance = self.query_vault_token_balance(address);
        assert!(balance < amount);
        self
    }
}

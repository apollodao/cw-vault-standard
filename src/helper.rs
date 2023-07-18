use std::marker::PhantomData;

use cosmwasm_std::{
    coin, to_binary, Addr, Api, CosmosMsg, QuerierWrapper, StdResult, Uint128, WasmMsg,
};
use schemars::JsonSchema;
use serde::Serialize;

use crate::{
    VaultInfoResponse, VaultStandardExecuteMsg, VaultStandardInfoResponse, VaultStandardQueryMsg,
};

pub struct VaultContractUnchecked<E, Q> {
    addr: String,
    execute_msg_extension: PhantomData<E>,
    query_msg_extension: PhantomData<Q>,
}

impl<E, Q> VaultContractUnchecked<E, Q>
where
    E: Serialize,
    Q: Serialize + JsonSchema,
{
    pub fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_string(),
            execute_msg_extension: PhantomData,
            query_msg_extension: PhantomData,
        }
    }

    pub fn check(&self, api: &dyn Api) -> StdResult<VaultContract<E, Q>> {
        Ok(VaultContract::new(&api.addr_validate(&self.addr)?))
    }
}

pub struct VaultContract<E, Q> {
    addr: Addr,
    execute_msg_extension: PhantomData<E>,
    query_msg_extension: PhantomData<Q>,
}

impl<E, Q> VaultContract<E, Q>
where
    E: Serialize,
    Q: Serialize + JsonSchema,
{
    /// Create a new VaultContract instance.
    pub fn new(addr: &Addr) -> Self {
        Self {
            addr: addr.clone(),
            execute_msg_extension: PhantomData,
            query_msg_extension: PhantomData,
        }
    }

    /// Returns a CosmosMsg to deposit base tokens into the vault.
    pub fn deposit(
        &self,
        amount: impl Into<Uint128>,
        base_denom: &str,
        recipient: Option<impl Into<String>>,
    ) -> StdResult<CosmosMsg> {
        let amount = amount.into();
        Ok(WasmMsg::Execute {
            contract_addr: self.addr.to_string(),
            msg: to_binary(&VaultStandardExecuteMsg::<E>::Deposit {
                amount: amount.clone(),
                recipient: recipient.map(|r| r.into()),
            })?,
            funds: vec![coin(amount.u128(), base_denom)],
        }
        .into())
    }

    /// Returns a CosmosMsg to redeem vault tokens from the vault.
    pub fn redeem(
        &self,
        amount: impl Into<Uint128>,
        vault_token_denom: &str,
        recipient: Option<impl Into<String>>,
    ) -> StdResult<CosmosMsg> {
        let amount = amount.into();
        Ok(WasmMsg::Execute {
            contract_addr: self.addr.to_string(),
            msg: to_binary(&VaultStandardExecuteMsg::<E>::Redeem {
                amount: amount.clone(),
                recipient: recipient.map(|r| r.into()),
            })?,
            funds: vec![coin(amount.u128(), vault_token_denom)],
        }
        .into())
    }

    /// Queries the vault for the vault standard info
    pub fn query_vault_standard_info(
        &self,
        querier: &QuerierWrapper,
    ) -> StdResult<VaultStandardInfoResponse> {
        querier.query_wasm_smart(
            &self.addr,
            &to_binary(&VaultStandardQueryMsg::<Q>::VaultStandardInfo {})?,
        )
    }

    /// Queries the vault for the vault info
    pub fn query_vault_info(&self, querier: &QuerierWrapper) -> StdResult<VaultInfoResponse> {
        querier.query_wasm_smart(
            &self.addr,
            &to_binary(&VaultStandardQueryMsg::<Q>::Info {})?,
        )
    }

    /// Queries the vault for a preview of a deposit
    pub fn query_preview_deposit(
        &self,
        querier: &QuerierWrapper,
        amount: impl Into<Uint128>,
    ) -> StdResult<Uint128> {
        querier.query_wasm_smart(
            &self.addr,
            &to_binary(&VaultStandardQueryMsg::<Q>::PreviewDeposit {
                amount: amount.into(),
            })?,
        )
    }

    /// Queries the vault for a preview of a redeem
    pub fn query_preview_redeem(
        &self,
        querier: &QuerierWrapper,
        amount: impl Into<Uint128>,
    ) -> StdResult<Uint128> {
        querier.query_wasm_smart(
            &self.addr,
            &to_binary(&VaultStandardQueryMsg::<Q>::PreviewRedeem {
                amount: amount.into(),
            })?,
        )
    }

    /// Queries the vault for the total assets held in the vault
    pub fn query_total_assets(&self, querier: &QuerierWrapper) -> StdResult<Uint128> {
        querier.query_wasm_smart(
            &self.addr,
            &to_binary(&VaultStandardQueryMsg::<Q>::TotalAssets {})?,
        )
    }

    /// Queries the vault for the total vault token supply
    pub fn query_total_vault_token_supply(&self, querier: &QuerierWrapper) -> StdResult<Uint128> {
        querier.query_wasm_smart(
            &self.addr,
            &to_binary(&VaultStandardQueryMsg::<Q>::TotalVaultTokenSupply {})?,
        )
    }

    /// Queries the vault to convert an amount of vault tokens to base tokens
    pub fn query_convert_to_shares(
        &self,
        querier: &QuerierWrapper,
        amount: impl Into<Uint128>,
    ) -> StdResult<Uint128> {
        querier.query_wasm_smart(
            &self.addr,
            &to_binary(&VaultStandardQueryMsg::<Q>::ConvertToShares {
                amount: amount.into(),
            })?,
        )
    }

    /// Queries the vault to convert an amount of base tokens to vault tokens
    pub fn query_convert_to_assets(
        &self,
        querier: &QuerierWrapper,
        amount: impl Into<Uint128>,
    ) -> StdResult<Uint128> {
        querier.query_wasm_smart(
            &self.addr,
            &to_binary(&VaultStandardQueryMsg::<Q>::ConvertToAssets {
                amount: amount.into(),
            })?,
        )
    }
}
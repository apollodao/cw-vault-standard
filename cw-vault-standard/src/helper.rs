use std::marker::PhantomData;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coin, to_json_binary, Addr, CosmosMsg, Decimal, Deps, QuerierWrapper, StdResult, Uint128,
    WasmMsg,
};
use schemars::JsonSchema;
use serde::Serialize;

use crate::{
    ExtensionExecuteMsg, ExtensionQueryMsg, VaultInfoResponse, VaultStandardExecuteMsg,
    VaultStandardInfoResponse, VaultStandardQueryMsg,
};

/// A helper struct to interact with a vault contract that adheres to the vault
/// standard. This struct contains an unchecked address. By calling the `check`
/// method, the address is checked against the api and the checked version of
/// the struct is returned.
#[cw_serde]
pub struct VaultContractUnchecked<E = ExtensionExecuteMsg, Q = ExtensionQueryMsg> {
    pub addr: String,
    execute_msg_extension: PhantomData<E>,
    query_msg_extension: PhantomData<Q>,
}

impl<E, Q> VaultContractUnchecked<E, Q>
where
    E: Serialize,
    Q: Serialize + JsonSchema,
{
    /// Create a new VaultContractUnchecked instance.
    pub fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_string(),
            execute_msg_extension: PhantomData,
            query_msg_extension: PhantomData,
        }
    }

    /// Check the address against the api and return a checked version of the
    /// struct.
    pub fn check(&self, deps: Deps) -> StdResult<VaultContract<E, Q>> {
        VaultContract::new(&deps.querier, &deps.api.addr_validate(&self.addr)?)
    }
}

/// A helper struct to interact with a vault contract that adheres to the vault
/// standard.
#[cw_serde]
pub struct VaultContract<E = ExtensionExecuteMsg, Q = ExtensionQueryMsg> {
    /// The address of the vault contract.
    pub addr: Addr,
    /// The base token of the vault contract.
    pub base_token: String,
    /// The vault token denom of the vault contract.
    pub vault_token: String,
    /// The extension enum for ExecuteMsg variants.
    execute_msg_extension: PhantomData<E>,
    /// The extension enum for QueryMsg variants.
    query_msg_extension: PhantomData<Q>,
}

impl<E, Q> VaultContract<E, Q>
where
    E: Serialize,
    Q: Serialize + JsonSchema,
{
    /// Create a new VaultContract instance.
    pub fn new(querier: &QuerierWrapper, addr: &Addr) -> StdResult<Self> {
        // Query vault info
        let vault_info: VaultInfoResponse =
            querier.query_wasm_smart(addr, &VaultStandardQueryMsg::<Q>::Info {})?;

        Ok(Self {
            addr: addr.clone(),
            base_token: vault_info.base_token,
            vault_token: vault_info.vault_token,
            execute_msg_extension: PhantomData,
            query_msg_extension: PhantomData,
        })
    }

    /// Returns a CosmosMsg to deposit base tokens into the vault.
    pub fn deposit(
        &self,
        amount: impl Into<Uint128>,
        recipient: Option<String>,
    ) -> StdResult<CosmosMsg> {
        let amount = amount.into();

        Ok(WasmMsg::Execute {
            contract_addr: self.addr.to_string(),
            msg: to_json_binary(&VaultStandardExecuteMsg::<E>::Deposit { recipient })?,
            funds: vec![coin(amount.u128(), &self.base_token)],
        }
        .into())
    }

    /// Returns a CosmosMsg to redeem vault tokens from the vault.
    pub fn redeem(
        &self,
        amount: impl Into<Uint128>,
        recipient: Option<String>,
    ) -> StdResult<CosmosMsg> {
        let amount = amount.into();
        Ok(WasmMsg::Execute {
            contract_addr: self.addr.to_string(),
            msg: to_json_binary(&VaultStandardExecuteMsg::<E>::Redeem { recipient })?,
            funds: vec![coin(amount.u128(), &self.vault_token)],
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
            &VaultStandardQueryMsg::<Q>::VaultStandardInfo {},
        )
    }

    /// Queries the vault for the vault info
    pub fn query_vault_info(&self, querier: &QuerierWrapper) -> StdResult<VaultInfoResponse> {
        querier.query_wasm_smart(&self.addr, &VaultStandardQueryMsg::<Q>::Info {})
    }

    /// Queries the vault for the total assets held in the vault
    pub fn query_total_assets(&self, querier: &QuerierWrapper) -> StdResult<Uint128> {
        querier.query_wasm_smart(&self.addr, &VaultStandardQueryMsg::<Q>::TotalAssets {})
    }

    /// Queries the vault for the total vault token supply
    pub fn query_total_vault_token_supply(&self, querier: &QuerierWrapper) -> StdResult<Uint128> {
        querier.query_wasm_smart(
            &self.addr,
            &VaultStandardQueryMsg::<Q>::TotalVaultTokenSupply {},
        )
    }

    /// Queries the vault for the vault token exchange rate
    pub fn query_vault_token_exchange_rate(
        &self,
        quote_denom: String,
        querier: &QuerierWrapper,
    ) -> StdResult<Decimal> {
        querier.query_wasm_smart(
            &self.addr,
            &VaultStandardQueryMsg::<Q>::VaultTokenExchangeRate { quote_denom },
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
            &VaultStandardQueryMsg::<Q>::ConvertToShares {
                amount: amount.into(),
            },
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
            &VaultStandardQueryMsg::<Q>::ConvertToAssets {
                amount: amount.into(),
            },
        )
    }
}

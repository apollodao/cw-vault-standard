use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub base_token: String,
}

#[cw_serde]
pub struct Config {
    pub base_token: String,
    pub vault_token: String,
}

pub type ExecuteMsg = cw_vault_standard::VaultStandardExecuteMsg;
pub type QueryMsg = cw_vault_standard::VaultStandardQueryMsg;

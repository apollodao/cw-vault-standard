use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct KeeperJob {
    //The numeric ID of the job
    pub id: u64,
    /// bool whether only whitelisted keepers can execute the job
    pub whitelist: bool,
    /// A list of whitelisted addresses that can execute the job
    pub whitelisted_keepers: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum KeeperExecuteMsg {
    /// Callable by vault admin to whitelist a keeper to be able to execute a job
    WhitelistKeeper { job_id: u64, keeper: String },
    /// Callable by vault admin to remove a keeper from the whitelist of a job
    BlacklistKeeper { job_id: u64, keeper: String },
    /// Execute a keeper job. Should only be able to be called if
    /// QueryMsg::KeeperJobReady returns true, and only by whitelisted
    /// keepers if the whitelist bool on the KeeperJob is set to true.
    ExecuteJob { job_id: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum KeeperQueryMsg {
    /// Returns Vec<KeeperJob>
    KeeperJobs,
    /// Returns Vec<Addr>
    WhitelistedKeepers { job_id: u64 },
    /// Returns bool, whether the keeper job can be executed or not
    KeeperJobReady { job_id: u64 },
}

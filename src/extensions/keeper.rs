use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

/// A job that can be performed by a keeper.
#[cw_serde]
pub struct KeeperJob {
    /// The numeric ID of the job
    pub id: u64,
    /// whether only whitelisted keepers can execute the job or not
    pub whitelist: bool,
    /// A list of whitelisted addresses that can execute the job
    pub whitelisted_keepers: Vec<Addr>,
}

/// Additional ExecuteMsg variants for vaults that enable the Keeper extension.
#[cw_serde]
pub enum KeeperExecuteMsg {
    /// Callable by vault admin to whitelist a keeper to be able to execute a
    /// job
    WhitelistKeeper {
        /// The ID of the job to whitelist the keeper for
        job_id: u64,
        /// The address of the keeper to whitelist
        keeper: String,
    },
    /// Callable by vault admin to remove a keeper from the whitelist of a job
    BlacklistKeeper {
        /// The ID of the job to blacklist the keeper for
        job_id: u64,
        /// The address of the keeper to blacklist
        keeper: String,
    },
    /// Execute a keeper job. Should only be able to be called if
    /// [`KeeperQueryMsg::KeeperJobReady`] returns true, and only by whitelisted
    /// keepers if the whitelist bool on the KeeperJob is set to true.
    ExecuteJob {
        /// The ID of the job to execute
        job_id: u64,
    },
}

/// Additional QueryMsg variants for vaults that enable the Keeper extension.
#[cw_serde]
#[derive(QueryResponses)]
pub enum KeeperQueryMsg {
    /// Returns [`Vec<KeeperJob>`]
    #[returns(Vec<KeeperJob>)]
    KeeperJobs {},
    /// Returns [`Vec<Addr>`]
    #[returns(Vec<Addr>)]
    WhitelistedKeepers {
        /// The ID of the job to get the whitelisted keepers for
        job_id: u64,
    },
    /// Returns bool, whether the keeper job can be executed or not
    #[returns(bool)]
    KeeperJobReady {
        /// The ID of the job to check whether it is ready to be executed
        job_id: u64,
    },
}

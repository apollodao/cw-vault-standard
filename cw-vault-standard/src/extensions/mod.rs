/// The lockup extension can be used to create vaults where the vault tokens are
/// not immediately reedemable. Instead of normally calling the
/// `VaultStandardExecuteMsg::Redeem` variant, the user has to call the `Unlock`
/// variant on the Lockup extension `ExecuteMsg` and wait for a specified period
/// of time before they can withdraw their assets via the
/// `WithdrawUnlocked` variant.
#[cfg(feature = "lockup")]
#[cfg_attr(docsrs, doc(cfg(feature = "lockup")))]
pub mod lockup;

/// The force unlock extension can be used to create a vault that also
/// implements the `Lockup` extension, but where some whitelisted addresses are
/// allowed to call the `ForceUnlock` variant on the extension `ExecuteMsg` and
/// immediately unlock the vault tokens of the specified user. This is useful if
/// the vault is used with leverage and a liquidator needs to be able to
/// liquidate the tokens locked in the vault.
#[cfg(feature = "force-unlock")]
#[cfg_attr(docsrs, doc(cfg(feature = "force-unlock")))]
pub mod force_unlock;

/// The keeper extension can be used to add functionality for either whitelisted
/// addresses or anyone to act as a "keeper" for the vault and call functions to
/// perform jobs that need to be done to keep the vault running.
#[cfg(feature = "keeper")]
#[cfg_attr(docsrs, doc(cfg(feature = "keeper")))]
pub mod keeper;

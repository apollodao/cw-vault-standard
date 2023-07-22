use cosmwasm_std::{Deps, Env, StdResult, Uint128};

use crate::state::{CONFIG, VAULT_TOKEN_SUPPLY};

/// Converts an amount of base tokens to vault tokens
///
/// # Arguments
///
/// * `base_tokens` - The amount of base tokens to convert
/// * `deduct_deposit` - If true, the amount of base tokens will be deducted
///   from the total staked base tokens. This is useful when calculating the
///   amount of vault tokens to mint after a deposit, since we want to use the
///   total staked amount before the deposit.
pub fn calculate_vault_tokens(
    deps: &Deps,
    env: &Env,
    base_tokens: Uint128,
    deduct_deposit: bool,
) -> StdResult<Uint128> {
    let config = CONFIG.load(deps.storage)?;

    let total_vt_supply = VAULT_TOKEN_SUPPLY.load(deps.storage)?;
    let mut total_staked_bt = deps
        .querier
        .query_balance(&env.contract.address, config.base_token)?
        .amount;
    if deduct_deposit {
        total_staked_bt = total_staked_bt.checked_sub(base_tokens)?;
    }

    let vault_token_amount = if total_vt_supply.is_zero() || total_staked_bt.is_zero() {
        base_tokens
    } else {
        base_tokens.multiply_ratio(total_vt_supply, total_staked_bt)
    };

    Ok(vault_token_amount)
}

/// Converts an amount of vault tokens to base tokens
pub fn calculate_base_tokens(deps: &Deps, env: &Env, vault_tokens: Uint128) -> StdResult<Uint128> {
    let config = CONFIG.load(deps.storage)?;

    let total_vt_supply = VAULT_TOKEN_SUPPLY.load(deps.storage)?;
    let total_staked_bt = deps
        .querier
        .query_balance(&env.contract.address, config.base_token)?
        .amount;
    let base_token_amount = if total_vt_supply.is_zero() || total_staked_bt.is_zero() {
        vault_tokens
    } else {
        vault_tokens.multiply_ratio(total_staked_bt, total_vt_supply)
    };

    Ok(base_token_amount)
}

use cosmwasm_std::{
    coin, entry_point, to_binary, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, Event,
    MessageInfo, Response, StdError, StdResult, Uint128,
};

use cw_vault_standard::{VaultInfoResponse, VaultStandardInfoResponse};

use osmosis_std::types::cosmos::base::v1beta1::Coin;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgBurn, MsgCreateDenom, MsgMint};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

use crate::helpers;
use crate::msg::{Config, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{CONFIG, VAULT_TOKEN_SUPPLY};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let subdenom = "vault-token";

    // Create vault token denom
    let create_msg: CosmosMsg = MsgCreateDenom {
        sender: env.contract.address.to_string(),
        subdenom: subdenom.to_string(),
    }
    .into();

    let vault_token_denom = format!("factory/{}/{}", env.contract.address, subdenom);

    // Create and store config
    CONFIG.save(
        deps.storage,
        &Config {
            base_token: msg.base_token,
            vault_token: vault_token_denom.clone(),
        },
    )?;
    VAULT_TOKEN_SUPPLY.save(deps.storage, &Uint128::zero())?;

    let event =
        Event::new("mock-vault/instantiate").add_attribute("vault_token_denom", vault_token_denom);

    Ok(Response::default().add_message(create_msg).add_event(event))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::Deposit { amount, recipient } => {
            let config = CONFIG.load(deps.storage)?;
            let vault_token = config.vault_token;

            let recipient = recipient
                .map(|x| deps.api.addr_validate(&x))
                .unwrap_or_else(|| Ok(info.sender))?;

            let mint_amount = helpers::calculate_vault_tokens(&deps.as_ref(), &env, amount, true)?;

            VAULT_TOKEN_SUPPLY.update(deps.storage, |mut supply| -> StdResult<_> {
                supply += mint_amount;
                Ok(supply)
            })?;

            let mint_msg: CosmosMsg = MsgMint {
                sender: env.contract.address.to_string(),
                amount: Some(Coin {
                    denom: vault_token.clone(),
                    amount: mint_amount.into(),
                }),
                mint_to_address: env.contract.address.to_string(),
            }
            .into();
            let send_msg: CosmosMsg<Empty> = CosmosMsg::Bank(BankMsg::Send {
                amount: vec![coin(mint_amount.into(), &vault_token)],
                to_address: recipient.to_string(),
            });

            let event = Event::new("mock-vault/deposit")
                .add_attribute("deposit_amount", amount.to_string())
                .add_attribute("mint_amount", mint_amount)
                .add_attribute("recipient", recipient)
                .add_attribute("vault_token", vault_token);

            Ok(Response::default()
                .add_message(mint_msg)
                .add_message(send_msg)
                .add_event(event))
        }
        ExecuteMsg::Redeem { recipient, amount } => {
            let config = CONFIG.load(deps.storage)?;
            let vault_token = config.vault_token;
            let base_token = config.base_token;

            if info.funds.len() != 1 {
                return Err(StdError::generic_err("Must deposit exactly one coin"));
            }
            if info.funds[0].denom != vault_token {
                return Err(StdError::generic_err("Must deposit vault tokens"));
            }
            if info.funds[1].amount != amount {
                return Err(StdError::generic_err("Must deposit exactly amount"));
            }

            let recipient = recipient
                .map(|x| deps.api.addr_validate(&x))
                .unwrap_or_else(|| Ok(info.sender))?;

            let redeem_amount = helpers::calculate_base_tokens(&deps.as_ref(), &env, amount)?;

            VAULT_TOKEN_SUPPLY.update(deps.storage, |mut supply| -> StdResult<_> {
                supply += amount;
                Ok(supply)
            })?;

            let burn_msg: CosmosMsg = MsgBurn {
                sender: env.contract.address.to_string(),
                amount: Some(Coin {
                    denom: vault_token.clone(),
                    amount: amount.into(),
                }),
                burn_from_address: env.contract.address.to_string(),
            }
            .into();
            let send_msg: CosmosMsg<Empty> = CosmosMsg::Bank(BankMsg::Send {
                amount: vec![coin(redeem_amount.into(), base_token)],
                to_address: recipient.to_string(),
            });

            let event = Event::new("mock-vault/redeem")
                .add_attribute("vault_token_amount", amount)
                .add_attribute("redeem_amount", redeem_amount)
                .add_attribute("recipient", recipient)
                .add_attribute("vault_token", vault_token);

            Ok(Response::default()
                .add_message(burn_msg)
                .add_message(send_msg)
                .add_event(event))
        }
        ExecuteMsg::VaultExtension(_) => unimplemented!("No vault extensions enabled"),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::VaultStandardInfo {} => Ok(to_binary(&VaultStandardInfoResponse {
            version: 1u16,
            extensions: vec![],
        })?),
        QueryMsg::Info {} => {
            let config = CONFIG.load(deps.storage)?;
            Ok(to_binary(&VaultInfoResponse {
                base_token: config.base_token,
                vault_token: config.vault_token,
            })?)
        }
        QueryMsg::PreviewDeposit { amount } => Ok(to_binary(&helpers::calculate_vault_tokens(
            &deps, &env, amount, false,
        )?)?),
        QueryMsg::PreviewRedeem { amount } => Ok(to_binary(&helpers::calculate_base_tokens(
            &deps, &env, amount,
        )?)?),
        QueryMsg::TotalAssets {} => Ok(to_binary(
            &deps
                .querier
                .query_balance(env.contract.address, CONFIG.load(deps.storage)?.base_token)?
                .amount,
        )?),
        QueryMsg::TotalVaultTokenSupply {} => {
            Ok(to_binary(&VAULT_TOKEN_SUPPLY.load(deps.storage)?)?)
        }
        QueryMsg::ConvertToShares { amount } => Ok(to_binary(&helpers::calculate_vault_tokens(
            &deps, &env, amount, false,
        )?)?),
        QueryMsg::ConvertToAssets { amount } => Ok(to_binary(&helpers::calculate_base_tokens(
            &deps, &env, amount,
        )?)?),
        QueryMsg::VaultExtension(_) => unimplemented!("No vault extensions enabled"),
    }
}

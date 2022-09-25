#[cfg(not(feature = "library"))]
use crate::error::ContractError;
use crate::execute;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query;
use crate::state;
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw-contract-template";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: InstantiateMsg,
) -> Result<Response, ContractError> {
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  state::initialize(deps, &env, &info, &msg)?;
  Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  match msg {
    ExecuteMsg::IncrementClaim {
      claimant,
      amount,
      expiry,
    } => execute::increment_claim(deps, env, info, claimant, amount, expiry),
    ExecuteMsg::IncrementClaimBatch { claims } => {
      execute::increment_claim_batch(deps, env, info, claims)
    },
    ExecuteMsg::WithdrawClaim { claimant } => execute::withdraw_claim(deps, env, info, claimant),
    ExecuteMsg::Claim { amount } => execute::claim(deps, env, info, amount),
  }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
  deps: Deps,
  env: Env,
  msg: QueryMsg,
) -> StdResult<Binary> {
  let result = match msg {
    QueryMsg::GetClaimAmount { claimant } => {
      to_binary(&query::get_claim_amount(deps, env, claimant)?)
    },
  }?;
  Ok(result)
}

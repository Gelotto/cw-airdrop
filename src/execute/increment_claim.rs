use crate::{
  error::ContractError,
  state::{Claim, CLAIMS, OWNER_ADDRESS, TOTAL_CLAIM_AMOUNT},
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response, Timestamp, Uint128};

/// Add or update a Claim. Requires owner auth.
pub fn increment_claim(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  claimant: Addr,
  amount: Uint128,
  expiry: Option<Timestamp>,
) -> Result<Response, ContractError> {
  let owner_address = OWNER_ADDRESS.load(deps.storage)?;
  // abort if the sender isn't the contract owner
  if info.sender != owner_address {
    return Err(ContractError::NotAuthorized {
      reason: Some("only the airdrop owner can perform this action".into()),
    });
  }
  // increment global total claim amount
  TOTAL_CLAIM_AMOUNT.update(
    deps.storage,
    |total_amount| -> Result<Uint128, ContractError> { Ok(total_amount + amount) },
  )?;
  // insert new claim
  CLAIMS.update(
    deps.storage,
    claimant.clone(),
    |some_claim| -> Result<Claim, ContractError> {
      if let Some(mut claim) = some_claim {
        // update existing claim
        claim.amount += amount;
        Ok(claim)
      } else {
        // create new claim
        Ok(Claim { amount, expiry })
      }
    },
  )?;
  Ok(Response::new().add_attributes(vec![
    attr("action", "increment_claim"),
    attr("claimant", claimant.to_string()),
    attr("amount", amount.to_string()),
  ]))
}

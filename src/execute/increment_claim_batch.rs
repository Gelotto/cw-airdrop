use crate::{
  error::ContractError,
  msg::ClaimConfig,
  state::{Claim, CLAIMS, OWNER_ADDRESS, TOTAL_CLAIM_AMOUNT},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Uint128};

/// Add or update a Claim amount. Requires owner auth.
pub fn increment_claim_batch(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  claim_configs: Vec<ClaimConfig>,
) -> Result<Response, ContractError> {
  let owner_address = OWNER_ADDRESS.load(deps.storage)?;
  // abort if the sender isn't the contract owner
  if info.sender != owner_address {
    return Err(ContractError::NotAuthorized {
      reason: Some("only the airdrop owner can perform this action".into()),
    });
  }
  // total claim amount registered in this request:
  let mut claim_amount_subtotal = Uint128::zero();
  // increment each claim record
  for claim_config in claim_configs.iter() {
    claim_amount_subtotal += claim_config.amount;
    // insert new claim
    CLAIMS.update(
      deps.storage,
      claim_config.claimant.clone(),
      |some_claim| -> Result<Claim, ContractError> {
        if let Some(mut claim) = some_claim {
          // update existing claim
          claim.amount += claim_config.amount;
          claim.expiry = claim_config.expiry.clone();
          Ok(claim)
        } else {
          // create new claim
          Ok(Claim {
            amount: claim_config.amount,
            expiry: claim_config.expiry.clone(),
          })
        }
      },
    )?;
  }
  // increment global claim amount
  TOTAL_CLAIM_AMOUNT.update(
    deps.storage,
    |total_amount| -> Result<Uint128, ContractError> { Ok(total_amount + claim_amount_subtotal) },
  )?;
  Ok(Response::new().add_attributes(vec![
    attr("action", "increment_claim_batch"),
    attr("total_amount", claim_amount_subtotal.to_string()),
  ]))
}

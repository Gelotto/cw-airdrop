use crate::{
  error::ContractError,
  state::{CLAIMS, CW20_TOKEN_ADDRESS},
  util::build_cw20_transfer_submsg,
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Uint128};

/// The sender can use the `claim` function to transfer any amount up to their
/// outstanding Claim balance to their account.
pub fn claim(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  claim_amount: Option<Uint128>,
) -> Result<Response, ContractError> {
  let some_claim = CLAIMS.may_load(deps.storage, info.sender.clone())?;
  if let Some(mut claim) = some_claim {
    // abort if the claim has expired
    if let Some(expiry) = claim.expiry {
      if env.block.time >= expiry {
        return Err(ContractError::NotAuthorized {
          reason: Some("claim expired".to_string()),
        });
      }
    }
    // resolve the actual amount to claim, defaults to total outstanding balance
    let resolved_claim_amount = claim_amount.unwrap_or(claim.amount).min(claim.amount);
    // subtract the amount claimed from the outstanding claim balance
    claim.amount -= resolved_claim_amount;
    // remove existing claim record if it's now empty
    if claim.amount == resolved_claim_amount {
      CLAIMS.remove(deps.storage, info.sender.clone());
    } else {
      CLAIMS.save(deps.storage, info.sender.clone(), &claim)?;
    }
    // transfer tokens to claimant
    let cw20_token_address = CW20_TOKEN_ADDRESS.load(deps.storage)?;
    Ok(
      Response::new()
        .add_attributes(vec![
          attr("action", "claim"),
          attr("claimant", info.sender.to_string()),
          attr("amount", resolved_claim_amount.to_string()),
        ])
        .add_submessage(build_cw20_transfer_submsg(
          &cw20_token_address,
          &info.sender,
          resolved_claim_amount,
        )?),
    )
  } else {
    // abort if claim not found
    Err(ContractError::NotAuthorized {
      reason: Some("claim not found".into()),
    })
  }
}

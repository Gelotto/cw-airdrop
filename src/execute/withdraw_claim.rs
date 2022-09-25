use crate::{
  error::ContractError,
  state::{CLAIMS, CW20_TOKEN_ADDRESS, OWNER_ADDRESS},
  util::build_cw20_transfer_submsg,
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

/// Clears out an existing claim record, transfering the balance to the airdrop
/// "owner" account. Requires owner auth.
pub fn withdraw_claim(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  claimant: Addr,
) -> Result<Response, ContractError> {
  let owner_address = OWNER_ADDRESS.load(deps.storage)?;
  // abort if the sender isn't the contract owner
  if info.sender != owner_address {
    return Err(ContractError::NotAuthorized {
      reason: Some("only the airdrop owner can perform this action".into()),
    });
  }
  let some_claim = CLAIMS.may_load(deps.storage, claimant.clone())?;
  if let Some(claim) = some_claim {
    // abort if the claim has NOT expired
    if let Some(expiry) = claim.expiry {
      if env.block.time < expiry {
        return Err(ContractError::NotAuthorized {
          reason: Some("claim has not expired".into()),
        });
      }
    }
    // remove existing claim record
    CLAIMS.remove(deps.storage, info.sender.clone());
    // transfer tokens to claimant
    let cw20_token_address = CW20_TOKEN_ADDRESS.load(deps.storage)?;
    Ok(
      Response::new()
        .add_attributes(vec![
          attr("action", "withdraw_claim"),
          attr("amount", claim.amount.to_string()),
        ])
        .add_submessage(build_cw20_transfer_submsg(
          &cw20_token_address,
          &owner_address,
          claim.amount,
        )?),
    )
  } else {
    // abort if claim not found
    Err(ContractError::NotAuthorized {
      reason: Some("claim not found".into()),
    })
  }
}

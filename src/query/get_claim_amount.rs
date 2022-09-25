use crate::{msg::GetClaimAmount, state::CLAIMS};
use cosmwasm_std::{Addr, Deps, Env, StdResult, Uint128};

/// Return the oustanding Claim balance for a given claimant address, along with
/// an optional "reason" string, detailing the reason for the balance being what
/// it is.
pub fn get_claim_amount(
  deps: Deps,
  env: Env,
  claimant: Addr,
) -> StdResult<GetClaimAmount> {
  let some_claim = CLAIMS.may_load(deps.storage, claimant.clone())?;
  if let Some(claim) = some_claim {
    if let Some(expiry) = claim.expiry {
      // can't claim because the claim has expired
      if expiry < env.block.time {
        return Ok(GetClaimAmount {
          amount: claim.amount,
          reason: Some("claim expired".into()),
        });
      }
    }
    // can claim!
    Ok(GetClaimAmount {
      amount: claim.amount,
      reason: None,
    })
  } else {
    // can't claim because claim not found
    Ok(GetClaimAmount {
      amount: Uint128::zero(),
      reason: None,
    })
  }
}

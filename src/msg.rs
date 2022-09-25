use cosmwasm_std::{Addr, Timestamp, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Initial contract state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
  pub cw20_token_address: Addr,
}

/// Executable contract endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
  WithdrawClaim {
    claimant: Addr,
  },
  IncrementClaim {
    claimant: Addr,
    amount: Uint128,
    expiry: Option<Timestamp>,
  },
  IncrementClaimBatch {
    claims: Vec<ClaimConfig>,
  },
  Claim {
    amount: Option<Uint128>,
  },
}

/// Custom contract query endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  GetClaimAmount { claimant: Addr },
}

/// Custom contract query endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetClaimAmount {
  pub amount: Uint128,
  pub reason: Option<String>,
}

/// Custom contract query endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ClaimConfig {
  pub claimant: Addr,
  pub amount: Uint128,
  pub expiry: Option<Timestamp>,
}

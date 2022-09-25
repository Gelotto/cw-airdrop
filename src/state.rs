use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Claim {
  pub amount: Uint128,
  pub expiry: Option<Timestamp>,
}

pub const OWNER_ADDRESS: Item<Addr> = Item::new("owner");
pub const CW20_TOKEN_ADDRESS: Item<Addr> = Item::new("cw20_token_address");
pub const CLAIMS: Map<Addr, Claim> = Map::new("claims");
pub const TOTAL_CLAIM_AMOUNT: Item<Uint128> = Item::new("total_claim_amount");

/// Initialize contract state data.
pub fn initialize(
  deps: DepsMut,
  _env: &Env,
  info: &MessageInfo,
  msg: &InstantiateMsg,
) -> Result<(), ContractError> {
  OWNER_ADDRESS.save(deps.storage, &info.sender)?;
  CW20_TOKEN_ADDRESS.save(deps.storage, &msg.cw20_token_address)?;
  TOTAL_CLAIM_AMOUNT.save(deps.storage, &Uint128::zero())?;
  Ok(())
}

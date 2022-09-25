use cosmwasm_std::{to_binary, Addr, StdResult, SubMsg, Uint128, WasmMsg};
use cw20::Cw20ExecuteMsg;

pub fn build_cw20_transfer_submsg(
  cw20_token: &Addr,
  wallet: &Addr,
  amount: Uint128,
) -> StdResult<SubMsg> {
  Ok(SubMsg::new(WasmMsg::Execute {
    contract_addr: cw20_token.clone().into(),
    msg: to_binary(&Cw20ExecuteMsg::Transfer {
      recipient: wallet.into(),
      amount: amount.into(),
    })?,
    funds: vec![],
  }))
}

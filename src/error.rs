use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContractError {
  #[error("{0}")]
  Std(#[from] StdError),

  #[error("ValidationError")]
  ValidationError {},

  #[error("NotAuthorized")]
  NotAuthorized { reason: Option<String> },
}

use anchor_lang::prelude::*;

#[error_code]
pub enum NFTMatketPlaceProgramError {
  #[msg("nft martket place program error")]
  UndefinedError,
}
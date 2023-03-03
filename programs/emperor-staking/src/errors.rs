use anchor_lang::prelude::*;

// Custom Program Errors
#[error_code]
pub enum CustomError {
  #[msg("Keys should be equal")]
  KeyMismatch,
  #[msg("Unauthorized access")]
  Unauthorized,
  #[msg("User does not beling to the vault")]
  IncorrectVault,
  #[msg("NFT is not in collection")]
  IncorrectTreasury,
  #[msg("Treasury Supplied doesn't belong to the vault")]
  WrongNFT,
  #[msg("Mint should be equal")]
  MintMismatch,
  #[msg("Authority should be equal")]
  AccountMismatch,
  #[msg("Max number staked")]
  MaxStaked,
  #[msg("Already Boosted")]
  AlreadyBoosted,
}

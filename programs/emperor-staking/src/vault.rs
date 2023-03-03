use crate::errors::*;
use crate::ins::*;
use crate::state::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{self};


/*
* Vault:: Initialize Instruction - Create the vault account.
*/
pub fn handle_vault_initialization(
  ctx: Context<InitializeVault>,
  vault_name: String,
  creator_address: Pubkey,
  payout_interval: u64,
  payout_amount: u64,
  community_wallet: Pubkey,
  stake_fee: u64,
  unstake_fee: u64,
) -> Result<()> {
  let vault = &mut ctx.accounts.vault;
  vault.name = vault_name;
  vault.authority = ctx.accounts.authority.key();
  vault.creator_address = creator_address;
  vault.community_wallet = community_wallet;
  vault.payout_interval = payout_interval;
  vault.payout_amount = payout_amount;
  vault.total_amount = 0;
  vault.stake_fee = stake_fee;
  vault.unstake_fee = unstake_fee;
  vault.total_earned = 0;
  
  msg!("User Account Size: {:?}", User::LEN);
  msg!("Staked Item Size: {:?}", StakedNft::LEN);

  vault.bump = *ctx.bumps.get("vault").unwrap();
  Ok(())
}

/*
* Vault:: Update Instruction - Update the vault account.
*/
pub fn handle_vault_update(
  ctx: Context<UpdateVault>,
  creator_address: Pubkey,
  payout_interval: u64,
  payout_amount: u64,
  community_wallet: Pubkey,
  stake_fee: u64,
  unstake_fee: u64,
) -> Result<()> {
  require_keys_eq!(
    ctx.accounts.vault.authority,
    ctx.accounts.authority.key(),
    CustomError::Unauthorized
  );
  let vault = &mut ctx.accounts.vault;

  vault.creator_address = creator_address;
  vault.community_wallet = community_wallet;
  vault.payout_interval = payout_interval;
  vault.payout_amount = payout_amount;
  vault.stake_fee = stake_fee;
  vault.unstake_fee = unstake_fee;

  Ok(())
}

/*
* Vault:: Fund Instruction - Fund the vault account with tokens.
*/
pub fn handle_fund(ctx: Context<FundTokenVault>, amount: u64) -> Result<()> {
  let token_ctx = CpiContext::new(
    ctx.accounts.token_program.to_account_info(),
    token::Transfer {
      authority: ctx.accounts.funder.to_account_info(),
      from: ctx.accounts.funder_ata.to_account_info(),
      to: ctx.accounts.reward_token_vault_ata.to_account_info(),
    },
  );
  token::transfer(token_ctx, amount)?;
  let vault = &mut ctx.accounts.vault;
  vault.total_amount = vault.total_amount.checked_add(amount).unwrap();
  Ok(())
}

/*
* Vault:: Drain Instruction - Drain the vault account tokens.
*/
pub fn handle_drain(ctx: Context<DrainTokenVault>, amount: u64) -> Result<()> {
  require_keys_eq!(
    ctx.accounts.vault.authority.key(),
    ctx.accounts.funder.key(),
    CustomError::Unauthorized
  );

  let token_vault_name = &ctx.accounts.vault.name;
  let token_vault_bump = ctx.accounts.vault.bump;

  let seeds = &[
    b"vault".as_ref(),
    token_vault_name.as_ref(),
    &[token_vault_bump],
  ];
  let signer = &[&seeds[..]];

  let token_ctx = CpiContext::new_with_signer(
    ctx.accounts.token_program.to_account_info(),
    token::Transfer {
      authority: ctx.accounts.vault.to_account_info(),
      from: ctx.accounts.reward_token_vault_ata.to_account_info(),
      to: ctx.accounts.funder_ata.to_account_info(),
    },
    signer,
  );
  token::transfer(token_ctx, amount)?;
  let vault = &mut ctx.accounts.vault;
  vault.total_amount = vault.total_amount.checked_sub(amount).unwrap();
  Ok(())
}

/*
* Vault:: Set Authority Instruction - Set a new authority over the vault.
*/
pub fn handle_set_vault_authority(ctx: Context<SetVaultAuthority>) -> Result<()> {
  require_keys_eq!(
    ctx.accounts.vault.authority,
    ctx.accounts.authority.key(),
    CustomError::Unauthorized
  );

  let vault = &mut ctx.accounts.vault;
  vault.authority = ctx.accounts.new_authority.key();
  Ok(())
}

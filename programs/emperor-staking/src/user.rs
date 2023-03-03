use std::cell::RefMut;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;
use anchor_spl::token::{self};

use crate::state::*;
use crate::ins::*;
/*
* User::Create Instruction - Create the user account.
*/
pub fn handle_create_stake_account(ctx: Context<CreateStakeAccount>) -> Result<()> {
  let mut user_account = ctx.accounts.user_account.load_init()?;

  user_account.user = *ctx.accounts.user.key;

  Ok(())
}

pub fn handle_claim_rewards(ctx: Context<Claim>) -> Result<u64> {
  // Get the vault.
  let vault = &mut ctx.accounts.vault;

  // Get the current timestamp.
  let now: u64 = clock::Clock::get()
    .unwrap()
    .unix_timestamp
    .try_into()
    .unwrap();

  let staker_account = &mut ctx.accounts.staker_account.load_mut()?;

  // Update stakers earned Rewards.
  let staker_earned_amount_since_last_update = get_rewards_earned(
    now,
    staker_account.last_update_time,
    staker_account,
    vault,
  );

  let staker_earned_amount = staker_account
    .reward_earned_pending
    .checked_add(staker_earned_amount_since_last_update)
    .unwrap();

  // Reset the rewards pending to 0.
  staker_account.reward_earned_pending = 0;
  staker_account.last_update_time = now;

  staker_account.reward_earned_claimed = staker_account
    .reward_earned_claimed
    .checked_add(staker_earned_amount)
    .unwrap();

  vault.total_amount = vault.total_amount.checked_sub(staker_earned_amount).unwrap();

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
      to: ctx.accounts.staker_ata.to_account_info(),
    },
    signer,
  );
  token::transfer(token_ctx, staker_earned_amount)?;

  Ok(staker_earned_amount)
}


/*
* Helper function to get the rewards earned by the staker.
*/
#[inline(always)]
pub fn get_rewards_earned(
  current_time: u64,
  last_update_time: u64,
  staker_account: &mut RefMut<User>,
  vault: &Account<Vault>,
) -> u64 {

  // Get time since last update.
  let time_from_last_stake_in_seconds = current_time.checked_sub(last_update_time).unwrap();
  
  let total_amount_earned = time_from_last_stake_in_seconds
    .checked_mul(staker_account.total_reward_rate).unwrap()
    .checked_div(vault.payout_interval).unwrap();
  
  return total_amount_earned;
}

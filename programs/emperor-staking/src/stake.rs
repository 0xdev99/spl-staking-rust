use std::cell::RefMut;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;
use anchor_spl::token::Mint;
use mpl_token_metadata::instruction::{freeze_delegated_account, thaw_delegated_account};
use solana_program::program::{invoke, invoke_signed};

use crate::errors::*;
use crate::state::*;
use crate::user::*;
use crate::ins::*;
/*
* Stake:: Stake Instruction - Stake the user's NFT.
*/
pub fn handle_stake(ctx: Context<Stake>) -> Result<()> {
  let vault = &mut ctx.accounts.vault;
  let staker_account = &mut ctx.accounts.staker_account.load_mut()?;
  let token_mint = &ctx.accounts.token_mint;

  let mut is_max_staked = false;
  if staker_account.mint_staked_count >= MAX_NFT_PER_USER as u32 {
    is_max_staked = true
  }
  require_eq!(is_max_staked, false, CustomError::MaxStaked);

  // Load the NFT metadata
  let metadata =
    spl_token_metadata::state::Metadata::from_account_info(&ctx.accounts.nft_metadata_account)?;
  let creators = metadata.data.creators.unwrap();
  let mut creator_found = false;
  for creator in creators {
    if creator.address.key() == vault.creator_address {
      creator_found = true;
    }
  }

  // NFT must be created by whitelist owner.
  require_eq!(creator_found, true, CustomError::WrongNFT);
  
  if vault.stake_fee > 0 {
    invoke(
      &anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.staker.key,
        ctx.accounts.community_wallet.key,
        vault.stake_fee,
      ),
      &[
        ctx.accounts.staker.to_account_info().clone(),
        ctx.accounts.community_wallet.to_account_info().clone(),
        ctx.accounts.system_program.to_account_info().clone(),
      ],
    )?;
  }
  

  update_accounts("stake", vault, staker_account, token_mint);

  let cpi_context = CpiContext::new(
    ctx.accounts.token_program.to_account_info(),
    anchor_spl::token::Approve {
        to: ctx.accounts.staker_ata.to_account_info().clone(),
        delegate: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.staker.to_account_info()
    }
  );

  anchor_spl::token::approve(cpi_context, 1)?;
  
  // Get the NFT from the Vault,
  let token_vault_name = &ctx.accounts.vault.name;
  let token_vault_bump = ctx.accounts.vault.bump;
  let seeds = &[
    b"vault".as_ref(),
    token_vault_name.as_ref(),
    &[token_vault_bump],
  ];

  invoke_signed(
      &freeze_delegated_account(
          ctx.accounts.token_metadata_program.key(),
          ctx.accounts.vault.key(),
          ctx.accounts.staker_ata.key(),
          ctx.accounts.edition.key(),
          ctx.accounts.token_mint.key(),
      ),
      &[
          ctx.accounts.vault.to_account_info(),
          ctx.accounts.staker_ata.to_account_info(),
          ctx.accounts.edition.to_account_info(),
          ctx.accounts.token_mint.to_account_info()
      ],
      &[seeds]
  )?;

  Ok(())
}

/*
* Unstake:: Untake Instruction - Unstake the user's NFT.
*/
pub fn handle_unstake(ctx: Context<Unstake>) -> Result<()> {
  let vault = &mut ctx.accounts.vault;
  let staker_account = &mut ctx.accounts.staker_account.load_mut()?;
  let token_mint = &ctx.accounts.token_mint;
  
  // Staker should own staker account
  require_keys_eq!(
    ctx.accounts.staker.key(),
    staker_account.user.key(),
    CustomError::KeyMismatch
  );

  // If the staker key is not the same as the signer key,
  // then the signer account should match authority key.
  if ctx.accounts.staker.key() != ctx.accounts.signer.key() {
    require_keys_eq!(
      ctx.accounts.signer.key(),
      vault.authority.key(),
      CustomError::Unauthorized
    );
  }

  // Is correct mint
  let mut is_owner = false;
  for item in &staker_account.staked_items {
    if item.mint == token_mint.key() {
      is_owner = true;
    }
  }

  // Allow authoirity to unstake on users behalf.
  if ctx.accounts.signer.key() == vault.authority.key() {
    is_owner = true;
  }

  require_eq!(is_owner, true, CustomError::Unauthorized);

  if vault.unstake_fee > 0 {
    invoke(
      &anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.staker.key,
        // &vault.key(),
        ctx.accounts.community_wallet.key,
        vault.unstake_fee,
      ),
      &[
        ctx.accounts.staker.to_account_info().clone(),
        // vault.to_account_info().clone(),
        ctx.accounts.community_wallet.to_account_info().clone(),
        ctx.accounts.system_program.to_account_info().clone(),
      ],
    )?;
  }
 
  update_accounts("unstake", vault, staker_account, token_mint);

  // Get the NFT from the Vault,
  let token_vault_name = &ctx.accounts.vault.name;
  let token_vault_bump = ctx.accounts.vault.bump;

  let seeds = &[
    b"vault".as_ref(),
    token_vault_name.as_ref(),
    &[token_vault_bump],
  ];
  invoke_signed(
    &thaw_delegated_account(
        ctx.accounts.token_metadata_program.key(),
        ctx.accounts.vault.key(),
        ctx.accounts.staker_ata.key(),
        ctx.accounts.edition.key(),
        ctx.accounts.token_mint.key(),
    ),
    &[
        ctx.accounts.vault.to_account_info(),
        ctx.accounts.staker_ata.to_account_info(),
        ctx.accounts.edition.to_account_info(),
        ctx.accounts.token_mint.to_account_info()
    ],
    &[seeds]
  )?;


  if ctx.accounts.staker.key() == ctx.accounts.signer.key() {
    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        anchor_spl::token::Revoke {
            source: ctx.accounts.staker_ata.to_account_info(),
            authority: ctx.accounts.staker.to_account_info()
        }
    );

    anchor_spl::token::revoke(cpi_context)?;
  }

  Ok(())
}


/*
* Helper function to update the accounts.
*/
fn update_accounts(
  method: &str,
  vault: &mut Account<Vault>,
  staker_account: &mut RefMut<User>,
  token_mint: &Account<Mint>,
) {
  // Get the last time the account was updated.
  let last_update_time = staker_account.last_update_time;

  // Get the current time.
  let now: u64 = clock::Clock::get()
    .unwrap()
    .unix_timestamp
    .try_into()
    .unwrap();
  staker_account.last_update_time = now;

  // Update stakers earned Rewards.
  let staker_earned_amount = get_rewards_earned(
    now,
    last_update_time,
    staker_account,
    &vault,
  );

  // Update stakers earned Rewards.
  staker_account.reward_earned_pending = staker_account
    .reward_earned_pending
    .checked_add(staker_earned_amount)
    .unwrap();

  staker_account.last_update_time = now;
  
  /*
   * User Is Staking
   */
  if method == "stake" {
    // increment staked count
    vault.total_staked = vault.total_staked.checked_add(1).unwrap();
    let index = staker_account.mint_staked_count as usize;
    staker_account.mint_staked_count = staker_account.mint_staked_count.checked_add(1).unwrap();
    // Add NFT that is being staked to user's mint staked account.
    staker_account.staked_items[index] = StakedNft {
      mint: token_mint.key(),
      reward_rate: vault.payout_amount,
      staked_time: now,
    };
    
    staker_account.total_reward_rate = staker_account.total_reward_rate.checked_add(
      vault.payout_amount
    ).unwrap();

  }
  /*
   * User Is Unstaking
   */
  else if method == "unstake" {
    // decrement staked count
    vault.total_staked = vault.total_staked.checked_sub(1).unwrap();
    staker_account.mint_staked_count = staker_account.mint_staked_count.checked_sub(1).unwrap();
    let last_index = staker_account.mint_staked_count as usize;
    let index = staker_account.staked_items.iter().position(|x| x.mint == token_mint.key()).unwrap();
   
    // Remove NFT that is being unstaked.
    staker_account.staked_items[index] = staker_account.staked_items[last_index];
    staker_account.staked_items[last_index] = StakedNft::default();

    staker_account.total_reward_rate = staker_account.total_reward_rate.checked_sub(
      vault.payout_amount
    ).unwrap();

    msg!("Total Reward Rate: {:?}", staker_account.total_reward_rate);
  }
}

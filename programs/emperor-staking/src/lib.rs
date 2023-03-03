mod ins;
mod state;
mod errors;
mod user;
mod stake;
mod vault;

use anchor_lang::prelude::*;

use crate::ins::*;
use crate::vault::*;
use crate::user::*;
use crate::stake::*;

declare_id!("DVTouieqqLknDQn2UPE87HvWVizMkVj1Q4rqDgjNFYpK");

#[program]
pub mod emperor_staking {
    use super::*;

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        vault_name: String,
        creator_address: Pubkey,
        payout_interval: u64,
        payout_amount: u64,
        community_wallet: Pubkey,
        stake_fee: u64,
        unstake_fee: u64,
    ) -> Result<()> {
        handle_vault_initialization(
            ctx,
            vault_name,
            creator_address,
            payout_interval,
            payout_amount,
            community_wallet,
            stake_fee,
            unstake_fee,
        )
    }

    pub fn update_vault(
        ctx: Context<UpdateVault>,
        creator_address: Pubkey,
        payout_interval: u64,
        payout_amount: u64,
        community_wallet: Pubkey,
        stake_fee: u64,
        unstake_fee: u64,
    ) -> Result<()> {
        handle_vault_update(
            ctx,
            creator_address,
            payout_interval,
            payout_amount,
            community_wallet,
            stake_fee,
            unstake_fee,
        )
    }

    pub fn set_vault_authority(ctx: Context<SetVaultAuthority>) -> Result<()> {
        handle_set_vault_authority(ctx)
    }

    pub fn fund(ctx: Context<FundTokenVault>, amount: u64) -> Result<()> {
        handle_fund(ctx, amount)
    }

    pub fn drain(ctx: Context<DrainTokenVault>, amount: u64) -> Result<()> {
        handle_drain(ctx, amount)
    }

    pub fn create_stake_account(ctx: Context<CreateStakeAccount>) -> Result<()> {
        handle_create_stake_account(ctx)
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        handle_stake(ctx)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        handle_unstake(ctx)
    }

    pub fn claim(ctx: Context<Claim>) -> Result<u64> {
        handle_claim_rewards(ctx)
    }
    
    pub fn close_pda(ctx: Context<ClosePda>) -> Result<()> {
        let dest_account_info = ctx.accounts.signer.to_account_info();
        let source_account_info = ctx.accounts.pda.to_account_info();
        let dest_starting_lamports = dest_account_info.lamports();
        **dest_account_info.lamports.borrow_mut() = dest_starting_lamports
            .checked_add(source_account_info.lamports())
            .unwrap();
        **source_account_info.lamports.borrow_mut() = 0;

        Ok(())
    }
}

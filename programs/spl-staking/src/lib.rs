mod ins;
mod state;

use crate::ins::*;
use anchor_lang::{prelude::*, system_program};
use anchor_spl::token::{transfer, Transfer};
use emperor_staking::cpi::{accounts::Claim as ClaimJewels, claim as claim_jewels};
use emperor_staking::{self};

declare_id!("9GAsSHWvHoHoqbk8tqHYCq3fcpyGmovgXD5GBkSo4p3f");

#[program]
pub mod spl_staking {

    use super::*;

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        daily_payout_amount: u64,
        bump: u8,
    ) -> Result<()> {
        let mut vault = ctx.accounts.vault.load_init()?;
        vault.bump = bump;
        vault.stake_token_mint = ctx.accounts.stake_token_mint.key();
        vault.daily_payout_amount = daily_payout_amount;
        vault.authority = ctx.accounts.authority.key();

        Ok(())
    }

    pub fn update_vault(
        ctx: Context<UpdateVault>,
        new_authority: Pubkey,
        daily_payout_amount: u64,
    ) -> Result<()> {
        let mut vault = ctx.accounts.vault.load_mut()?;
        vault.stake_token_mint = ctx.accounts.stake_token_mint.key();
        vault.daily_payout_amount = daily_payout_amount;
        vault.authority = new_authority;

        Ok(())
    }

    pub fn fund(ctx: Context<Fund>, amount: u64) -> Result<()> {
        let mut vault = ctx.accounts.vault.load_mut()?;

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.funder_ata.to_account_info(),
                    to: ctx.accounts.vault_ata.to_account_info(),
                    authority: ctx.accounts.funder.to_account_info(),
                },
            ),
            amount,
        )?;

        vault.reward_pool_amount = vault.reward_pool_amount.checked_add(amount).unwrap();

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let mut vault = ctx.accounts.vault.load_mut()?;
        let bump = vault.bump;
        let vault_bump = bump;
        let seeds = [b"vault".as_ref(), &[vault_bump]];
        let signer = &[&seeds[..]];
        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault_ata.to_account_info(),
                    to: ctx.accounts.authority_ata.to_account_info(),
                    authority: ctx.accounts.token_vault.to_account_info(),
                },
                signer,
            ),
            amount,
        )?;

        vault.reward_pool_amount = vault.reward_pool_amount.checked_sub(amount).unwrap();

        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let mut vault = ctx.accounts.vault.load_mut()?;
        let stake_fee = ctx.accounts.fee_vault.stake_fee;

        if stake_fee > 0 {
            system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: ctx.accounts.staker.to_account_info(),
                        to: ctx.accounts.fee_wallet.to_account_info(),
                    },
                ),
                stake_fee,
            )?;
        }

        vault.stake(ctx.accounts.staker.key(), amount);

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.staker_ata.to_account_info(),
                    to: ctx.accounts.vault_ata.to_account_info(),
                    authority: ctx.accounts.staker.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn stake_with_claim(ctx: Context<StakeWithClaim>) -> Result<()> {
        let mut vault = ctx.accounts.vault.load_mut()?;

        let stake_fee = ctx.accounts.fee_vault.stake_fee;

        if stake_fee > 0 {
            system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: ctx.accounts.staker.to_account_info(),
                        to: ctx.accounts.fee_wallet.to_account_info(),
                    },
                ),
                stake_fee,
            )?;
        }

        let amount = claim_jewels(CpiContext::new(
            ctx.accounts.emperor_program.to_account_info(),
            ClaimJewels {
                signer: ctx.accounts.staker.to_account_info(),
                staker: ctx.accounts.staker.to_account_info(),
                staker_account: ctx.accounts.staker_account.to_account_info(),
                vault: ctx.accounts.emperor_vault.to_account_info(),
                reward_token_mint: ctx.accounts.stake_token_mint.to_account_info(),
                staker_ata: ctx.accounts.staker_ata.to_account_info(),
                reward_token_vault_ata: ctx.accounts.reward_token_vault_ata.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ))?
        .get();

        vault.stake(ctx.accounts.staker.key(), amount);

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.staker_ata.to_account_info(),
                    to: ctx.accounts.vault_ata.to_account_info(),
                    authority: ctx.accounts.staker.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        let mut vault = ctx.accounts.vault.load_mut()?;

        let unstake_fee = ctx.accounts.fee_vault.unstake_fee;

        if unstake_fee > 0 {
            system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: ctx.accounts.staker.to_account_info(),
                        to: ctx.accounts.fee_wallet.to_account_info(),
                    },
                ),
                unstake_fee,
            )?;
        }

        let bump = vault.bump;
        let vault_bump = bump;
        let seeds = [b"vault".as_ref(), &[vault_bump]];
        let signer = &[&seeds[..]];
        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault_ata.to_account_info(),
                    to: ctx.accounts.staker_ata.to_account_info(),
                    authority: ctx.accounts.token_vault.to_account_info(),
                },
                signer,
            ),
            amount,
        )?;

        vault.unstake(ctx.accounts.staker.key(), amount);

        Ok(())
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        let mut vault = ctx.accounts.vault.load_mut()?;
        let bump = vault.bump;
        let vault_bump = bump;

        let amount = vault.claim(ctx.accounts.staker.key());

        let seeds = [b"vault".as_ref(), &[vault_bump]];
        let signer = &[&seeds[..]];
        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault_ata.to_account_info(),
                    to: ctx.accounts.staker_ata.to_account_info(),
                    authority: ctx.accounts.token_vault.to_account_info(),
                },
                signer,
            ),
            amount,
        )?;

        Ok(())
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

    pub fn initialize_fee_vault(
        ctx: Context<InitializeFeeVault>,
        fee_wallet: Pubkey,
        stake_fee: u64,
        unstake_fee: u64,
    ) -> Result<()> {
        let fee_vault = &mut ctx.accounts.fee_vault;
        fee_vault.bump = *ctx.bumps.get("fee_vault").unwrap();
        fee_vault.fee_wallet = fee_wallet;
        fee_vault.stake_fee = stake_fee;
        fee_vault.unstake_fee = unstake_fee;
        fee_vault.authority = ctx.accounts.vault.load()?.authority;

        Ok(())
    }

    pub fn update_fee_vault(
        ctx: Context<UpdateFeeVault>,
        fee_wallet: Pubkey,
        stake_fee: u64,
        unstake_fee: u64,
    ) -> Result<()> {
        let fee_vault = &mut ctx.accounts.fee_vault;
        fee_vault.fee_wallet = fee_wallet;
        fee_vault.stake_fee = stake_fee;
        fee_vault.unstake_fee = unstake_fee;

        Ok(())
    }
}

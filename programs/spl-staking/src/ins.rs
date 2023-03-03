use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token::{Mint, Token, TokenAccount},
};
use emperor_staking::program::EmperorStaking;

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(zero)]
    pub vault: AccountLoader<'info, Vault>,

    #[account(
        seeds = [
            b"vault".as_ref(),
        ],
        bump,
    )]
    pub token_vault: SystemAccount<'info>,

    pub stake_token_mint: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::authority = token_vault,
        associated_token::mint = stake_token_mint,
    )]
    pub vault_ata: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct UpdateVault<'info> {
    #[account(mut, address = vault.load()?.authority)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub vault: AccountLoader<'info, Vault>,

    #[account(
        seeds = [
            b"vault".as_ref(),
        ],
        bump = vault.load()?.bump,
    )]
    pub token_vault: SystemAccount<'info>,

    pub stake_token_mint: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::authority = token_vault,
        associated_token::mint = stake_token_mint,
    )]
    pub vault_ata: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Fund<'info> {
    #[account(mut)]
    pub funder: Signer<'info>,

    #[account(mut)]
    pub vault: AccountLoader<'info, Vault>,

    #[account(
        seeds = [
            b"vault".as_ref(),
        ],
        bump = vault.load()?.bump,
    )]
    pub token_vault: SystemAccount<'info>,

    #[account(address = USDC)]
    pub usdc_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::authority = funder,
        associated_token::mint = usdc_mint,
    )]
    pub funder_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = funder,
        associated_token::authority = token_vault,
        associated_token::mint = usdc_mint,
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, address = vault.load()?.authority)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub vault: AccountLoader<'info, Vault>,

    #[account(
        seeds = [
            b"vault".as_ref(),
        ],
        bump = vault.load()?.bump,
    )]
    pub token_vault: SystemAccount<'info>,

    #[account(
        mut,
        associated_token::authority = authority,
        associated_token::mint = USDC,
    )]
    pub authority_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::authority = token_vault,
        associated_token::mint = USDC,
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,

    #[account(mut)]
    pub vault: AccountLoader<'info, Vault>,

    #[account(
        mut,
        seeds =[
            b"fee-vault".as_ref(),
            vault.key().as_ref(),
        ],
        bump = fee_vault.bump,

    )]
    pub fee_vault: Account<'info, FeeVault>,

    #[account(mut, address = fee_vault.fee_wallet)]
    pub fee_wallet: SystemAccount<'info>,

    #[account(
        seeds = [
            b"vault".as_ref(),
        ],
        bump = vault.load()?.bump,
    )]
    pub token_vault: SystemAccount<'info>,

    #[account(
        mut,
        associated_token::authority = staker,
        associated_token::mint = vault.load()?.stake_token_mint,
    )]
    pub staker_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::authority = token_vault,
        associated_token::mint = vault.load()?.stake_token_mint,
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct StakeWithClaim<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,

    /// CHECK:
    #[account(mut)]
    pub staker_account: AccountInfo<'info>,

    /// CHECK:
    #[account(mut)]
    pub emperor_vault: AccountInfo<'info>,

    #[account(address = vault.load()?.stake_token_mint)]
    pub stake_token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = stake_token_mint,
        associated_token::authority = emperor_vault,
    )]
    pub reward_token_vault_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub vault: AccountLoader<'info, Vault>,

    #[account(
        mut,
        seeds =[
            b"fee-vault".as_ref(),
            vault.key().as_ref(),
        ],
        bump = fee_vault.bump,

    )]
    pub fee_vault: Box<Account<'info, FeeVault>>,

    #[account(mut, address = fee_vault.fee_wallet)]
    pub fee_wallet: SystemAccount<'info>,

    #[account(
        seeds = [
            b"vault".as_ref(),
        ],
        bump = vault.load()?.bump,
    )]
    pub token_vault: SystemAccount<'info>,

    #[account(
        mut,
        associated_token::authority = staker,
        associated_token::mint = stake_token_mint,
    )]
    pub staker_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::authority = token_vault,
        associated_token::mint = stake_token_mint,
    )]
    pub vault_ata: Box<Account<'info, TokenAccount>>,

    pub emperor_program: Program<'info, EmperorStaking>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,

    #[account(mut)]
    pub vault: AccountLoader<'info, Vault>,

    #[account(
        mut,
        seeds =[
            b"fee-vault".as_ref(),
            vault.key().as_ref(),
        ],
        bump = fee_vault.bump,

    )]
    pub fee_vault: Account<'info, FeeVault>,

    #[account(mut, address = fee_vault.fee_wallet)]
    pub fee_wallet: SystemAccount<'info>,

    #[account(
        seeds = [
            b"vault".as_ref(),
        ],
        bump = vault.load()?.bump,
    )]
    pub token_vault: SystemAccount<'info>,

    #[account(
        mut,
        associated_token::authority = staker,
        associated_token::mint = vault.load()?.stake_token_mint,
    )]
    pub staker_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::authority = token_vault,
        associated_token::mint = vault.load()?.stake_token_mint,
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,

    #[account(mut)]
    pub vault: AccountLoader<'info, Vault>,

    #[account(
        seeds = [
            b"vault".as_ref(),
        ],
        bump = vault.load()?.bump,
    )]
    pub token_vault: SystemAccount<'info>,

    #[account(address = USDC)]
    pub usdc_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::authority = token_vault,
        associated_token::mint = usdc_mint,
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = staker,
        associated_token::authority = staker,
        associated_token::mint = usdc_mint,
    )]
    pub staker_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ClosePda<'info> {
    #[account(mut, address = "3qWq2ehELrVJrTg2JKKERm67cN6vYjm1EyhCEzfQ6jMd".parse::<Pubkey>().unwrap())]
    pub signer: Signer<'info>,

    /// CHECK:
    #[account(mut)]
    pub pda: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeFeeVault<'info> {
    #[account(mut, address = vault.load()?.authority)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub vault: AccountLoader<'info, Vault>,

    #[account(
        init,
        space =  FeeVault::LEN + 8,
        seeds =[
            b"fee-vault".as_ref(),
            vault.key().as_ref(),
        ],
        bump,
        payer = authority,

    )]
    pub fee_vault: Account<'info, FeeVault>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateFeeVault<'info> {
    #[account(mut, address = vault.load()?.authority)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub vault: AccountLoader<'info, Vault>,

    #[account(
        mut,
        seeds =[
            b"fee-vault".as_ref(),
            vault.key().as_ref(),
        ],
        bump = fee_vault.bump,

    )]
    pub fee_vault: Account<'info, FeeVault>,
}

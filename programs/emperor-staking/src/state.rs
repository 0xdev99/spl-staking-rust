use anchor_lang::prelude::*;

pub const MAX_NFT_PER_USER: usize = 150;

#[account]
pub struct Vault {
    pub name: String,
    pub authority: Pubkey,
    pub creator_address: Pubkey,
    pub community_wallet: Pubkey,
    pub total_earned: u64,
    pub payout_interval: u64,
    pub payout_amount: u64,
    pub total_staked: u32,
    pub total_amount: u64,
    pub stake_fee: u64,
    pub unstake_fee: u64,
    pub bump: u8,
}

impl Vault {
    pub const LEN: usize = std::mem::size_of::<Vault>();
}


/*
 * USER INFORMATION
 */
#[account(zero_copy)]
pub struct User {
    /// The user
    pub user: Pubkey,
    /// The total amount of reward pending
    pub reward_earned_pending: u64,

    pub reward_earned_claimed: u64,
    // pub mint_staked: Pubkey,
    pub staked_items: [StakedNft; MAX_NFT_PER_USER], 

    //last update time for stake/unstake
    pub last_update_time: u64,
    pub total_reward_rate: u64,
    pub mint_staked_count: u32,
}

impl User {
    pub const LEN: usize = std::mem::size_of::<User>();
}

impl Default for User {
    #[inline]
    fn default() -> User {
        User {
            user: Pubkey::default(),
            reward_earned_pending: 0,
            reward_earned_claimed: 0,
            mint_staked_count: 0,
            staked_items: [
                StakedNft::default(); 
                MAX_NFT_PER_USER
            ],
            last_update_time: 0,
            total_reward_rate: 0,
        }
    }
}

#[zero_copy]
#[derive(Debug, PartialEq)]
pub struct StakedNft {
  pub mint: Pubkey,
  pub reward_rate: u64,
  pub staked_time: u64,
}

impl StakedNft {
    pub const LEN: usize = std::mem::size_of::<StakedNft>();
}

impl Default for StakedNft {
    fn default() -> StakedNft {
        StakedNft {
            mint: Pubkey::default(),
            reward_rate: 0,
            staked_time: 0,
        }
    }
}

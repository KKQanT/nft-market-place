use anchor_lang::prelude::*;

#[account]
pub struct SellerVault {
    pub owner: Pubkey,
    pub mint_address: Pubkey,
    pub price: u64,
    pub first_creator: Pubkey
}

impl SellerVault {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 32; 
}
use anchor_lang::prelude::*;

#[account]
pub struct WhitelistedCollection {
    pub first_creator: Pubkey,
    pub collection_name: String,
}

impl WhitelistedCollection {
    pub const LEN: usize = 8 + 32 + 4 + 32;
}
use anchor_lang::prelude::*;

use crate::{state::WhitelistedCollection};

#[derive(Accounts)]
#[instruction(first_creator: Pubkey)]
pub struct WhitelistCollection<'info> {
    #[account(
        init,
        seeds=[
                b"collection", 
                first_creator.key().as_ref(),
            ],
        bump,
        payer=payer,
        space=WhitelistedCollection::LEN,
    )]
    pub whitelisted_collection: Account<'info, WhitelistedCollection>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>
}

pub fn handler(
    ctx: Context<WhitelistCollection>,
    first_creator: Pubkey,
    collection_name: String
) -> Result<()> {
    let whitelisted_collection = &mut ctx.accounts.whitelisted_collection;
    whitelisted_collection.first_creator = first_creator;
    whitelisted_collection.collection_name = collection_name;
    msg!("collection first creator: {}", whitelisted_collection.first_creator);
    msg!("collection name: {}", whitelisted_collection.collection_name);
    Ok(())
}


use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod state;
pub mod instructions;
pub mod constant;
pub mod errors;

pub use state::*;
pub use instructions::*;
pub use constant::*;
pub use errors::*;

#[program]
pub mod nft_market_place {
    use super::*;

    pub fn whitelist_collection(
        ctx: Context<WhitelistCollection>,
        first_creator: Pubkey,
        collection_name: String
    ) -> Result<()> {
        instructions::whitelist_collection::handler(
            ctx, 
            first_creator, 
            collection_name
        )
    }

    pub fn list_nft(
        ctx: Context<ListNFT>,
        mint_address: Pubkey,
        first_creator: Pubkey,
        whitelist_collection_bump: u8,
        price: u64,
    ) -> Result<()> {
            instructions::list_nft::handler(
                ctx, 
                mint_address, 
                first_creator, 
                whitelist_collection_bump,        
                price
            )
        }
}

#[derive(Accounts)]
pub struct Initialize {}

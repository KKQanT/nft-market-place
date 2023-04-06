use anchor_lang::prelude::*;
use anchor_spl::associated_token;
use anchor_spl::token::{TokenAccount, Token, self};
use mpl_token_metadata::state::{Metadata, TokenMetadataAccount};

use crate::{SellerVault, WhitelistedCollection};
use crate::constant::METADATA_PROGRAM_ID;
use crate::errors::NFTMatketPlaceProgramError;

#[derive(Accounts)]
#[instruction(
    mint_address: Pubkey,
    first_creator: Pubkey,
    whitelist_collection_bump:u8,
    price: u64,
)]
pub struct ListNFT<'info> {
    #[account(
        seeds=[
            b"collection", 
            first_creator.key().as_ref()
            ],
        bump=whitelist_collection_bump
    )]
    pub whitelisted_collection: Account<'info, WhitelistedCollection>,
    #[account(
        init,
        seeds = [
            b"seller_vault",
            mint_address.as_ref(),
            seller.key().as_ref(),
        ],
        bump,
        payer = seller,
        space = SellerVault::LEN
    )]
    pub seller_vault: Account<'info, SellerVault>,
    #[account(mut)]
    pub seller_vault_nft_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub seller_nft_token_account: Account<'info, TokenAccount>,
    ///CHECK: checked via instruction
    pub metadata_account: AccountInfo<'info>,
    ///CHECK : check via #[account(address = crate::address::METADATA_PROGRAM_ID.parse::<Pubkey>().unwrap())]
    #[account(address = METADATA_PROGRAM_ID.parse::<Pubkey>().unwrap())]
    pub token_metadata_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<ListNFT>,
    mint_address: Pubkey,
    first_creator: Pubkey,
    _whitelist_collection_bump:u8,
    price: u64,
) -> Result<()> {
    let whitelisted_collection = &ctx.accounts.whitelisted_collection;
    let seller_vault = &mut ctx.accounts.seller_vault;
    let seller_vault_nft_token_account = &ctx.accounts.seller_vault_nft_token_account;
    let seller = &ctx.accounts.seller;
    let seller_nft_token_account = &ctx.accounts.seller_nft_token_account;
    let nft_metadata_account = &ctx.accounts.metadata_account;
    let token_metadata_program = &ctx.accounts.token_metadata_program;

    if nft_metadata_account.owner.key() != token_metadata_program.key() {
        msg!("invalid nft_metadata_account owner");
        return err!(NFTMatketPlaceProgramError::UndefinedError)
    };

    let metadata_seed = &[
        b"metadata",
        token_metadata_program.key.as_ref(),
        seller_nft_token_account.mint.as_ref(),
    ];

    let (expected_metadata_key, _metadata_bump) = Pubkey::find_program_address(
        metadata_seed, 
        token_metadata_program.key
      );
    
    if  nft_metadata_account.key() != expected_metadata_key {
        msg!("invalid nft_metadata_account");
        return err!(NFTMatketPlaceProgramError::UndefinedError)
    }

    if  nft_metadata_account.data_is_empty() {
        msg!("data_is_empty");
        return err!(NFTMatketPlaceProgramError::UndefinedError)
    }

    let nft_metadata: Metadata = Metadata::from_account_info(&nft_metadata_account)?;
    let nft_first_creator = &nft_metadata.data.creators.unwrap()[0];

    if !nft_first_creator.verified {
        msg!("not verified");
        return err!(NFTMatketPlaceProgramError::UndefinedError)
    }

    if nft_first_creator.address != whitelisted_collection.first_creator {
        msg!("invalid nft_first_creator");
        return  err!(NFTMatketPlaceProgramError::UndefinedError);
    }

    let expected_seller_vault_token_account = associated_token::get_associated_token_address(
        &seller_vault.key(), 
        &mint_address
      );
    
    if seller_vault_nft_token_account.key() != expected_seller_vault_token_account {
        msg!("invalid vault_nft_token_account");
        return  err!(NFTMatketPlaceProgramError::UndefinedError);
    }

    seller_vault.owner = seller.key();
    seller_vault.mint_address = mint_address;
    seller_vault.price = price;
    seller_vault.first_creator = first_creator;

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: seller_nft_token_account.to_account_info(),
            to: seller_vault_nft_token_account.to_account_info(),
            authority: seller.to_account_info()
        },
    );

    token::transfer(cpi_ctx, 1)?;

    Ok(())
}
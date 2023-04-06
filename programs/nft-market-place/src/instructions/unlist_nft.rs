use anchor_lang::prelude::*;
use anchor_spl::{token::{TokenAccount, Token, self}, associated_token};

use crate::{WhitelistedCollection, SellerVault, NFTMatketPlaceProgramError};

#[derive(Accounts)]
#[instruction(
    mint_address: Pubkey,
    first_creator: Pubkey,
    whitelist_collection_bump:u8,
    seller_vault_bump: u8
)]
pub struct UnlistNFT<'info> {
    #[account(
        mut,
        seeds = [
            b"seller",
            mint_address.as_ref(),
            seller.key().as_ref(),
        ],
        bump=seller_vault_bump,
        close=seller
    )]
    pub seller_vault: Account<'info, SellerVault>,
    #[account(mut)]
    pub seller_vault_nft_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub seller_nft_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<UnlistNFT>,
    mint_address: Pubkey,
    _first_creator: Pubkey,
    _whitelist_collection_bump:u8,
    seller_vault_bump: u8
) -> Result<()> {
    let seller_vault = &mut ctx.accounts.seller_vault;
    let seller_vault_nft_token_account = &mut ctx.accounts.seller_vault_nft_token_account;
    let seller = &ctx.accounts.seller;
    let seller_nft_token_account = &ctx.accounts.seller_nft_token_account;
    
    if seller_vault.owner != seller.key() {
        msg!("invalid authority");
        return err!(NFTMatketPlaceProgramError::UndefinedError);
    }

    let expected_seller_token_account = associated_token::get_associated_token_address(
        &seller.key(), 
        &mint_address
    );

    let expected_seller_vault_token_account = associated_token::get_associated_token_address(
        &seller_vault.key(), 
        &mint_address
    );

    if seller_nft_token_account.key() != expected_seller_token_account {
        msg!("invalid seller_nft_token_account");
        return  err!(NFTMatketPlaceProgramError::UndefinedError);
    }

    if seller_vault_nft_token_account.key() != expected_seller_vault_token_account {
        msg!("invalid vault_nft_token_account");
        return  err!(NFTMatketPlaceProgramError::UndefinedError);
    }

    let seller_key = seller.key();

    let seller_vault_seeds = &[
        b"seller_vault",
        mint_address.as_ref(),
        seller_key.as_ref(),
        &[seller_vault_bump]
    ];

    let seller_vault_signer = [&seller_vault_seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer{
            from: seller_vault_nft_token_account.to_account_info(),
            to: seller_nft_token_account.to_account_info(),
            authority: seller_vault.to_account_info()
        },
        &seller_vault_signer
    );

    token::transfer(cpi_ctx, 1);

    let should_close_seller_token_account = {
        seller_vault_nft_token_account.reload()?;
        seller_vault_nft_token_account.amount == 0
    };

    if should_close_seller_token_account {
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::CloseAccount {
                account: seller_vault_nft_token_account.to_account_info(),
                destination: seller.to_account_info(),
                authority: seller_vault.to_account_info()
            },
            &seller_vault_signer
        );
        token::close_account(cpi_ctx)?;
      }

    Ok(())
}
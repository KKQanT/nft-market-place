use anchor_lang::prelude::*;
use anchor_spl::{token::{TokenAccount, Token, self}, associated_token};
use anchor_lang::system_program;
use crate::{SellerVault, NFTMatketPlaceProgramError};

#[derive(Accounts)]
#[instruction(
    mint_address: Pubkey,
    first_creator: Pubkey,
    seller_vault_bump: u8
)]
pub struct BuyNFT<'info> {
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
    /// CHECK: This is not dangerous because we check seller == owner via instruction
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    #[account(mut)]
    pub buyer : Signer<'info>,
    #[account(mut)]
    pub buyer_nft_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<BuyNFT>,
    mint_address: Pubkey,
    _first_creator: Pubkey,
    _whitelist_collection_bump: u8,
    seller_vault_bump: u8
) -> Result<()> {
    let seller_vault = &mut ctx.accounts.seller_vault;
    let seller_vault_nft_token_account = &mut ctx.accounts.seller_vault_nft_token_account;
    let seller = &ctx.accounts.seller;
    let buyer = &ctx.accounts.buyer;
    let buyer_nft_token_account = &ctx.accounts.buyer_nft_token_account;

    if seller_vault.owner != seller.key() {
        msg!("invalid authority");
        return err!(NFTMatketPlaceProgramError::UndefinedError);
    }

    let expected_seller_vault_token_account = associated_token::get_associated_token_address(
        &seller_vault.key(), 
        &mint_address
    );

    if seller_vault_nft_token_account.key() != expected_seller_vault_token_account {
        msg!("invalid vault_nft_token_account");
        return  err!(NFTMatketPlaceProgramError::UndefinedError);
    }

    let expected_buyer_nft_account = associated_token::get_associated_token_address(
        &buyer.key(), 
        &mint_address
    );

    if buyer_nft_token_account.key() != expected_buyer_nft_account {
        msg!("invalid buyer_nft_token_account");
        return  err!(NFTMatketPlaceProgramError::UndefinedError);
    }

    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(), 
        system_program::Transfer {
            from: buyer.to_account_info().clone(),
            to: seller.clone()
        });
    
    system_program::transfer(cpi_context, seller_vault.price)?;

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
            to: buyer_nft_token_account.to_account_info(),
            authority: seller_vault.to_account_info()
        },
        &seller_vault_signer
    );

    token::transfer(cpi_ctx, 1)?;

    let should_close_seller_vault_nft_token_account = {
        seller_vault_nft_token_account.reload()?;
        seller_vault_nft_token_account.amount == 0
    };

    if should_close_seller_vault_nft_token_account {
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

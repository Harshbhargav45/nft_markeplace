use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, transfer};

use crate::state::listing::Listing;
use crate::errors::MarketplaceError;

#[derive(Accounts)]
pub struct ListNft<'info> {
    
    #[account(mut)]
    pub seller: Signer<'info>,

    
    pub mint: Account<'info, Mint>,

    
    #[account(
        mut,
        constraint = seller_nft.amount == 1 @ MarketplaceError::InvalidAmount,
        constraint = seller_nft.owner == seller.key() @ MarketplaceError::NotNftOwner,
        constraint = seller_nft.mint == mint.key() @ MarketplaceError::InvalidMint
    )]
    pub seller_nft: Account<'info, TokenAccount>,

    
    #[account(
        init,
        payer = seller,
        seeds = [b"listing", mint.key().as_ref()],
        bump,
        space = 8 + Listing::LEN
    )]
    pub listing: Account<'info, Listing>,


    #[account(
        init,
        payer = seller,
        token::mint = mint,
        token::authority = escrow_auth,
        seeds = [b"escrow_auth", mint.key().as_ref()],
        bump
    )]
    pub nft_escrow_account: Account<'info, TokenAccount>,

    #[account(seeds = [b"escrow_auth", mint.key().as_ref()], bump)]
    pub escrow_auth: UncheckedAccount<'info>,


    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ListNft>, price: u64) -> Result<()> {
    require!(price > 0, MarketplaceError::PriceCannotBeZero);

    let listing = &mut ctx.accounts.listing;
    listing.seller = ctx.accounts.seller.key();
    listing.nft_mint = ctx.accounts.mint.key();
    listing.nft_escrow_account = ctx.accounts.nft_escrow_account.key();
    listing.price = price;
    listing.is_active = true;
    listing.bump = ctx.bumps.listing;

    
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.seller_nft.to_account_info(),
                to: ctx.accounts.nft_escrow_account.to_account_info(),
                authority: ctx.accounts.seller.to_account_info(),
            },
        ),
        1,
    )?;

    Ok(())
}

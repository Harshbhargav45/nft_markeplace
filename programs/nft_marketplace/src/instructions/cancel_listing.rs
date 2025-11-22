use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, transfer};
use crate::state::listing::Listing;
use crate::errors::MarketplaceError;

#[derive(Accounts)]
pub struct CancelListing<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut,
        seeds = [b"listing", listing.nft_mint.as_ref()],
        bump = listing.bump,
        constraint = listing.seller == seller.key(),
        constraint = listing.is_active == true
    )]
    pub listing: Account<'info, Listing>,

    
    #[account(
        mut,
        seeds = [b"escrow_auth", listing.nft_mint.as_ref()],
        bump
    )]
    pub escrow_auth: UncheckedAccount<'info>,

    #[account(mut)]
    pub nft_escrow_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = seller_nft.owner == seller.key(),
        constraint = seller_nft.mint == listing.nft_mint
    )]
    pub seller_nft: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<CancelListing>) -> Result<()> {
    let listing = &mut ctx.accounts.listing;
    listing.is_active = false;

    let bump = listing.bump;
    let seeds = &[b"escrow_auth", listing.nft_mint.as_ref(), &[bump]];
    let signer = &[&seeds[..]];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.nft_escrow_account.to_account_info(),
                to: ctx.accounts.seller_nft.to_account_info(),
                authority: ctx.accounts.escrow_auth.to_account_info(),
            },
            signer
        ),
        1
    )?;

    Ok(())
}

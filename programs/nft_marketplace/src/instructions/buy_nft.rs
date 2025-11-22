use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, transfer};
use crate::state::listing::Listing;
use crate::errors::MarketplaceError;

#[derive(Accounts)]
pub struct BuyNft<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"listing", listing.nft_mint.as_ref()],
        bump = listing.bump,
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
        constraint = buyer_nft.owner == buyer.key(),
        constraint = buyer_nft.mint == listing.nft_mint
    )]
    pub buyer_nft: Account<'info, TokenAccount>,

    
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<BuyNft>) -> Result<()> {
    let listing = &mut ctx.accounts.listing;

    if listing.seller == ctx.accounts.buyer.key() {
        return err!(MarketplaceError::CannotBuyOwnListing);
    }

    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.buyer.key(),
        &ctx.accounts.seller.key(),
        listing.price,
    );

    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.buyer.to_account_info(),
            ctx.accounts.seller.to_account_info(),
        ],
    )?;

    listing.is_active = false;

    let bump = listing.bump;
    let seeds = &[b"escrow_auth", listing.nft_mint.as_ref(), &[bump]];
    let signer = &[&seeds[..]];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.nft_escrow_account.to_account_info(),
                to: ctx.accounts.buyer_nft.to_account_info(),
                authority: ctx.accounts.escrow_auth.to_account_info(),
            },
            signer
        ),
        1
    )?;

    Ok(())
}

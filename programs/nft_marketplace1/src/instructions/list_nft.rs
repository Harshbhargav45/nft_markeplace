use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use crate::state::listing::Listing;

#[derive(Accounts)]
pub struct ListNft<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut, 
        constraint = seller_token_account.amount == 1,
        constraint = seller_token_account.owner == seller.key()
    )]
    pub seller_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = seller,
        seeds = [b"vault", mint.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = listing
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = seller,
        seeds = [b"listing", mint.key().as_ref()],
        bump,
        space = 8 + Listing::LEN
    )]
    pub listing: Account<'info, Listing>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<ListNft>, price: u64) -> Result<()> {
    let listing = &mut ctx.accounts.listing;
    listing.seller = ctx.accounts.seller.key();
    listing.mint = ctx.accounts.mint.key();
    listing.price = price;
    listing.is_active = true;
    listing.bump = ctx.bumps.listing;

   
    let cpi_accounts = Transfer {
        from: ctx.accounts.seller_token_account.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.seller.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, 1)?;

    Ok(())
}
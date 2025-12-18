use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use crate::state::listing::Listing;

#[derive(Accounts)]
pub struct BuyNft<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub seller: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"vault", mint.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = listing
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"listing", mint.key().as_ref()],
        bump = listing.bump,
        constraint = listing.seller == seller.key(),
        constraint = listing.is_active == true
    )]
    pub listing: Account<'info, Listing>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<BuyNft>, price: u64) -> Result<()> {
    let listing = &mut ctx.accounts.listing;

    require!(listing.price == price, ErrorCode::IncorrectPrice);

    let transfer_sol = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.buyer.key(),
        &ctx.accounts.seller.key(),
        listing.price,
    );
    anchor_lang::solana_program::program::invoke(
        &transfer_sol,
        &[
            ctx.accounts.buyer.to_account_info(),
            ctx.accounts.seller.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    let seeds = &[
        b"listing",
        ctx.accounts.mint.to_account_info().key.as_ref(),
        &[listing.bump],
    ];
    let signer = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.buyer_token_account.to_account_info(),
        authority: listing.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    token::transfer(cpi_ctx, 1)?;

    listing.is_active = false;

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Price mismatch")]
    IncorrectPrice,
}
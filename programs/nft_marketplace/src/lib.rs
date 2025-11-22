use anchor_lang::prelude::*;
pub mod state;
pub mod errors;
pub mod instructions;

use instructions::*;

declare_id!("FMs8619zki6RRzankv1WPt5oEUxWLQiPXQEcP6Th4WFG");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn list_nft(ctx: Context<ListNft>, price: u64) -> Result<()> {
        list_nft::handler(ctx, price)
    }

    pub fn buy_nft(ctx: Context<BuyNft>) -> Result<()> {
        buy_nft::handler(ctx)
    }

    pub fn cancel_listing(ctx: Context<CancelListing>) -> Result<()> {
        cancel_listing::handler(ctx)
    }
}

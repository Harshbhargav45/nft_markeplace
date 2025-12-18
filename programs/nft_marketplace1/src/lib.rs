use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;

use instructions::list_nft::*;
use instructions::buy_nft::*;

declare_id!("HgCDvfBZpx6EYNFkTCkpLdHGoAQPeJ698X1N6j8s4bVo");

#[program]
pub mod nft_marketplace1 {
    use super::*;

    pub fn list_nft(ctx: Context<ListNft>, price: u64) -> Result<()> {
        instructions::list_nft::handler(ctx, price)
    }

    pub fn buy_nft(ctx: Context<BuyNft>, price: u64) -> Result<()> {
        instructions::buy_nft::handler(ctx, price)
    }
}
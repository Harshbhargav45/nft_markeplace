use anchor_lang::prelude::*;

#[account]
pub struct Listing {
    pub seller: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
    pub is_active: bool,
    pub bump: u8,
}

impl Listing {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 1 + 1;
}
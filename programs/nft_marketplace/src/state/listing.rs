use anchor_lang::prelude::*;

#[account]
pub struct Listing {
    pub seller: Pubkey,
    pub nft_mint: Pubkey,
    pub nft_escrow_account: Pubkey, // <-- store escrow account here
    pub price: u64,
    pub is_active: bool,
    pub bump: u8,
}

impl Listing {
    // 32 (seller) + 32 (mint) + 32 (escrow) + 8 (price) + 1 (is_active) + 1 (bump)
    pub const LEN: usize = 32 + 32 + 32 + 8 + 1 + 1;
}

use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("Listing price must be above zero.")]
    PriceCannotBeZero,

    #[msg("NFT amount must be exactly 1.")]
    InvalidAmount,

    #[msg("You are not the owner of this NFT.")]
    NotNftOwner,

    #[msg("Mint address does not match.")]
    InvalidMint,

    #[msg("Listing is not active.")]
    ListingNotActive,

    #[msg("Only the seller can cancel listing.")]
    NotSeller,

    #[msg("You cannot buy your own listing.")]
    CannotBuyOwnListing,

    #[msg("Listing does not exist.")]
    ListingNotFound,

    #[msg("Invalid escrow authority bump.")]
    InvalidEscrowAuth,
}

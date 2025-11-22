
🖼️ Solana NFT Marketplace (Anchor)

A fully on-chain NFT listing + escrow marketplace built using Solana + Anchor.
Users can list NFTs, cancel listings, and buy NFTs securely using a PDA-based escrow.


---

🚀 Features

Feature	Description

🧾 List NFT for sale	Seller locks their NFT in program-controlled PDA escrow
💰 Buy NFT	Buyer pays SOL, receives NFT from escrow
❌ Cancel listing	Seller can cancel listing and retrieve NFT
🧠 Fully on-chain logic	No backend needed — all trustless
🔐 Secure PDA escrow	Prevents users removing NFTs mid-sale
📦 Anchor-powered program	Reliable, testable, auditable



---

📦 Program Architecture

🔐 PDA Accounts

PDA	Purpose

listing	Stores price, seller pubkey, NFT mint, status
escrow_auth	PDA authority controlling the escrow
nft_escrow_account	Token account holding the NFT


🧠 Instructions

Instruction	Who	Description

list_nft(price)	Seller	Create listing + escrow NFT
cancel_listing()	Seller	Cancel listing + return NFT
buy_nft()	Buyer	Purchase NFT + send SOL to seller



---

🏗️ Project Structure

programs/
  nft_marketplace/
    src/
      lib.rs                # Anchor program entry
      state/
        listing.rs          # Listing struct
      instructions/
        list_nft.rs         # List NFT instruction
        buy_nft.rs          # Buy NFT logic
        cancel_listing.rs   # Cancel listing logic
      errors.rs             # Custom program errors
app/
  (optional frontend)


---

🛠️ Setup & Deployment

📌 Prerequisites

Install:

Solana CLI v2+
Anchor v0.32+
Rust nightly
Node.js (optional for frontend)

🔧 Build & Deploy Locally

anchor build
anchor deploy

🧪 Test (Optional)

anchor test


---

🔗 API: How to Use the Program

✳️ List NFT

await program.methods
  .listNft(new BN(price))
  .accounts({
     seller,
     sellerNft,
     mint,
     listing,
     escrowAuth,
     nftEscrowAccount,
     tokenProgram,
     systemProgram,
  })
  .rpc();

💎 Buy NFT

await program.methods
  .buyNft()
  .accounts({
     buyer,
     buyerNft,
     listing,
     escrowAuth,
     nftEscrowAccount,
     seller,
     tokenProgram,
     systemProgram,
  })
  .rpc();

❌ Cancel Listing

await program.methods
  .cancelListing()
  .accounts({
     seller,
     listing,
     escrowAuth,
     nftEscrowAccount,
     sellerNft,
     tokenProgram,
  })
  .rpc();


---

🛡️ Security & Assurance

✔️ NFT locked in escrow PDA prevents seller from scamming buyers
✔️ Ownership checks enforced before listing
✔️ Buyer & seller payments executed atomically
✔️ Only seller can cancel their listing
✔️ PDAs guarantee safe authority control


---

🌐 Optional Future Upgrades

🔮 Royalties from Metadata Program
🧺 Bidding & Auctions
📈 Floor price + analytics
⭐ Favorites watchlist
🔐 Whitelist creators only


---

🤝 Contributions & License

Open to contributions! PRs are welcome.
Licensed under MIT.

📌 Built with ❤️ using Solana & Anchor


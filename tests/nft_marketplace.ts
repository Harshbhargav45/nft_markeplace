import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("nft_marketplace", function () {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);
  const program = anchor.workspace.NftMarketplace as Program;

  const seller = provider.wallet.payer;
  const buyer = Keypair.generate();

  let mintPubkey: PublicKey;
  let sellerAta: PublicKey;
  let buyerAta: PublicKey;

  it("Create Fake NFT Mint", async function () {
    this.timeout(60000);

    await provider.connection.requestAirdrop(buyer.publicKey, 2_000_000_000);

    mintPubkey = await createMint(
      provider.connection,
      seller,
      seller.publicKey,
      seller.publicKey,
      0
    );

    const sellerATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      seller,
      mintPubkey,
      seller.publicKey
    );
    sellerAta = sellerATA.address;

    await mintTo(provider.connection, seller, mintPubkey, sellerAta, seller, 1);

    console.log("\n🎨 Created Fake NFT:", mintPubkey.toBase58());
  });

  it("List NFT", async function () {
    this.timeout(60000);

    const [listing] = PublicKey.findProgramAddressSync(
      [Buffer.from("listing"), mintPubkey.toBuffer()],
      program.programId
    );
    const [escrowAuth] = PublicKey.findProgramAddressSync(
      [Buffer.from("escrow_auth"), mintPubkey.toBuffer()],
      program.programId
    );

    await program.methods
      .listNft(new anchor.BN(1_000_000_000))
      .accounts({
        seller: seller.publicKey,
        mint: mintPubkey,
        sellerNft: sellerAta,
        listing,
        nftEscrowAccount: escrowAuth,
        escrowAuth,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("🛒 Listed NFT");
  });

  it("Buyer Pays and Buys NFT", async function () {
    this.timeout(60000);

    const receiverATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      buyer,
      mintPubkey,
      buyer.publicKey
    );
    buyerAta = receiverATA.address;

    const [listing] = PublicKey.findProgramAddressSync(
      [Buffer.from("listing"), mintPubkey.toBuffer()],
      program.programId
    );
    const [escrowAuth] = PublicKey.findProgramAddressSync(
      [Buffer.from("escrow_auth"), mintPubkey.toBuffer()],
      program.programId
    );

    await program.methods
      .buyNft()
      .accounts({
        buyer: buyer.publicKey,
        mint: mintPubkey,
        buyerNft: buyerAta,
        listing,
        escrowAuth,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([buyer])
      .rpc();

    console.log("Buyer Bought NFT!");
  });
});

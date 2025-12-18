import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftMarketplace1 } from "../target/types/nft_marketplace1";
import { 
  createMint, 
  getOrCreateAssociatedTokenAccount, 
  mintTo, 
  TOKEN_PROGRAM_ID 
} from "@solana/spl-token";
import { assert } from "chai";

describe("nft_marketplace1", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.NftMarketplace1 as Program<NftMarketplace1>;

  const seller = anchor.web3.Keypair.generate();
  const buyer = anchor.web3.Keypair.generate();
  
  let mint: anchor.web3.PublicKey;
  let sellerTokenAccount: anchor.web3.PublicKey;
  let buyerTokenAccount: anchor.web3.PublicKey;
  let listingPda: anchor.web3.PublicKey;
  let vaultPda: anchor.web3.PublicKey;

  const price = new anchor.BN(100000000);

  it("Setup: Mint NFT and Fund Accounts", async () => {
    const amount = 0.5 * anchor.web3.LAMPORTS_PER_SOL;
    
    const transaction = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: provider.publicKey,
        toPubkey: seller.publicKey,
        lamports: amount,
      }),
      anchor.web3.SystemProgram.transfer({
        fromPubkey: provider.publicKey,
        toPubkey: buyer.publicKey,
        lamports: amount,
      })
    );

    await provider.sendAndConfirm(transaction);

    mint = await createMint(
      provider.connection,
      seller,
      seller.publicKey,
      null,
      0
    );

    const sellerTa = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      seller,
      mint,
      seller.publicKey
    );
    sellerTokenAccount = sellerTa.address;

    await mintTo(
      provider.connection,
      seller,
      mint,
      sellerTokenAccount,
      seller,
      1
    );
  });

  it("List NFT", async () => {
    const [listing] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("listing"), mint.toBuffer()],
      program.programId
    );
    listingPda = listing;

    const [vault] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), mint.toBuffer()],
      program.programId
    );
    vaultPda = vault;

    await program.methods
      .listNft(price)
      .accounts({
        seller: seller.publicKey,
        mint: mint,
        sellerTokenAccount: sellerTokenAccount,
        vault: vaultPda,
        listing: listingPda,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([seller])
      .rpc();

    const vaultAccount = await provider.connection.getTokenAccountBalance(vaultPda);
    assert.equal(vaultAccount.value.uiAmount, 1);
  });

  it("Buy NFT", async () => {
    const buyerTa = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      buyer,
      mint,
      buyer.publicKey
    );
    buyerTokenAccount = buyerTa.address;

    await program.methods
      .buyNft(price)
      .accounts({
        buyer: buyer.publicKey,
        seller: seller.publicKey,
        mint: mint,
        buyerTokenAccount: buyerTokenAccount,
        vault: vaultPda,
        listing: listingPda,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([buyer])
      .rpc();

    const buyerBalance = await provider.connection.getTokenAccountBalance(buyerTokenAccount);
    assert.equal(buyerBalance.value.uiAmount, 1);

    const listingAccount = await program.account.listing.fetch(listingPda);
    assert.isFalse(listingAccount.isActive);
  });
});
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ethers } from "ethers";
import { DeoppContract } from "../target/types/deopp_contract";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";

const { PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram } = anchor.web3;

describe("deopp-contract", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.DeoppContract as Program<DeoppContract>;
  const tokenMint = new PublicKey(
    "2f4ygn6Qtqd7TN8zEYjNPEhRAo4SMDycBxYQ1fRbwvZ5"
  );
  const giveawayId = ethers.Wallet.createRandom().address;
  console.log("giveawayId==>", giveawayId);

  it("createGiveaway", async () => {
    // Add your test here.

    const [giveaway_pool] = PublicKey.findProgramAddressSync(
      [ethers.toBeArray(giveawayId)],
      program.programId
    );

    const [tokenPool] = PublicKey.findProgramAddressSync(
      [new PublicKey(tokenMint).toBytes()],
      program.programId
    );

    const fromAccount = getAssociatedTokenAddressSync(
      tokenMint,
      provider.wallet.publicKey
    );

    const tx = await program.methods
      .createGiveaway({
        giveawayId: Array.from(ethers.toBeArray(giveawayId)),
        receiver: [provider.wallet.publicKey],
        amount: new anchor.BN(200000),
      })
      .accounts({
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
        payer: provider.wallet.publicKey,
        giveawayPool: giveaway_pool,
        fromAccount,
        tokenMint,
        tokenPool,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("receiveGiveaway", async () => {
    const [giveaway_pool] = PublicKey.findProgramAddressSync(
      [ethers.toBeArray(giveawayId)],
      program.programId
    );

    const [tokenPool] = PublicKey.findProgramAddressSync(
      [new PublicKey(tokenMint).toBytes()],
      program.programId
    );

    const toAccount = getAssociatedTokenAddressSync(
      tokenMint,
      provider.wallet.publicKey
    );

    const tx = await program.methods
      .receiveGiveaway(Array.from(ethers.toBeArray(giveawayId)))
      .accounts({
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        payer: provider.wallet.publicKey,
        giveawayPool: giveaway_pool,
        toAccount,
        tokenMint,
        tokenPool,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});

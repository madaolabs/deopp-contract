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

  it("createGiveaway", async () => {
    // Add your test here.
    const giveawayId = ethers.Wallet.createRandom().address;

    const [giveaway_pool] = PublicKey.findProgramAddressSync(
      [ethers.toBeArray(giveawayId)],
      program.programId
    );
    const tokenMint = new PublicKey(
      "HACxPsSXREaQzrcaVamHj4eK58BuMEZxfATk188VcLzb"
    );

    const [tokenPool] = PublicKey.findProgramAddressSync(
      [provider.wallet.publicKey.toBytes(), new PublicKey(tokenMint).toBytes()],
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
});

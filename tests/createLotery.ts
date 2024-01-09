import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solotery } from "../target/types/solotery";
import { SystemProgram, PublicKey } from "@solana/web3.js";

describe("Register a business", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.Solotery as Program<Solotery>;
  const programId = program.programId;

  it("Is initialized!", async () => {
    const solotery = PublicKey.findProgramAddressSync(
      [Buffer.from("SOLotery", "utf8")],
      programId
    )[0];

    const tx = await program.methods
      .createStake()
      .accounts({
        solotery: solotery,
        signer: payer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    console.log("Transaction signature", tx);
  });
});

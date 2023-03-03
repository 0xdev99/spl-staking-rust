import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SplStaking } from "../target/types/spl_staking";

describe("spl-staking", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SplStaking as Program<SplStaking>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

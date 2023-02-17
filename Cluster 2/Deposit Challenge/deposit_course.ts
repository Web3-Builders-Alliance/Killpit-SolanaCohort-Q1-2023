import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DepositCourse } from "../target/types/deposit_course";

describe("deposit_course", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DepositCourse as Program<DepositCourse>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

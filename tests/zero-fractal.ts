import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ZeroFractal } from "../target/types/zero_fractal";
import { assert } from "chai";

describe("zero-fractal", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ZeroFractal as Program<ZeroFractal>;

  it("Create a small fractal on chain", async () => {
    const size = new anchor.BN(2);
    const x = new anchor.BN("7714059436743860000");
    const y = new anchor.BN("9940888892175165000");
    const zoom = new anchor.BN("11458566504308191000");

    const fractalKp = anchor.web3.Keypair.generate();

    await program.methods
      .createFractalOnChain(size, x, y, zoom)
      .accounts({ fractal: fractalKp.publicKey })
      .signers([fractalKp])
      .preInstructions([
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
          units: 1_400_000, // Max possible CU
        }),
      ])
      .rpc();
  });

  it("Fail to create a bigger fractal on chain", async () => {
    const size = new anchor.BN(3);
    const x = new anchor.BN("7714059436743860000");
    const y = new anchor.BN("9940888892175165000");
    const zoom = new anchor.BN("11458566504308191000");

    const fractalKp = anchor.web3.Keypair.generate();

    try {
      await program.methods
        .createFractalOnChain(size, x, y, zoom)
        .accounts({ fractal: fractalKp.publicKey })
        .signers([fractalKp])
        .preInstructions([
          anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
            units: 1_400_000, // Max possible CU
          }),
        ])
        .rpc();
      assert(false);
    } catch (err) {}
  });
});

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaZeroCopyToolkit } from "../target/types/solana_zero_copy_toolkit";

describe("solana_zero_copy_toolkit", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SolanaZeroCopyToolkit as Program<SolanaZeroCopyToolkit>;

  it("is initialized!", async () => {
    // Test logic here
  });
});

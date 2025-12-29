import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VotingProgram } from "../target/types/voting_program";

describe("voting-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.VotingProgram as Program<VotingProgram>;

  it("Is initialized!", async () => {
    // Students will write test cases here
  });
});

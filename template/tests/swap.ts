import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Swap } from "../target/types/swap";

describe("swap", async () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.Swap as Program<Swap>;

    it("Is initialized!", async () => {
        const tx = await program.methods.initialize().rpc();
        console.log("Transaction signature:", tx);
    });
});

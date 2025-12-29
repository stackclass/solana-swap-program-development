# Anchor First Try

This stage guides you through creating and deploying your first Anchor program. You will experience the complete development workflow from initialization to on-chain interaction.

## Initializing a New Project

Begin by creating a new Anchor project using the CLI. This scaffold provides a complete working structure with build configuration, testing setup, and example code:

```bash
anchor init my-first-program
cd my-first-program
```

The command creates a directory structure with `programs/` containing your Rust code, `tests/` for integration tests, `migrations/` for deployment scripts, and configuration files including `Anchor.toml` and `Cargo.toml`.

Examine the generated files to understand the project structure. The `Anchor.toml` file configures your deployment target and wallet. The workspace `Cargo.toml` lists all programs in the project. Individual program `Cargo.toml` files specify dependencies.

## Understanding the Default Program

Open the generated lib.rs file in the programs directory. You will see a minimal Anchor program with a default instruction:

```rust
use anchor_lang::prelude::*;

declare_id!("YourProgramIDHere");

#[program]
pub mod my_first_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub data_account: Account<'info, Data>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Data {
    pub value: u64,
}
```

This code demonstrates Anchor's key components. The `#[program]` attribute marks the module containing instruction handlers. Each public function becomes an instruction callable from the client. The `#[derive(Accounts)]` struct validates and deserializes incoming accounts. The `#[account]` macro defines on-chain data structures.

## Building the Program

Compile your program to verify everything is configured correctly:

```bash
anchor build
```

This command compiles the Rust code to a shared object file and generates TypeScript type definitions in the `target/types/` directory. Build errors appear here if your code has syntax issues or type mismatches.

The build process also creates a keypair file in `target/deploy/` if one does not exist. This keypair corresponds to your program's on-chain address specified in `Anchor.toml`.

## Deploying to Devnet

Before deploying, ensure your Solana config points to devnet and you have a funded keypair:

```bash
solana config set --url devnet
solana balance
```

If your balance is zero, request airdrop:

```bash
solana airdrop 2
```

Deploy your compiled program:

```bash
anchor deploy
```

The deployment process uploads your program bytecode to the blockchain and sets the program executable. The output shows your program's ID and transaction signature. Save this ID for later client interactions.

## Interacting with Your Program

Anchor generates TypeScript client bindings during build. Create a simple test to interact with your deployed program:

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MyFirstProgram } from "../target/types/my_first_program";

describe("my-first-program", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.MyFirstProgram as Program<MyFirstProgram>;

    it("Calls initialize instruction", async () => {
        const dataAccount = anchor.web3.Keypair.generate();
        
        await program.methods
            .initialize()
            .accounts({
                dataAccount: dataAccount.publicKey,
                user: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([dataAccount])
            .rpc();

        console.log("Data account created!");
    });
});
```

Run the test:

```bash
anchor test
```

## Verifying On-Chain State

After successful execution, verify your program created the account correctly:

```bash
solana account YourDataAccountAddress
```

The output shows account data including lamport balance and stored data. For the initialize instruction, you should see your account with initialized data.

## Common Issues and Solutions

**Build failures** often stem from missing dependencies or version mismatches. Ensure your Cargo.toml specifies compatible versions and run `cargo update` to refresh dependencies.

**Deployment timeouts** may occur on congested networks. Try again during lower-traffic periods or specify a higher compute unit limit.

**Test failures** typically result from incorrect account addresses or insufficient lamports for rent. Double-check account addresses in your test and ensure your wallet has sufficient balance.

**Program ID mismatch** errors occur when the ID in your lib.rs differs from the deployed program. Update the `declare_id!` macro or redeploy to match.

## Next Steps

Your first Anchor program is deployed and working. The subsequent stages build on this foundation, adding increasingly complex functionality. Experiment by modifying the default program before proceeding to add custom logic.

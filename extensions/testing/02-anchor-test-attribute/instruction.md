# Anchor Test Attribute

This stage teaches Anchor's program testing framework using `#[program_test]` and the test validator.

## The #[program_test] Attribute

Anchor provides a test attribute that creates a local validator and tests your program:

```rust
use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

#[program_test]
mod tests {
    use super::*;
    
    #[test]
    async fn test_create_offer() {
        // Test code here
    }
}
```

The `#[program_test]` attribute:
- Creates a local Solana validator
- Deploys your program
- Provides test helpers

## Setting Up the Test

Initialize the test environment:

```rust
use anchor_lang::prelude::*;
use anchor_lang::solana_program::genesis_config::GenesisConfig;

#[program_test]
async fn test_make_offer(
    // Setup function runs before tests
    mut test_ctx: ProgramTestContext,
) {
    // Configure the test
    test_ctx.configure_cluster(Cluster::Localnet);
    
    // Add your program
    test_ctx.add_program(
        "swap", 
        program_id, 
        None
    );
    
    // Start the validator
    test_ctx.start().await;
}
```

## ProgramTestContext

The `ProgramTestContext` provides:
- **banks_client**: For interacting with the validator
- **payer**: Keypair paying for transactions
- **program_id**: Your program's ID

```rust
let program_id = Pubkey::new_unique();
let banks_client = &test_ctx.banks_client;
let payer = &test_ctx.payer;
```

## Creating Test Accounts

Create accounts for testing:

```rust
let lamports = 1_000_000_000;
let rent = Rent::default();
let space = 100;

let account = Account::new(lamports, space, &program_id);
banks_client.process_transaction(&Transaction::new_with_payer(
    &[SystemInstruction::create_account(
        &payer.pubkey(),
        &account_address,
        lamports,
        space,
        &program_id,
    )],
    Some(&payer.pubkey()),
)).await?;
```

## Calling Program Instructions

Use the banks client to call your program:

```rust
let instruction = anchor_lang::Instruction {
    program_id,
    accounts: vec![
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(account_address, false),
    ],
    data: vec![],
};

banks_client.process_transaction(&Transaction::new_signed_with_payer(
    &[instruction],
    Some(&payer.pubkey()),
    &[payer],
    None,
)).await?;
```

## Practical Exercise

Set up a basic Anchor test for your swap program. Create the ProgramTestContext, add your program, and verify it deploys successfully.

## Key Takeaways

#[program_test] creates a test environment. ProgramTestContext provides banks client and payer. Use banks_client to send transactions. The validator runs locally for fast testing.

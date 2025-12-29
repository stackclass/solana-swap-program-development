# Test Setup and Teardown

This stage teaches patterns for setting up test environments and managing resources in Anchor tests.

## Test Setup Patterns

Good tests need consistent setup. Create helper functions:

```rust
#[program_test]
async fn setup_test_environment(
    mut test_ctx: ProgramTestContext,
) -> (ProgramTestContext, Keypair, Keypair) {
    // Add programs
    test_ctx.add_program("swap", swap::ID, None);
    
    // Create test users
    let maker = Keypair::new();
    let taker = Keypair::new();
    
    // Fund test users
    test_ctx.banks_client.process_transaction(&Transaction::new_with_payer(
        &[
            system_instruction::transfer(
                &test_ctx.payer.pubkey(),
                &maker.pubkey(),
                10_000_000_000,
            ),
            system_instruction::transfer(
                &test_ctx.payer.pubkey(),
                &taker.pubkey(),
                10_000_000_000,
            ),
        ],
        Some(&test_ctx.payer.pubkey()),
    )).await.unwrap();
    
    test_ctx.start().await;
    
    (test_ctx, maker, taker)
}
```

## Creating Token Mints

Create mints for testing:

```rust
async fn create_token_mint(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    mint: &Keypair,
) -> Result<(), BanksClientError> {
    let mint_account = Account::new_data_with_space(
        &Rent::default().minimum_balance(MINT_SIZE),
        &Mint {
            mint_authority: Some(payer.pubkey()),
            supply: 0,
            decimals: 6,
            is_initialized: true,
            freeze_authority: Some(payer.pubkey()),
        },
        MINT_SIZE,
        &spl_token::ID,
    )?;
    
    let transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &mint.pubkey(),
                1_000_000_000,
                MINT_SIZE as u64,
                &spl_token::ID,
            ),
            initialize_mint(&spl_token::ID, &mint.pubkey(), &payer.pubkey(), None, 6).unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    
    banks_client.process_transaction(&transaction).await
}
```

## Creating Token Accounts

Create associated token accounts for testing:

```rust
async fn create_associated_token_account(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    owner: &Pubkey,
    mint: &Pubkey,
) -> Result<Pubkey, BanksClientError> {
    let ata = get_associated_token_address(owner, mint);
    
    let transaction = Transaction::new_with_payer(
        &[create_associated_token_account(
            &payer.pubkey(),
            &ata,
            owner,
            mint,
        )],
        Some(&payer.pubkey()),
    );
    
    banks_client.process_transaction(&transaction).await?;
    
    Ok(ata)
}
```

## Minting Tokens for Tests

Mint tokens to test accounts:

```rust
async fn mint_tokens(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    mint: &Pubkey,
    recipient: &Pubkey,
    amount: u64,
) -> Result<(), BanksClientError> {
    let recipient_ata = get_associated_token_address(recipient, mint);
    
    let transaction = Transaction::new_with_payer(
        &[mint_to(
            &spl_token::ID,
            mint,
            &recipient_ata,
            &payer.pubkey(),
            &[],
            amount,
        )],
        Some(&payer.pubkey()),
    );
    
    banks_client.process_transaction(&transaction).await
}
```

## Teardown Considerations

Local validator tests don't require explicit teardown—the validator stops when the test context is dropped. However, clean test functions help:

```rust
#[program_test]
async fn test_make_offer(mut test_ctx: ProgramTestContext) {
    // Setup
    let (mut test_ctx, maker, taker) = setup_test_environment(test_ctx).await;
    
    // Test
    // ...
    
    // No explicit teardown needed
}
```

## Practical Exercise

Create helper functions for setting up your test environment. Create a mint, associated token accounts, and mint tokens. Use these helpers in a complete test scenario.

## Key Takeaways

Helper functions simplify test setup. Create reusable setup code for mints and accounts. Fund test users with SOL for transactions. The test context handles cleanup automatically.

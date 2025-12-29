# Comprehensive Tests

This stage teaches how to write comprehensive tests covering both happy paths and error cases for your swap program.

## Test Structure

Organize tests by instruction and scenario:

```rust
#[program_test]
mod swap_tests {
    use super::*;
    
    mod make_offer_tests {
        use super::*;
        
        #[test]
        async fn test_successful_make_offer() {
            // Test happy path
        }
        
        #[test]
        async fn test_make_offer_with_zero_amount() {
            // Test error case
        }
        
        #[test]
        async fn test_make_offer_insufficient_balance() {
            // Test error case
        }
    }
    
    mod take_offer_tests {
        use super::*;
        
        #[test]
        async fn test_successful_take_offer() {
            // Test happy path
        }
        
        #[test]
        async fn test_take_offer_self_trade() {
            // Test error case
        }
    }
}
```

## Happy Path Test

Test a complete successful flow:

```rust
#[test]
async fn test_successful_make_offer() {
    let (mut test_ctx, maker, _) = setup_test_environment().await;
    
    // Create token mint and accounts
    let (mint, maker_token_account) = setup_token_for_maker(&mut test_ctx, &maker, 1_000_000).await;
    
    // Create offer
    let offer_id = 1;
    let offered_amount = 100_000;
    let wanted_amount = 50_000;
    
    let instruction = make_offer_instruction(
        &maker,
        &offer_id,
        &offered_amount,
        &wanted_amount,
        &mint,
    );
    
    test_ctx.banks_client.process_transaction(&transaction).await.unwrap();
    
    // Verify offer was created
    let offer_account = test_ctx.banks_client.get_account(offer_address).await.unwrap();
    assert!(offer_account.is_some());
}
```

## Error Case Test

Test that invalid inputs are rejected:

```rust
#[test]
async fn test_make_offer_zero_amount() {
    let (mut test_ctx, maker, _) = setup_test_environment().await;
    
    // Try to create offer with zero amount
    let instruction = make_offer_instruction(
        &maker,
        &1,
        &0,  // Invalid: zero amount
        &50_000,
        &mint,
    );
    
    let result = test_ctx.banks_client.process_transaction(&transaction).await;
    
    assert!(result.is_err());
}
```

## State Verification

Verify state changes after operations:

```rust
#[test]
async fn test_vault_balance_after_deposit() {
    let (mut test_ctx, maker, _) = setup_test_environment().await;
    let (mint, maker_token_account) = setup_token_for_maker(&mut test_ctx, &maker, 1_000_000).await;
    
    // Create offer
    make_offer(&mut test_ctx, &maker, &mint, 100_000, 50_000).await;
    
    // Verify maker's balance decreased
    let maker_token_after = get_token_balance(&mut test_ctx, &maker_token_account).await;
    assert_eq!(maker_token_after, 900_000);
    
    // Verify vault was created and has correct balance
    let vault_address = get_vault_address(&maker, &mint, &1);
    let vault_balance = get_token_balance(&mut test_ctx, &vault_address).await;
    assert_eq!(vault_balance, 100_000);
}
```

## Complete Swap Flow Test

Test the entire swap lifecycle:

```rust
#[test]
async fn test_complete_swap_flow() {
    let (mut test_ctx, maker, taker) = setup_test_environment().await;
    
    // Setup tokens
    let (mint_a, maker_token_a) = setup_token_for_maker(&mut test_ctx, &maker, 1_000_000).await;
    let (mint_b, taker_token_b) = setup_token_for_taker(&mut test_ctx, &taker, 1_000_000).await;
    
    // Maker creates offer
    make_offer(&mut test_ctx, &maker, &mint_a, 100_000, 50_000).await;
    
    // Verify offer state
    assert!(offer_is_open(&mut test_ctx, &maker, &1).await);
    
    // Taker takes offer
    take_offer(&mut test_ctx, &taker, &maker, &mint_a, &mint_b, &1).await;
    
    // Verify token transfers
    let maker_balance_b = get_token_balance(&mut test_ctx, &maker_token_b).await;
    assert_eq!(maker_balance_b, 50_000);
    
    let taker_balance_a = get_token_balance(&mut test_ctx, &taker_token_a).await;
    assert_eq!(taker_balance_a, 100_000);
    
    // Verify offer is closed
    assert!(offer_is_closed(&mut test_ctx, &maker, &1).await);
}
```

## Test Coverage Goals

Aim for comprehensive coverage:

- Happy path for each instruction
- All validation error cases
- Token balance verification
- Account state changes
- Complete end-to-end flows

## Practical Exercise

Write comprehensive tests for your swap program. Cover happy paths, error cases, state verification, and complete flows. Aim for high test coverage.

## Key Takeaways

Organize tests by instruction and scenario. Test happy paths first. Test error cases with invalid inputs. Verify state changes after operations. Test complete end-to-end flows. Aim for comprehensive coverage.

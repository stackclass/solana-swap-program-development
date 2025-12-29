## Implement Partial Fills

In this advanced stage, you'll add partial fill support, allowing offers to be accepted in multiple transactions instead of all at once.

## Understanding Partial Fills

Partial fills enable:
- **Liquidity Flexibility**: Large offers can be filled by multiple smaller takers
- **Risk Distribution**: Reduces exposure for both makers and takers
- **Better UX**: Takers don't need to match the full offer amount
- **Market Efficiency**: Improves capital utilization

## Prerequisite Reading

- **Fractional Ownership**: Understand how to track partial ownership in DeFi protocols
- **State Updates**: Learn about updating account state in Anchor
- **Atomic Operations**: Review atomic transaction guarantees

## Implementation

### 1. Update Offer Structure

Add filled amount tracking:

```rust
#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_a_offered_amount: u64,
    pub token_a_filled_amount: u64,  // Track filled amount
    pub token_b_wanted_amount: u64,
    pub token_b_filled_amount: u64,  // Track filled amount
    pub bump: u8,
}
```

### 2. Update TakeOffer Function

Add amount parameter and partial fill logic:

```rust
pub fn take_offer(
    context: Context<TakeOffer>,
    fill_amount: u64,
) -> Result<()> {
    let offer = &mut context.accounts.offer;
    
    // Calculate proportional amounts
    let token_a_fill = fill_amount;
    let token_b_fill = (fill_amount * offer.token_b_wanted_amount) / offer.token_a_offered_amount;
    
    // Check if fill is valid
    require!(
        offer.token_a_filled_amount + token_a_fill <= offer.token_a_offered_amount,
        SwapError::InsufficientOfferAmount
    );
    
    // Transfer tokens
    // ... transfer token_a_fill from vault to taker
    // ... transfer token_b_fill from taker to maker
    
    // Update filled amounts
    offer.token_a_filled_amount += token_a_fill;
    offer.token_b_filled_amount += token_b_fill;
    
    // Check if offer is fully filled
    if offer.token_a_filled_amount == offer.token_a_offered_amount {
        // Close offer and vault
    }
    
    Ok(())
}
```

### 3. Close Offer When Fully Filled

```rust
if offer.token_a_filled_amount == offer.token_a_offered_amount {
    // Close offer account
    // Close vault account
}
```

## Test Cases

| Test | Expected Result | Purpose |
|------|-----------------|---------|
| Partial fill | Correct amounts transferred | Validates partial fill logic |
| Overfill rejection | Error returned | Confirms amount limits |
| Full fill | Offer closed | Verifies completion logic |
| Multiple partial fills | Cumulative tracking | Confirms state persistence |

## Notes

- Use proportional calculations to maintain exchange rate
- Track both filled amounts for transparency
- Consider adding a minimum fill amount to prevent dust
- Fully filled offers should be closed to free up rent
- Partial fills increase complexity but improve UX

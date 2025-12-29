# Reentrancy Protection

This stage teaches specific techniques to protect your swap program from reentrancy attacks.

## Understanding Reentrancy in Solana

While Solana lacks Ethereum-style smart contract callbacks, reentrancy is still possible through Cross-Program Invocation. If your program calls another program that can invoke your program again, recursive calls may occur.

In the swap context, consider this scenario: Your take_offer function calls the Token Program to transfer payment to the maker. If the Token Program (or any program it calls) could somehow invoke your take_offer again before the first invocation completes, reentrancy could occur.

## The Checks-Effects-Interactions Pattern

The classic mitigation is the checks-effects-interactions pattern:

1. **Checks**: Validate all inputs and caller permissions
2. **Effects**: Update all internal state
3. **Interactions**: Make external calls (CPI)

```rust
pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
    // 1. CHECKS: Validate everything first
    require!(ctx.accounts.taker.key() != ctx.accounts.maker.key(), ErrorCode::SelfTrade);
    require!(ctx.accounts.vault.amount >= ctx.accounts.offer.token_a_offered_amount, ErrorCode::InsufficientFunds);
    
    // 2. EFFECTS: Update all state
    let offer = &mut ctx.accounts.offer;
    offer.status = OfferStatus::Taken;
    
    // 3. INTERACTIONS: Make external calls last
    transfer_tokens_to_maker()?;
    transfer_tokens_from_vault()?;
    
    Ok(())
}
```

## State Flags

Use state flags to prevent reentrant calls:

```rust
#[account]
pub struct Offer {
    pub id: u64,
    pub maker: Pubkey,
    pub status: OfferStatus,
    // ... other fields
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OfferStatus {
    Open,
    Taken,
}

pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
    // Check status first
    require!(ctx.accounts.offer.status == OfferStatus::Open, ErrorCode::OfferAlreadyTaken);
    
    // Set status before any CPI
    ctx.accounts.offer.status = OfferStatus::Taken;
    
    // Now CPI is safe - reentrant calls will fail the status check
    perform_token_transfers()?;
    
    Ok(())
}
```

## Limiting External Calls

Minimize the attack surface by reducing external calls:

```rust
// Collect all data needed before making CPI calls
let payment_amount = ctx.accounts.offer.token_b_wanted_amount;
let vault_balance = ctx.accounts.vault.amount;

// Perform all validations with local data
require!(payment_amount > 0, ErrorCode::InvalidAmount);

// Now make the minimum necessary CPI calls
```

## Avoiding Callback Patterns

Be cautious of patterns that could enable callbacks:

```rust
// AVOID: External calls in loops that modify state
for i in 0..n {
    external_call()?;
    state[i] = updated;  // Reentrant call could see inconsistent state
}

// BETTER: Update all state first, then make single external call
for i in 0..n {
    state[i] = updated;
}
external_call()?;
```

## Practical Exercise

Review your take_offer implementation. Ensure state updates happen before any CPI calls. Add status checking to prevent double-taking an offer. Verify no loop patterns could expose inconsistent state to reentrant calls.

## Key Takeaways

Follow checks-effects-interactions: validate, update state, then call externally. Use status flags to block reentrant calls. Minimize external calls and collect data before CPI. Avoid modifying state after external calls.

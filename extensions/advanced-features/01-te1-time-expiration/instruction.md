## Implement Time-Based Offer Expiration

In this advanced stage, you'll add time-based expiration to swap offers, allowing offers to automatically expire after a specified duration.

## Understanding Offer Expiration

Offer expiration provides several benefits:
- **Price Protection**: Prevents stale offers from being accepted at outdated prices
- **Risk Management**: Limits exposure to market volatility
- **User Control**: Gives makers control over offer validity period
- **Protocol Efficiency**: Automatically cleans up old offers

## Prerequisite Reading

- **Solana Clock**: Read about the `Clock` sysvar in the [Solana Sysvar Documentation](https://solana.com/docs/core/sysvars)
- **Time-Based Logic**: Learn about time-based smart contract logic in the [Anchor Documentation](https://www.anchor-lang.com/docs/account-constraints#sysvar)
- **Unix Timestamps**: Understand Unix timestamps for time calculations

## Implementation

### 1. Update Offer Structure

Add expiration time to the Offer account:

```rust
#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_b_wanted_amount: u64,
    pub expires_at: i64,  // Unix timestamp
    pub bump: u8,
}
```

### 2. Update MakeOffer Context

Add Clock sysvar and expiration parameter:

```rust
#[derive(Accounts)]
#[instruction(id: u64, expires_in_seconds: u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
    // ... other accounts
}
```

### 3. Calculate Expiration

In `make_offer` function:

```rust
pub fn make_offer(
    context: Context<MakeOffer>,
    id: u64,
    token_a_offered_amount: u64,
    token_b_wanted_amount: u64,
    expires_in_seconds: u64,
) -> Result<()> {
    let clock = Clock::get()?;
    let expires_at = clock.unix_timestamp + expires_in_seconds as i64;
    
    context.accounts.offer.set_inner(Offer {
        // ... other fields
        expires_at,
        bump: context.bumps.offer,
    });
    Ok(())
}
```

### 4. Update TakeOffer Context

Add Clock sysvar:

```rust
#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
    // ... other accounts
}
```

### 5. Validate Expiration

In `take_offer` function:

```rust
pub fn take_offer(context: Context<TakeOffer>) -> Result<()> {
    let clock = Clock::get()?;
    require!(
        clock.unix_timestamp < context.accounts.offer.expires_at,
        SwapError::OfferExpired
    );
    // ... rest of the function
}
```

## Test Cases

| Test | Expected Result | Purpose |
|------|-----------------|---------|
| Expiration calculation | Correct timestamp | Validates time logic |
| Expired offer rejection | Error returned | Confirms expiration check |
| Valid offer acceptance | Success | Verifies normal operation |

## Notes

- Unix timestamps are in seconds since January 1, 1970
- Use `Clock::get()?` to access the current time
- Consider adding a minimum expiration time to prevent spam
- Expired offers can be cleaned up by a separate function

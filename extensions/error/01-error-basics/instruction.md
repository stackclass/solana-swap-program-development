# Error Basics

This stage introduces error handling in Anchor programs, covering the Result type and how errors propagate through the system.

## The Result Type

Solana program instructions return `Result<()>`:

```rust
pub fn make_offer(
    ctx: Context<MakeOffer>,
    id: u64,
    token_a_offered_amount: u64,
    token_b_wanted_amount: u64,
) -> Result<()>
```

The `Result` type is either `Ok(())` for success or `Err(error)` for failure. The `()` (unit type) indicates no return value on success.

## Using the Question Mark Operator

The `?` operator propagates errors:

```rust
pub fn make_offer(...) -> Result<()> {
    transfer_tokens(...)?;  // If this fails, return the error
    // If it succeeds, continue
    Ok(())
}
```

If `transfer_tokens` returns an error, the `?` operator immediately returns that error from the function. If it succeeds, execution continues.

## Require Macro

Anchor provides the `require!` macro for validation:

```rust
require!(condition, error_code);
```

If the condition is false, the specified error is returned. This is cleaner than manual if statements:

```rust
// Manual way
if amount == 0 {
    return Err(ErrorCode::InvalidAmount.into());
}

// Using require!
require!(amount > 0, ErrorCode::InvalidAmount);
```

## Require Keys

For comparing public keys:

```rust
require_keys!(
    ctx.accounts.taker.key(),
    ctx.accounts.maker.key(),
    ErrorCode::SelfTrade
);
```

This is equivalent to:

```rust
require!(
    ctx.accounts.taker.key() == ctx.accounts.maker.key(),
    ErrorCode::SelfTrade
);
```

## Error Propagation Through CPI

When a CPI fails, the error propagates automatically:

```rust
pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
    // If transfer fails, this returns the error
    transfer_tokens(...)?;  // Payment to maker
    
    // If first transfer succeeded, try second
    transfer_tokens(...)?;  // Vault to taker
    
    Ok(())
}
```

If the payment to maker fails, the vault transfer never happens. This atomicity is crucial for swaps.

## Practical Exercise

Review your program and identify all error conditions. Convert manual if statements to require! macros. Ensure all CPIs use ? for error propagation.

## Key Takeaways

Instructions return Result<()>. The ? operator propagates errors. require! provides clean validation. require_keys! compares public keys. CPI errors propagate automatically, maintaining atomicity.

# Error Practice

This stage provides hands-on exercises to implement comprehensive error handling in your swap program.

## Exercise 1: Define Complete Error Set

Define a comprehensive error enum for your swap program:

```rust
use anchor_lang::prelude::*;

#[error_code]
pub enum SwapError {
    // Validation errors
    #[msg("Token amount must be greater than zero")]
    InvalidAmount = 100,
    #[msg("Offer ID must be greater than zero")]
    InvalidOfferId,
    
    // Account errors
    #[msg("Offer not found")]
    OfferNotFound = 200,
    #[msg("Vault account mismatch")]
    VaultMismatch,
    #[msg("Token mint does not match offer")]
    MintMismatch,
    
    // State errors
    #[msg("Offer is not open for trading")]
    OfferNotOpen = 300,
    #[msg("Offer has already been taken")]
    OfferTaken,
    
    // Token errors
    #[msg("Insufficient token balance")]
    InsufficientBalance = 400,
    #[msg("Token transfer failed")]
    TransferFailed,
    
    // Security errors
    #[msg("Unauthorized: caller is not the offer maker")]
    NotMaker = 500,
    #[msg("Cannot trade with yourself")]
    SelfTrade,
}
```

## Exercise 2: Add Validation to Make Offer

Implement comprehensive validation in make_offer:

```rust
pub fn make_offer(
    ctx: Context<MakeOffer>,
    id: u64,
    token_a_offered_amount: u64,
    token_b_wanted_amount: u64,
) -> Result<()> {
    // Validate inputs
    require!(id > 0, SwapError::InvalidOfferId);
    require!(token_a_offered_amount > 0, SwapError::InvalidAmount);
    require!(token_b_wanted_amount > 0, SwapError::InvalidAmount);
    
    // Validate accounts
    require!(
        ctx.accounts.maker_token_account_a.amount >= token_a_offered_amount,
        SwapError::InsufficientBalance
    );
    
    // Proceed with offer creation
    Ok(())
}
```

## Exercise 3: Add Validation to Take Offer

Implement comprehensive validation in take_offer:

```rust
pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
    // Security validation
    require!(
        ctx.accounts.taker.key() != ctx.accounts.maker.key(),
        SwapError::SelfTrade
    );
    
    // State validation
    require!(
        ctx.accounts.offer.status == OfferStatus::Open,
        SwapError::OfferTaken
    );
    
    // Account validation
    require!(
        ctx.accounts.vault.mint == ctx.accounts.token_mint_a.key(),
        SwapError::VaultMismatch
    );
    
    // Balance validation
    require!(
        ctx.accounts.taker_token_account_b.amount >= ctx.accounts.offer.token_b_wanted_amount,
        SwapError::InsufficientBalance
    );
    
    require!(
        ctx.accounts.vault.amount >= ctx.accounts.offer.token_a_offered_amount,
        SwapError::InsufficientBalance
    );
    
    Ok(())
}
```

## Exercise 4: Handle CPI Errors

Add error handling for CPI operations:

```rust
pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
    // ... validation
    
    // Payment transfer
    let payment_result = transfer_payment_to_maker(ctx);
    if payment_result.is_err() {
        return Err(SwapError::TransferFailed.into());
    }
    
    // Vault transfer
    let vault_result = transfer_from_vault(ctx);
    if vault_result.is_err() {
        return Err(SwapError::TransferFailed.into());
    }
    
    Ok(())
}

// Or use map_err
transfer_payment_to_maker(ctx)
    .map_err(|_| SwapError::TransferFailed)?;
```

## Verification Criteria

Your implementation is complete when:

1. Error enum covers all failure cases
2. Validation in make_offer catches invalid inputs
3. Validation in take_offer catches invalid states
4. CPI errors are properly handled
5. Error messages are clear and actionable

## Common Mistakes to Avoid

Not covering all error cases. Think through every failure possibility.

Generic error messages. Be specific about what went wrong.

Silent failures. Let errors propagate with ?.

Forgetting to return errors. The ? operator or return is needed.

## Next Steps

With error handling complete, proceed to the Testing extension to learn about Rust-based testing.

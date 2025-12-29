# Security Practice

This stage provides hands-on exercises to implement comprehensive security measures in your swap program.

## Exercise 1: Add Status Field to Offer

Add a status field to prevent double-taking:

```rust
#[derive(Clone, Copy, PartialEq, Eq, AnchorSerialize, AnchorDeserialize, InitSpace)]
pub enum OfferStatus {
    Open,
    Taken,
}

#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_b_wanted_amount: u64,
    pub status: OfferStatus,
    pub bump: u8,
}
```

Initialize the status in make_offer:

```rust
offer.status = OfferStatus::Open;
```

## Exercise 2: Implement Status Validation in Take Offer

Add status checking to take_offer:

```rust
pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
    require!(
        ctx.accounts.offer.status == OfferStatus::Open,
        ErrorCode::OfferAlreadyTaken
    );
    
    // Set status before any CPI
    ctx.accounts.offer.status = OfferStatus::Taken;
    
    // ... rest of implementation
    
    Ok(())
}
```

## Exercise 3: Add Custom Validation

Add comprehensive custom validation to take_offer:

```rust
pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
    // Prevent self-trading
    require!(
        ctx.accounts.taker.key() != ctx.accounts.maker.key(),
        ErrorCode::CannotTakeOwnOffer
    );
    
    // Validate amounts
    require!(
        ctx.accounts.offer.token_b_wanted_amount > 0,
        ErrorCode::InvalidAmount
    );
    
    // Validate vault balance
    require!(
        ctx.accounts.vault.amount >= ctx.accounts.offer.token_a_offered_amount,
        ErrorCode::InsufficientVaultBalance
    );
    
    // Check status
    require!(
        ctx.accounts.offer.status == OfferStatus::Open,
        ErrorCode::OfferAlreadyTaken
    );
    
    // Set status before CPI
    ctx.accounts.offer.status = OfferStatus::Taken;
    
    Ok(())
}
```

## Exercise 4: Add Error Codes

Define appropriate error codes:

```rust
#[error_code]
pub enum SwapError {
    #[msg("Token amount must be greater than zero")]
    InvalidAmount,
    #[msg("Offer has already been taken")]
    OfferAlreadyTaken,
    #[msg("Cannot trade with yourself")]
    CannotTakeOwnOffer,
    #[msg("Vault has insufficient balance")]
    InsufficientVaultBalance,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Token mint mismatch")]
    TokenMintMismatch,
}
```

## Verification Criteria

Your implementation is complete when:

1. The Offer struct tracks status
2. Status is validated before any CPI
3. Status is set before external calls
4. Self-trading is prevented
5. Amounts are validated
6. Vault balance is checked
7. Appropriate error codes are defined

## Common Mistakes to Avoid

Setting status after CPI calls. This leaves a window for reentrancy.

Not initializing status in make_offer. The field will have garbage value.

Using generic error messages. Provide specific messages for debugging.

Skipping validation for "trusted" accounts. Always validate.

## Next Steps

With security measures in place, proceed to the CPI extension to deepen your understanding of cross-program invocation.

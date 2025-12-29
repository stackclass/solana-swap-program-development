# Account Validation

This stage teaches comprehensive account validation techniques to ensure your swap program only accepts legitimate accounts.

## Validation Layers

Effective validation happens at multiple levels:

1. **Constraint-level**: Using Anchor's built-in constraints
2. **Relationship-level**: Verifying accounts relate correctly
3. **Business-logic-level**: Enforcing program-specific rules

All three layers work together to create robust validation.

## Constraint-Level Validation

Anchor's constraints provide first-line defense:

```rust
#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    
    #[account(
        has_one = maker,
        has_one = token_mint_a,
        has_one = token_mint_b,
        seeds = [b"offer", maker.key().as_ref(), offer.id.to_le_bytes().as_ref()],
        bump = offer.bump
    )]
    pub offer: Account<'info, Offer>,
    
    pub token_mint_a: InterfaceAccount<'info, Mint>,
    pub token_mint_b: InterfaceAccount<'info, Mint>,
}
```

These constraints verify:
- The offer belongs to the specified maker
- The token mints match the offer
- The offer address is correctly derived

## Signer Verification

Always verify signers explicitly:

```rust
#[account(mut)]
pub taker: Signer<'info>,
```

The `Signer` type tells Anchor to verify the account signed the transaction. Without this, an attacker could provide accounts without the owner's signature.

For additional signer checks:

```rust
pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
    require!(ctx.accounts.taker.is_signer, ErrorCode::NotAuthorized);
    Ok(())
}
```

## Token Account Validation

Validate token accounts belong to the expected owner and mint:

```rust
#[account(
    mut,
    associated_token::mint = token_mint_a,
    associated_token::authority = taker
)]
pub taker_token_account_a: InterfaceAccount<'info, TokenAccount>,
```

The `associated_token` constraint ensures:
- The account is an ATA for the specified mint
- The account's owner is the specified authority

## Program Validation

Ensure accounts are owned by the correct programs:

```rust
// Token accounts must be owned by Token Program
pub token_account: InterfaceAccount<'info, TokenAccount>,

// System accounts must be owned by System Program  
pub system_account: Account<'info, SystemAccount>,
```

Anchor validates ownership automatically for known program types.

## Custom Validation

For complex validation beyond constraints:

```rust
pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
    // Validate taker is not the maker
    require!(
        ctx.accounts.taker.key() != ctx.accounts.maker.key(),
        ErrorCode::CannotTakeOwnOffer
    );
    
    // Validate amounts are reasonable
    require!(
        ctx.accounts.offer.token_b_wanted_amount > 0,
        ErrorCode::InvalidAmount
    );
    
    // Validate vault balance
    require!(
        ctx.accounts.vault.amount >= ctx.accounts.offer.token_a_offered_amount,
        ErrorCode::InsufficientVaultBalance
    );
    
    Ok(())
}
```

## Practical Exercise

Review your account structs for completeness. Add any missing constraints. Add custom validation for business rules. Test that invalid accounts are rejected with clear errors.

## Key Takeaways

Use Anchor constraints for automatic validation. Verify signers explicitly. Validate token accounts with associated_token constraints. Add custom validation for business logic. Test invalid inputs are rejected.

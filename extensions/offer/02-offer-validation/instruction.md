# Offer Validation

This stage explores comprehensive offer validation using Anchor's account constraints, ensuring only valid offers can be taken.

## Basic Validation Constraints

Anchor provides several constraints for offer validation:

```rust
#[account(
    mut,
    close = maker,
    has_one = maker,
    has_one = token_mint_a,
    has_one = token_mint_b,
    seeds = [b"offer", maker.key().as_ref(), offer.id.to_le_bytes().as_ref()],
    bump = offer.bump
)]
pub offer: Account<'info, Offer>,
```

The `has_one` constraints validate relationships between accounts. If the provided maker account does not match the offer's stored maker, validation fails. Same for token mints.

This prevents several attack vectors. An attacker cannot provide Bob's offer and claim it as Alice's. Cannot substitute different token mints than the offer specified. Cannot provide a tampered offer with modified terms.

## PDA Validation

The `seeds` and `bump` constraints validate that the offer account is the correct PDA:

```rust
seeds = [b"offer", maker.key().as_ref(), offer.id.to_le_bytes().as_ref()],
bump = offer.bump
```

This ensures the account address was derived using the expected seeds. Combined with `has_one = maker`, this tightly binds the offer to its creator and ID.

An attacker cannot create a fake offer account at a random address and present it as a valid offer. The PDA derivation validation catches this immediately.

## Close Constraint

The `close` constraint specifies where lamports go when the account is closed:

```rust
close = maker
```

This returns remaining rent to the maker after the offer is taken or canceled. The close destination should be the party that paid for account creation.

Never close to an unrelated party—this would enable theft of rent deposits.

## Mutability

The `mut` constraint marks the account as mutable:

```rust
mut
```

The offer is mutable because take_offer modifies it (to close it) and potentially to track state changes. Even accounts that are only closed should be marked mut because closing is a modification.

## Cross-Account Validation

Sometimes validation requires checking relationships between multiple accounts:

```rust
#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(
        has_one = token_mint_a,
        has_one = token_mint_b
    )]
    pub offer: Account<'info, Offer>,
    
    pub token_mint_a: InterfaceAccount<'info, Mint>,
    pub token_mint_b: InterfaceAccount<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = taker
    )]
    pub taker_token_account_a: InterfaceAccount<'info, TokenAccount>,
}
```

The validation ensures all three accounts agree on the token mints involved. Any mismatch causes validation failure.

## Custom Validation

For complex validation beyond constraints, add checks in the instruction handler:

```rust
pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
    let offer = &ctx.accounts.offer;
    
    require!(
        ctx.accounts.taker.key() != ctx.accounts.maker.key(),
        ErrorCode::CannotTakeOwnOffer
    );
    
    // Additional validation logic
    
    Ok(())
}
```

Custom validation handles business rules that cannot be expressed as simple constraints.

## Practical Exercise

Implement comprehensive validation for your take_offer instruction. Add has_one constraints for maker and both token mints. Add PDA validation with seeds and bump. Add custom validation for business rules like preventing self-trading.

Test validation by providing incorrect accounts and verifying rejection. Ensure the error messages clearly indicate what validation failed.

## Key Takeaways

has_one constraints validate account relationships. PDA validation with seeds and bump ensures address correctness. The close constraint specifies lamport destination. Custom validation handles complex business rules. All validation should fail fast with clear errors.

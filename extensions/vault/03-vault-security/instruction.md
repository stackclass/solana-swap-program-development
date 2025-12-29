# Vault Security

This stage explores critical security considerations for vault implementation. Secure vault design prevents unauthorized access and ensures funds are released only under correct conditions.

## Authority Validation

The vault's authority is the offer PDA, but validation must ensure the correct offer controls the vault:

```rust
#[account(
    mut,
    associated_token::mint = token_mint_a,
    associated_token::authority = offer  // Must match the offer PDA
)]
pub vault: InterfaceAccount<'info, TokenAccount>,
```

Anchor's `associated_token::authority = offer` constraint validates that the vault's authority matches the offer account. This prevents an attacker from providing a vault controlled by a different account.

## Mint Validation

The vault must hold only the token specified in the offer. Validate the mint matches expectations:

```rust
#[account(
    mut,
    associated_token::mint = token_mint_a,
    associated_token::authority = offer,
    constraint = vault.mint == token_mint_a.key()
)]
pub vault: InterfaceAccount<'info, TokenAccount>,
```

The mint is already enforced by the associated token constraint, but explicit validation can provide clearer error messages.

## Token Amount Validation

Validate the vault contains the expected amount before allowing any operation:

```rust
pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
    let vault_amount = ctx.accounts.vault.amount;
    require!(
        vault_amount >= ctx.accounts.offer.token_a_offered_amount,
        ErrorCode::InsufficientVaultBalance
    );
    
    // Proceed with swap
}
```

This prevents edge cases where the vault balance does not match offer expectations.

## Preventing Unauthorized Access

Only the take_offer instruction should be able to withdraw from the vault. The program design ensures this by:

1. Making the offer PDA the vault authority (no private key exists)
2. Requiring PDA signing for all vault withdrawals
3. Embedding vault access control in instruction handlers

External programs cannot withdraw from the vault because they lack the PDA signing capability. Even if they know the vault address, they cannot authorize transfers.

## Close Account Security

When closing the vault, specify a secure destination for the remaining lamports:

```rust
#[account(
    mut,
    close = taker  // Remaining lamports go to taker
)]
pub vault: InterfaceAccount<'info, TokenAccount>,
```

The close constraint ensures lamports return to an expected destination. The vault should close to either the taker (after successful swap) or the maker (if the offer is canceled).

Never close a vault to an arbitrary address—this would enable theft of any remaining lamports.

## Reentrancy Protection

Avoid calling external programs during vault operations that could lead to reentrancy attacks:

```rust
pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
    // Update all state FIRST
    // Then perform external calls LAST
    
    // BAD: Call external program before state updates
    external_call()?;
    state_update()?;
    
    // GOOD: Update state before external calls
    state_update()?;
    external_call()?;
    
    Ok(())
}
```

The pattern is to complete all state modifications before any CPI that could trigger callback behavior.

## Practical Exercise

Review your vault implementation for security vulnerabilities. Add explicit mint validation to the vault account. Implement balance checks before withdrawal. Ensure the close destination is secure.

Attempt to provide an incorrect vault account and verify validation fails. Attempt to close the vault to an unauthorized destination and verify rejection.

## Key Takeaways

Authority validation ensures the correct offer controls the vault. Mint validation prevents incorrect token types. Balance validation ensures expected funds are present. Close destination must be controlled and secure. Avoid reentrancy by updating state before external calls.

# PDA Bump Seeds

This stage explores bump seeds in depth, explaining why they exist, how the canonical bump is determined, and proper usage for secure PDA signing.

## Why Bump Seeds Exist

PDAs must not fall on the Ed25519 curve because addresses on the curve have corresponding private keys. When `find_program_address` attempts derivation, it checks if the result is on the curve. If it is, that address cannot be a PDA because someone could hold a private key for it.

The bump seed provides a way to find alternative addresses. Starting with bump 255, the derivation function tries each value descending until finding an off-curve result. The first off-curve result is the canonical bump.

```rust
// Internal logic of find_program_address
for bump in (0..=255).rev() {
    let address = derive_with_bump(bump);
    if !is_on_curve(address) {
        return (address, bump);  // Canonical bump found
    }
}
```

Most seed combinations produce an off-curve address quickly, but some require trying multiple bumps. The canonical bump is simply the first one that works.

## Understanding Canonical Bump

The canonical bump is the single official bump for a PDA. While you could technically derive the same PDA with different non-canonical bumps, only the canonical bump should be used for signing.

Using non-canonical bumps for signing would create ambiguity. If multiple bumps could sign for the same address, programs would need to track which bump was used. The canonical bump convention eliminates this complexity.

Anchor stores the canonical bump when creating PDA accounts and expects it during validation. This ensures consistency and prevents potential attacks where malicious actors try alternative bumps.

## Storing and Using the Bump

The bump must be stored in the account data when the account is created:

```rust
#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_b_wanted_amount: u64,
    pub bump: u8,
}
```

During initialization, Anchor automatically sets the bump field if it exists:

```rust
#[account(
    init,
    payer = maker,
    space = 8 + std::mem::size_of::<Offer>(),
    seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()],
    bump  // Anchor fills this
)]
pub offer: Account<'info, Offer>,
```

When the program needs to sign as the PDA (for vault transfers), it retrieves the stored bump:

```rust
let seeds = &[
    b"offer",
    ctx.accounts.maker.key().as_ref(),
    &ctx.accounts.offer.id.to_le_bytes(),
    &[ctx.accounts.offer.bump],  // Use stored canonical bump
];
let signer_seeds = [&seeds[..]];
```

## Bump in Validation vs Storage

Two different bumps serve distinct purposes. The validation bump exists only during account creation to derive the correct address. The storage bump persists in account data for later use.

Anchor conflates these in the `bump` constraint—the same bump value is both used for derivation and stored in the field. This is correct because both should use the canonical bump.

For accounts that are not created via `init` (like existing accounts), you must provide the bump explicitly:

```rust
#[account(
    mut,
    seeds = [b"offer", maker.key().as_ref(), offer.id.to_le_bytes().as_ref()],
    bump = offer.bump  // Reads from account data
)]
pub offer: Account<'info, Offer>,
```

The `bump = offer.bump` syntax means "read the bump from the offer account's bump field and use it for PDA derivation verification."

## Security Implications

Always use the canonical bump for signing. Using a non-canonical bump may work for derivation but creates inconsistencies:

1. The stored bump may not match your signing bump
2. Programs expecting canonical bump verification may reject your transaction
3. Future program versions may assume canonical bump behavior

Anchor's validation ensures the provided bump matches the account's stored bump, preventing mismatches. If you manually derive PDAs outside Anchor, ensure you always use and store the canonical bump.

## Practical Exercise

Experiment with `find_program_address` to observe bump behavior. Derive the same PDA with explicit bumps of 255, 254, etc. Note which values produce off-curve results.

Store the canonical bump in your offer account. Implement a function that retrieves the stored bump and uses it for CPI signing. Verify that without the correct bump, signing fails.

## Key Takeaways

Bump seeds ensure PDAs are off-curve with no corresponding private keys. The canonical bump is the first successful off-curve derivation. Store the canonical bump for later PDA signing operations. Always use canonical bump for security and consistency.

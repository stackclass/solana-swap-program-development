# PDA Derivation

This stage teaches how to implement PDA derivation in your swap program, choosing appropriate seeds and using derived addresses in your instruction handlers.

## Seed Selection Strategy

Seeds determine your PDA addresses and should capture the uniqueness of each account instance. For a swap offer, we need seeds that make each offer addressable and unique.

The maker's public key ensures offers are per-user—Alice's offer ID 1 differs from Bob's offer ID 1. The offer ID allows multiple offers per user. A literal string prefix distinguishes offers from other PDA types the program might manage.

```rust
seeds = [b"offer", maker_pubkey, id_bytes]
```

This seed combination produces unique, deterministic addresses. Alice offering 1000 USDC for 1 SOL gets one address. Alice offering 2000 USDC for 1 SOL gets a different address (different ID). Bob's offer with the same parameters as Alice's also gets a different address.

## Deriving in Account Validation

Anchor's `#[account]` attribute can derive PDAs automatically during validation:

```rust
#[account(
    init,
    payer = maker,
    space = 8 + std::mem::size_of::<Offer>(),
    seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()],
    bump
)]
pub offer: Account<'info, Offer>,
```

The `bump` constraint tells Anchor to derive the PDA and store the canonical bump in the account validation struct. Anchor also verifies the PDA derived from accounts matches the address provided in the transaction.

The `init` constraint combined with PDA seeds creates the account at the derived address. Anchor handles the SystemProgram call with the correct address, so you do not need to manually create the account.

## Storing the Bump

The canonical bump must be stored in the account data so it can be used later for signing:

```rust
#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_b_wanted_amount: u64,
    pub bump: u8,  // Store for later PDA signing
}
```

Anchor's `init` constraint automatically captures the bump from derivation and can initialize the field if named `bump`. Alternatively, you can manually set it after account creation.

The bump is essential for take_offer, where the program must sign as the PDA to transfer tokens from the vault. Without the stored bump, the program cannot reconstruct the signing authority.

## Derivation in Instruction Handlers

Sometimes you need to derive PDAs within instruction logic rather than validation:

```rust
pub fn close_offer(ctx: Context<CloseOffer>) -> Result<()> {
    let seeds = &[
        b"offer",
        ctx.accounts.maker.key.as_ref(),
        &ctx.accounts.offer.id.to_le_bytes(),
        &[ctx.accounts.offer.bump],
    ];
    
    // Use seeds for CPI signing
    let signer_seeds = [&seeds[..]];
    
    // ...
}
```

Derivation in the handler provides flexibility for operations where the PDA might already exist or where seed composition is dynamic.

## Validating PDA Relationships

The `has_one` constraint validates relationships between accounts:

```rust
#[account(
    mut,
    close = maker,
    has_one = maker,
    seeds = [b"offer", maker.key().as_ref(), offer.id.to_le_bytes().as_ref()],
    bump = offer.bump
)]
pub offer: Account<'info, Offer>,
```

The `has_one = maker` constraint verifies the account's maker field matches the maker account provided. This prevents attacks where someone provides an offer account belonging to a different maker.

Combined with PDA validation, this ensures the offer account is exactly what the transaction claims: an offer created by this maker with this ID.

## Practical Exercise

Implement PDA derivation for the offer account in your swap program. Define appropriate seeds that capture the offer's identity. Store the bump for later use in take_offer.

Add validation constraints to ensure the offer matches the expected maker and ID. Test that incorrect accounts are rejected during validation.

## Key Takeaways

Seed selection should capture account uniqueness. Anchor's `bump` constraint handles derivation automatically. Store the bump for later PDA signing operations. The `has_one` constraint validates account relationships. Handler-level derivation provides flexibility for dynamic cases.

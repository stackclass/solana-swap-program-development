# Offer Data Structure

This stage teaches how to design and implement the Offer account structure that stores swap proposal details on-chain.

## What Data Does an Offer Store

An offer represents a user's intent to swap tokens under specific terms. The offer account must store all information needed to execute the swap and validate it during take_offer.

The offer should store:

**Identity Information**: A unique identifier (ID) distinguishes this offer from others the maker might create. Combined with the maker's address, the ID creates a unique offer key.

**Token Information**: The addresses of both token mints involved in the swap. The offered mint (token A) and the wanted mint (token B) define what tokens are being exchanged.

**Amount Information**: The amounts specify the exchange rate. The offered amount indicates how much token A the maker will provide. The wanted amount indicates how much token B the maker expects to receive.

**Security Information**: The bump seed stores the canonical PDA bump for use during take_offer when signing as the offer PDA.

## Designing the Offer Struct

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

Each field serves a specific purpose. The `id` allows multiple offers per maker. The `maker` identifies who created the offer and who receives payment. The `token_mint_a` and `token_mint_b` specify the tokens involved. The `token_b_wanted_amount` defines the exchange rate. The `bump` enables PDA signing.

## Space Calculation

Anchor's `#[derive(InitSpace)]` automatically calculates the required space:

```rust
#[derive(InitSpace)]
pub struct Offer {
    pub id: u64,                // 8 bytes
    pub maker: Pubkey,          // 32 bytes
    pub token_mint_a: Pubkey,   // 32 bytes
    pub token_mint_b: Pubkey,   // 32 bytes
    pub token_b_wanted_amount: u64, // 8 bytes
    pub bump: u8,               // 1 byte (plus padding)
}
```

The total is approximately 113 bytes plus the 8-byte discriminator. Anchor automatically includes the discriminator in space calculations when you use the `space` attribute with `std::mem::size_of::<Offer>()`.

## Alternative: Explicit Space

For more control, you can calculate space manually:

```rust
#[account]
pub struct Offer {
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_b_wanted_amount: u64,
    pub bump: u8,
}

impl Offer {
    pub const SPACE: usize = 8 + 32 + 32 + 32 + 8 + 1;
}
```

Manual calculation ensures you account for all fields and can adjust for future additions.

## Pubkey vs InterfaceAccount

The offer stores token mints as `Pubkey` rather than `InterfaceAccount<Mint>`. This saves space and CPI overhead during validation.

```rust
pub token_mint_a: Pubkey,
pub token_mint_b: Pubkey,
```

During instruction validation, the `MakeOffer` and `TakeOffer` structs provide the actual `InterfaceAccount<Mint>` types. The offer only needs to store the addresses for comparison.

## Validation with Stored Data

The take_offer instruction validates the offer using its stored data:

```rust
#[account(
    mut,
    has_one = maker,
    has_one = token_mint_a,
    has_one = token_mint_b,
    seeds = [b"offer", maker.key().as_ref(), offer.id.to_le_bytes().as_ref()],
    bump = offer.bump
)]
pub offer: Account<'info, Offer>,
```

The `has_one` constraints verify the accounts match the offer's stored values. This prevents attacks where someone provides an offer with mismatched token mints.

## Practical Exercise

Design your Offer struct with appropriate fields. Use `#[derive(InitSpace)]` for automatic space calculation. Implement validation using `has_one` constraints. Store the bump for later use.

Consider what additional fields might be useful: creation timestamp, expiration time, minimum taker amount. Evaluate whether these belong in the base offer or as extensions.

## Key Takeaways

The Offer struct stores all information needed to execute a swap. Space calculation can be automatic via InitSpace or manual. Store Pubkeys for token mints rather than full account types. Use has_one constraints to validate accounts against stored data. The bump field enables PDA signing during take_offer.

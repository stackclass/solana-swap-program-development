# Save Offer

This stage teaches how to initialize and populate the Offer account with the swap terms during make_offer.

## Account Initialization

The Offer account is initialized by Anchor's `init` constraint in the account struct:

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

The `init` constraint tells Anchor to create the account via SystemProgram if it does not exist. The `payer = maker` means the maker pays for the account creation cost. The `seeds` and `bump` derive the PDA address.

The account data is zeroed on creation. We must then populate it with the offer terms.

## Setting Account Data

After account creation, populate the fields:

```rust
pub fn make_offer(
    ctx: Context<MakeOffer>,
    id: u64,
    token_a_offered_amount: u64,
    token_b_wanted_amount: u64,
) -> Result<()> {
    let offer = &mut ctx.accounts.offer;
    
    offer.id = id;
    offer.maker = ctx.accounts.maker.key();
    offer.token_mint_a = ctx.accounts.token_mint_a.key();
    offer.token_mint_b = ctx.accounts.token_mint_b.key();
    offer.token_b_wanted_amount = token_b_wanted_amount;
    offer.bump = ctx.bumps.offer;
    
    // ... then deposit tokens
    
    Ok(())
}
```

The `ctx.bumps.offer` field contains the canonical bump derived during account validation. This is essential for later PDA signing.

## Field Population Order

The order of operations matters for error handling:

1. Initialize Offer fields first
2. Then perform token transfer

If the token transfer fails, the Offer account is still created but will have invalid state. This is acceptable—the Offer can be closed later.

Alternatively, you can transfer tokens first, but this may leave orphaned accounts if transfer fails.

## Using References

The code uses `&mut ctx.accounts.offer` to get a mutable reference. This allows modifying the account data while keeping the context borrow.

```rust
let offer = &mut ctx.accounts.offer;
offer.id = id;
// ...
```

This pattern avoids consuming the context while allowing field updates.

## Why Store Pubkeys Not Accounts

The Offer struct stores `Pubkey` values rather than `InterfaceAccount<Mint>`:

```rust
pub token_mint_a: Pubkey,
pub token_mint_b: Pubkey,
```

This saves space (32 bytes vs full account) and simplifies storage. During validation, the TakeOffer struct provides full account types that are compared against the stored pubkeys via `has_one` constraints.

## Validation in Take Offer

The stored pubkeys are validated during take_offer:

```rust
#[account(
    has_one = token_mint_a,
    has_one = token_mint_b
)]
pub offer: Account<'info, Offer>,

pub token_mint_a: InterfaceAccount<'info, Mint>,
pub token_mint_b: InterfaceAccount<'info, Mint>,
```

The `has_one` constraints compare the stored pubkeys against the provided account addresses. Any mismatch causes validation failure.

## Practical Exercise

Implement the Offer field population in your make_offer instruction. Ensure all fields are set correctly. Verify the bump is stored. Test that take_offer validation succeeds with correct accounts.

Add a test that provides mismatched accounts and verify take_offer correctly rejects them.

## Key Takeaways

Offer account is initialized by Anchor's init constraint. Field population happens after account creation. Store Pubkeys for token mints, not full account types. The bump from ctx.bumps is essential for later signing. Validation in take_offer uses has_one constraints against stored pubkeys.

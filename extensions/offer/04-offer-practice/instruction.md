# Offer Practice

This stage provides hands-on exercises to implement the complete Offer data structure and validation in your swap program.

## Exercise 1: Define the Offer Struct

Define the Offer account struct with all necessary fields:

```rust
#[account]
#[derive(InitSpace)]
pub struct Offer {
    // Unique identifier for the offer
    pub id: u64,
    
    // Address of the user who created the offer
    pub maker: Pubkey,
    
    // Token being offered (maker will give this)
    pub token_mint_a: Pubkey,
    
    // Token wanted by maker (taker must provide this)
    pub token_mint_b: Pubkey,
    
    // Amount of token B that maker wants
    pub token_b_wanted_amount: u64,
    
    // Canonical bump for PDA signing
    pub bump: u8,
}
```

Consider what fields are essential vs optional. The minimum viable offer needs all these fields.

## Exercise 2: Initialize Offer in Make Offer

Implement the offer initialization in your MakeOffer struct:

```rust
#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    
    pub token_mint_a: InterfaceAccount<'info, Mint>,
    pub token_mint_b: InterfaceAccount<'info, Mint>,
    
    #[account(
        init,
        payer = maker,
        space = 8 + std::mem::size_of::<Offer>(),
        seeds = [/* YOUR SEEDS */],
        bump
    )]
    pub offer: Account<'info, Offer>,
    
    // ... vault account
}
```

The `bump` constraint tells Anchor to derive the PDA and store the canonical bump in the offer.bump field.

## Exercise 3: Initialize Offer Data

In your make_offer instruction, initialize the offer fields:

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
    
    // Then perform token transfer to vault
    
    Ok(())
}
```

## Exercise 4: Validate Offer in Take Offer

Implement comprehensive validation in your TakeOffer struct:

```rust
#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    
    pub maker: SystemAccount<'info>,
    
    pub token_mint_a: InterfaceAccount<'info, Mint>,
    pub token_mint_b: InterfaceAccount<'info, Mint>,
    
    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = token_mint_a,
        has_one = token_mint_b,
        seeds = [/* SAME SEEDS AS MAKE_OFFER */],
        bump = offer.bump
    )]
    pub offer: Account<'info, Offer>,
    
    // ... vault and other accounts
}
```

Add custom validation in the handler to prevent taker == maker.

## Verification Criteria

Your implementation is complete when:

1. The Offer struct stores all necessary information
2. The offer is created at the correct PDA address
3. The canonical bump is stored and accessible
4. Validation in take_offer rejects invalid offers
5. has_one constraints correctly validate account relationships

## Common Mistakes to Avoid

Using different seeds in MakeOffer vs TakeOffer. Both must use identical seed composition.

Not storing the bump field. The offer must store the bump for later use.

Forgetting has_one constraints. These provide essential validation against account tampering.

Using the wrong field names in has_one. The constraint name must match the field name in the struct.

## Next Steps

With the Offer structure complete, proceed to the Make Offer extension to implement the full make_offer instruction flow.

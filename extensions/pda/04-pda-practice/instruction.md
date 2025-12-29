# PDA Practice

This stage provides hands-on exercises to reinforce PDA concepts through implementation in your swap program.

## Exercise 1: Define Offer PDA Seeds

Your task is to define appropriate seeds for the swap offer PDA. Consider what information uniquely identifies an offer.

The offer should be:
- Findable by maker (so they can list their offers)
- Unique per offer ID (so a maker can have multiple offers)
- Distinguishable from other PDA types in the program

Implement the seeds in your `MakeOffer` account validation:

```rust
#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    
    #[account(
        init,
        payer = maker,
        space = 8 + std::mem::size_of::<Offer>(),
        seeds = [/* YOUR SEEDS HERE */],
        bump
    )]
    pub offer: Account<'info, Offer>,
    
    // ... other accounts
}
```

Consider using:
- A string literal for type distinction
- The maker's public key for per-user scoping
- The offer ID for per-offer uniqueness

## Exercise 2: Store the Canonical Bump

Ensure the Offer struct can store the canonical bump for later use in take_offer:

```rust
#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_b_wanted_amount: u64,
    // Add bump field here
}
```

The `#[derive(InitSpace)]` macro automatically calculates required space including the bump field. Verify the space allocation matches your actual data.

## Exercise 3: Validate Offer PDA in Take Offer

Implement validation for the offer account in the take_offer instruction. The validation should:
- Verify the offer matches the expected PDA derivation
- Ensure the offer belongs to the specified maker
- Use the stored bump for validation

```rust
#[derive(Accounts)]
pub struct TakeOffer<'info> {
    // ... other accounts
    
    #[account(
        mut,
        close = maker,
        has_one = maker,
        seeds = [/* SAME SEEDS AS MAKE_OFFER */],
        bump = offer.bump  // Use stored bump
    )]
    pub offer: Account<'info, Offer>,
    
    // ... vault account
}
```

The `has_one = maker` constraint provides additional security by verifying the account's stored maker matches the provided maker account.

## Exercise 4: Implement PDA Signing for Vault Transfer

In the take_offer instruction, implement PDA signing to authorize token transfer from the vault. The program must sign as the offer PDA because the vault is controlled by the offer.

```rust
pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
    let seeds = &[
        b"offer",
        ctx.accounts.maker.key().as_ref(),
        &ctx.accounts.offer.id.to_le_bytes(),
        &[ctx.accounts.offer.bump],  // Use stored bump
    ];
    let signer_seeds = [&seeds[..]];
    
    // Use signer_seeds in CPI context for vault transfer
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
        signer_seeds,
    );
    
    // Perform transfer
    transfer_checked(cpi_ctx, /* amount */, /* decimals */)?;
    
    Ok(())
}
```

Verify that the transfer succeeds only when using the correct seeds and bump.

## Verification Criteria

Your implementation is complete when:

1. The offer PDA can be derived from the maker address and offer ID
2. The canonical bump is stored in the offer account
3. The take_offer validation accepts only valid offer accounts
4. The vault transfer succeeds with PDA signing
5. Attempting to use an invalid offer account fails validation

## Common Mistakes to Avoid

Forgetting to include the bump in seeds during CPI signing. The seeds array must end with `&[bump]` where bump is a single-element slice.

Using different seeds in make_offer vs take_offer. Both must use identical seed composition for PDA validation to succeed.

Not storing the bump field in the Offer struct. Without storage, you cannot retrieve the bump for signing in take_offer.

Using the wrong account's bump. Each PDA has its own bump—the offer's bump differs from any other PDA's bump.

## Next Steps

With PDA implementation complete, proceed to the Vault extension to learn how PDAs control token vaults for secure fund custody.

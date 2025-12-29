# Make Offer Practice

This stage provides hands-on exercises to implement the complete make_offer instruction.

## Exercise 1: Define MakeOffer Account Struct

Define the complete MakeOffer account validation struct:

```rust
#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOffer<'info> {
    // The user creating the offer - must sign the transaction
    #[account(mut)]
    pub maker: Signer<'info>,
    
    // The token being offered (will go to vault)
    pub token_mint_a: InterfaceAccount<'info, Mint>,
    
    // The token wanted in exchange
    pub token_mint_b: InterfaceAccount<'info, Mint>,
    
    // Maker's token account for the offered token
    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker
    )]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,
    
    // The offer account - will store swap terms
    #[account(
        init,
        payer = maker,
        space = 8 + std::mem::size_of::<Offer>(),
        seeds = [/* YOUR SEEDS HERE */],
        bump
    )]
    pub offer: Account<'info, Offer>,
    
    // The vault - will hold deposited tokens
    #[account(
        init,
        payer = maker,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    
    // Required programs
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
```

## Exercise 2: Implement Token Transfer

Implement the token deposit logic using transfer_checked:

```rust
use anchor_spl::token_interface::{
    transfer_checked, TransferChecked, Mint, TokenAccount, TokenInterface
};

pub fn make_offer(
    ctx: Context<MakeOffer>,
    id: u64,
    token_a_offered_amount: u64,
    token_b_wanted_amount: u64,
) -> Result<()> {
    // Build TransferChecked CPI accounts
    let cpi_accounts = TransferChecked {
        from: /* maker's token account */,
        mint: /* token mint */,
        to: /* vault account */,
        authority: /* maker (signer) */,
    };
    
    // Create CPI context
    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );
    
    // Perform the transfer with decimal precision
    transfer_checked(
        cpi_context,
        token_a_offered_amount,
        ctx.accounts.token_mint_a.decimals,
    )?;
    
    Ok(())
}
```

## Exercise 3: Initialize Offer Fields

Complete the instruction by initializing the offer fields:

```rust
pub fn make_offer(
    ctx: Context<MakeOffer>,
    id: u64,
    token_a_offered_amount: u64,
    token_b_wanted_amount: u64,
) -> Result<()> {
    // First, transfer tokens to vault
    // ... (from Exercise 2)
    
    // Then, initialize offer fields
    let offer = &mut ctx.accounts.offer;
    
    offer.id = id;
    offer.maker = /* maker's public key */;
    offer.token_mint_a = /* token mint A public key */;
    offer.token_mint_b = /* token mint B public key */;
    offer.token_b_wanted_amount = token_b_wanted_amount;
    offer.bump = /* canonical bump from ctx.bumps */;
    
    Ok(())
}
```

## Exercise 4: Add Input Validation

Add validation to reject invalid inputs:

```rust
pub fn make_offer(
    ctx: Context<MakeOffer>,
    id: u64,
    token_a_offered_amount: u64,
    token_b_wanted_amount: u64,
) -> Result<()> {
    // Validate inputs
    require!(token_a_offered_amount > 0, ErrorCode::InvalidAmount);
    require!(token_b_wanted_amount > 0, ErrorCode::InvalidAmount);
    
    // ... rest of implementation
    
    Ok(())
}
```

Define appropriate error codes in your error module.

## Verification Criteria

Your implementation is complete when:

1. The MakeOffer struct validates all required accounts
2. The offer account is created at the correct PDA
3. The vault account is created with offer as authority
4. Tokens transfer from maker to vault
5. All offer fields are correctly initialized
6. Invalid inputs are rejected with appropriate errors

## Common Mistakes to Avoid

Forgetting to include the associated_token_program. This is required for creating associated token accounts.

Using the wrong mint for the vault. The vault should use token_mint_a (the offered token).

Not marking mutable accounts as mut. Both maker_token_account_a and offer need to be modified.

Using the wrong bump source. Use ctx.bumps.offer, not ctx.bumps.vault.

## Next Steps

With make_offer complete, proceed to the Take Offer extension to implement the complementary take_offer instruction.

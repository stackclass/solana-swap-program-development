# Take Offer Practice

This stage provides hands-on exercises to implement the complete take_offer instruction.

## Exercise 1: Define TakeOffer Account Struct

Define the complete TakeOffer account validation struct:

```rust
#[derive(Accounts)]
pub struct TakeOffer<'info> {
    // The user accepting the offer - must sign
    #[account(mut)]
    pub taker: Signer<'info>,
    
    // The offer creator - receives payment
    pub maker: SystemAccount<'info>,
    
    // Token mints from the offer
    pub token_mint_a: InterfaceAccount<'info, Mint>,
    pub token_mint_b: InterfaceAccount<'info, Mint>,
    
    // Taker's account for offered tokens (will receive)
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_a,
        associated_token::authority = taker
    )]
    pub taker_token_account_a: InterfaceAccount<'info, TokenAccount>,
    
    // Taker's account for wanted tokens (paying from)
    #[account(
        mut,
        associated_token::mint = token_mint_b,
        associated_token::authority = taker
    )]
    pub taker_token_account_b: InterfaceAccount<'info, TokenAccount>,
    
    // Maker's account for wanted tokens (receiving payment)
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_b,
        associated_token::authority = maker
    )]
    pub maker_token_account_b: InterfaceAccount<'info, TokenAccount>,
    
    // The offer - validates terms and authorizes vault
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
    
    // The vault - holds offered tokens
    #[account(
        mut,
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

## Exercise 2: Implement Payment Transfer

Implement the payment transfer from taker to maker:

```rust
pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
    // Transfer payment from taker to maker
    let payment_accounts = TransferChecked {
        from: ctx.accounts.taker_token_account_b.to_account_info(),
        mint: ctx.accounts.token_mint_b.to_account_info(),
        to: ctx.accounts.maker_token_account_b.to_account_info(),
        authority: ctx.accounts.taker.to_account_info(),
    };
    
    let payment_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        payment_accounts,
    );
    
    transfer_checked(
        payment_ctx,
        ctx.accounts.offer.token_b_wanted_amount,
        ctx.accounts.token_mint_b.decimals,
    )?;
    
    Ok(())
}
```

## Exercise 3: Implement Vault Withdrawal

Implement the vault withdrawal with PDA signing:

```rust
pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
    // Payment transfer (from Exercise 2)
    
    // Prepare PDA signing seeds
    let seeds = &[
        b"offer",
        ctx.accounts.maker.key().as_ref(),
        &ctx.accounts.offer.id.to_le_bytes(),
        &[ctx.accounts.offer.bump],
    ];
    let signer_seeds = [&seeds[..]];
    
    // Transfer from vault to taker
    let vault_accounts = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        mint: ctx.accounts.token_mint_a.to_account_info(),
        to: ctx.accounts.taker_token_account_a.to_account_info(),
        authority: ctx.accounts.offer.to_account_info(),
    };
    
    let vault_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        vault_accounts,
        signer_seeds,
    );
    
    transfer_checked(
        vault_ctx,
        ctx.accounts.vault.amount,
        ctx.accounts.token_mint_a.decimals,
    )?;
    
    Ok(())
}
```

## Exercise 4: Close Accounts

Add account closing to complete the instruction:

```rust
// Close the offer account - remaining lamports go to maker
// (Handled by close = maker in account struct)

// Close the vault account
let close_accounts = CloseAccount {
    account: ctx.accounts.vault.to_account_info(),
    destination: ctx.accounts.taker.to_account_info(),
    authority: ctx.accounts.offer.to_account_info(),
};

let close_ctx = CpiContext::new_with_signer(
    ctx.accounts.token_program.to_account_info(),
    close_accounts,
    signer_seeds,
);

close_account(close_ctx)?;
```

## Verification Criteria

Your implementation is complete when:

1. The TakeOffer struct validates all accounts and relationships
2. Payment transfers from taker to maker
3. Offered tokens transfer from vault to taker using PDA signing
4. The offer account closes to maker
5. The vault account closes to taker
6. All state changes are atomic

## Common Mistakes to Avoid

Using wrong seeds for PDA signing. Must match exactly what was used in make_offer.

Forgetting CpiContext::new_with_signer. Regular CpiContext will fail because vault authority is a PDA.

Not using init_if_needed for ATAs. Users may not have accounts for the token mints.

Not marking mutable accounts as mut. Both taker accounts and offer need mut.

## Next Steps

With take_offer complete, proceed to the Security extension to learn about common vulnerabilities and how to prevent them.

# CPI Practice

This stage provides hands-on exercises to implement CPI calls in your swap program.

## Exercise 1: Create Transfer Helper

Create a reusable transfer_tokens helper function:

```rust
use anchor_spl::token_interface::{
    transfer_checked, TransferChecked, Mint, TokenAccount, TokenInterface
};

pub fn transfer_tokens<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: u64,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &Signer<'info>,
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    let cpi_accounts = TransferChecked {
        from: from.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };
    
    let cpi_context = CpiContext::new(
        token_program.to_account_info(),
        cpi_accounts,
    );
    
    transfer_checked(cpi_context, amount, mint.decimals)
}
```

## Exercise 2: Use Helper in Make Offer

Use the helper in make_offer for token deposit:

```rust
pub fn make_offer(
    ctx: Context<MakeOffer>,
    id: u64,
    token_a_offered_amount: u64,
    token_b_wanted_amount: u64,
) -> Result<()> {
    transfer_tokens(
        &ctx.accounts.maker_token_account_a,
        &ctx.accounts.vault,
        token_a_offered_amount,
        &ctx.accounts.token_mint_a,
        &ctx.accounts.maker,
        &ctx.accounts.token_program,
    )?;
    
    Ok(())
}
```

## Exercise 3: Implement PDA-Signed Transfer

Implement vault withdrawal with PDA signing:

```rust
pub fn withdraw_from_vault(
    ctx: &Context<TakeOffer>,
    amount: u64,
) -> Result<()> {
    let seeds = &[
        b"offer",
        ctx.accounts.maker.key().as_ref(),
        &ctx.accounts.offer.id.to_le_bytes(),
        &[ctx.accounts.offer.bump],
    ];
    let signer_seeds = [&seeds[..]];
    
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        mint: ctx.accounts.token_mint_a.to_account_info(),
        to: ctx.accounts.taker_token_account_a.to_account_info(),
        authority: ctx.accounts.offer.to_account_info(),
    };
    
    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );
    
    transfer_checked(cpi_context, amount, ctx.accounts.token_mint_a.decimals)
}
```

## Exercise 4: Close Account with CPI

Implement vault closing with PDA signing:

```rust
use anchor_spl::token_interface::{close_account, CloseAccount};

pub fn close_vault(ctx: &Context<TakeOffer>) -> Result<()> {
    let seeds = &[
        b"offer",
        ctx.accounts.maker.key().as_ref(),
        &ctx.accounts.offer.id.to_le_bytes(),
        &[ctx.accounts.offer.bump],
    ];
    let signer_seeds = [&seeds[..]];
    
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
    
    close_account(close_ctx)
}
```

## Verification Criteria

Your implementation is complete when:

1. The transfer_tokens helper works for regular transfers
2. Make offer uses the helper for deposits
3. Vault withdrawal uses PDA signing
4. Vault closing uses PDA signing
5. All CPI calls succeed in testing

## Common Mistakes to Avoid

Forgetting to convert accounts to account_info. Using `to_account_info()` is required.

Using wrong CpiContext constructor. Use `new_with_signer` for PDA signing.

Not using correct seed format. Seeds must be byte slices.

## Next Steps

With CPI understanding complete, proceed to the Error extension to learn about proper error handling.

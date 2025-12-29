# Receive Tokens

This stage teaches how to implement the payment transfer from taker to maker as part of the take_offer instruction.

## Payment Flow

The taker must pay the maker the wanted amount in exchange for the offered tokens. This is a standard token transfer using the Token Program.

Unlike the vault transfer, this transfer uses the taker's direct signature. The taker authorizes moving tokens from their account to the maker's account.

## Implementing the Payment Transfer

```rust
use anchor_spl::token_interface::{
    transfer_checked, TransferChecked, Mint, TokenAccount, TokenInterface
};

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

## Amount Source

The payment amount comes from the offer's stored terms:

```rust
ctx.accounts.offer.token_b_wanted_amount
```

This ensures the taker pays exactly what the maker requested. The amount is validated during account validation via `has_one` constraints, so the taker cannot specify a different amount.

## Token Mint Validation

The payment uses token mint B, which is validated by the account struct:

```rust
#[account(
    has_one = token_mint_b
)]
pub offer: Account<'info, Offer>,

pub token_mint_b: InterfaceAccount<'info, Mint>,
```

If the provided token_mint_b does not match the offer's stored mint, validation fails.

## Associated Token Accounts

The maker and taker may not have token accounts for mint B yet. Use `init_if_needed` to create them if missing:

```rust
#[account(
    init_if_needed,
    payer = taker,
    associated_token::mint = token_mint_b,
    associated_token::authority = maker
)]
pub maker_token_account_b: InterfaceAccount<'info, TokenAccount>,
```

This allows the swap to proceed even if the maker has not previously interacted with mint B.

## Error Cases

The payment transfer can fail for:

**Insufficient balance**: The taker does not have enough tokens. Error: `TokenError::InsufficientFunds`.

**Missing ATA**: The maker has no token account for mint B. The `init_if_needed` should handle this, but may fail if rent is insufficient.

**Mint mismatch**: The accounts do not match. Should be caught by validation.

## Practical Exercise

Implement the payment transfer in your take_offer instruction. Use transfer_checked with the wanted amount from the offer. Test with sufficient balance and verify the maker receives tokens. Test with insufficient balance and verify proper error.

## Key Takeaways

Payment transfer moves wanted tokens from taker to maker. The amount comes from the offer's stored terms. init_if_needed handles missing associated token accounts. Errors propagate correctly through Anchor.

# Transfer Checked

This stage teaches the transfer_checked instruction, the preferred method for token transfers that handles decimal precision correctly.

## Why transfer_checked

The Token Program provides two transfer instructions: `transfer` and `transfer_checked`. Use `transfer_checked` because it validates decimal precision.

When you specify an amount, the Token Program must know the token's decimal count to interpret the amount correctly. For USDC with 6 decimals, an amount of "1000000" represents 1 USDC. Without decimal information, the program cannot distinguish between 1 USDC and 1000000 USDC.

## Implementing transfer_checked

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

## The Decimal Parameter

The third parameter `mint.decimals` is critical. This comes from the Mint account and specifies how many decimal places the token uses:

```rust
transfer_checked(cpi_context, amount, mint.decimals)
```

Common decimal values:
- SOL: 9 decimals
- USDC, USDT: 6 decimals
- BTC: 8 decimals
- Many tokens: varies

## How Amounts Work

Token amounts are always stored as integers. When users think of "1.5 USDC", the program works with "1500000" (1.5 × 10^6).

The client typically converts human-readable amounts to raw amounts before sending to the program. Your program works with raw amounts directly.

## Transfer Amount Limits

The Token Program has minimum and maximum transfer amounts. For most tokens, the maximum is u64::MAX. The minimum is 1 (you cannot transfer 0 tokens unless specifically allowed).

Always validate amounts are non-zero:

```rust
require!(amount > 0, ErrorCode::InvalidAmount);
```

## Practical Exercise

Implement a transfer_tokens helper function using transfer_checked. Test with different tokens having different decimal precisions. Verify the amounts are correctly interpreted.

## Key Takeaways

transfer_checked validates decimal precision. The mint.decimals parameter specifies token precision. Amounts are always raw integers, not human-readable. Always validate non-zero amounts.

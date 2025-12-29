# Deposit Tokens

This stage teaches how to implement the token deposit from maker to vault using Cross-Program Invocation to the Token Program.

## Understanding the Deposit

The deposit moves tokens from the maker's token account into the vault. This is a standard token transfer using the Token Program's transfer_checked instruction.

The maker signs as authority because they are transferring from their own account. The vault receives the tokens because it is the destination account. The program orchestrates this but does not control the tokens directly.

## Creating the Transfer Context

The transfer requires building a proper CPI context:

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
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.maker_token_account_a.to_account_info(),
        mint: ctx.accounts.token_mint_a.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.maker.to_account_info(),
    };
    
    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );
    
    transfer_checked(
        cpi_context,
        token_a_offered_amount,
        ctx.accounts.token_mint_a.decimals,
    )?;
    
    Ok(())
}
```

## Transfer Checked vs Transfer

The `transfer_checked` instruction is preferred over basic `transfer` because it validates the decimal precision of the transfer amount.

```rust
transfer_checked(cpi_context, amount, mint.decimals)
```

The `mint.decimals` field comes from the Mint account and specifies how many decimal places the token uses. For USDC (6 decimals), an amount of "1000000" represents 1 USDC.

This prevents common mistakes where developers confuse raw token amounts with human-readable amounts.

## The CPI Context

The `CpiContext::new` function creates a context for the CPI call:

```rust
let cpi_context = CpiContext::new(
    token_program_account_info,  // Which program to call
    cpi_accounts,                 // Accounts for the instruction
);
```

The token program ID is inferred from the first account in `cpi_accounts`. This must be a Token Program or Token Extensions Program account.

## Handling Decimal Precision

Different tokens have different decimal precision. Common values:

- SOL: 9 decimals
- USDC: 6 decimals
- USDT: 6 decimals
- Many meme tokens: varies widely

When accepting user input, you may receive amounts in human-readable form (e.g., "1.5 USDC"). Convert to raw amounts before transfer:

```rust
let raw_amount = human_amount * 10_u64.pow(mint.decimals as u32);
```

The client typically handles this conversion, but your program should work with raw amounts directly.

## Error Handling

The transfer can fail for several reasons:

**Insufficient balance**: The maker's token account does not have enough tokens. The error is `TokenError::InsufficientFunds`.

**Account mismatch**: The mint or authority does not match expectations. This should not happen with proper validation.

**Program mismatch**: The token program ID does not match. Validation should catch this.

Anchor propagates these errors to the caller, who receives a transaction failure with the error details.

## Practical Exercise

Implement the token deposit in your make_offer instruction. Use transfer_checked with proper decimal handling. Test with sufficient balance to verify the deposit works. Test with insufficient balance to verify proper error propagation.

## Key Takeaways

Token deposit uses transfer_checked CPI to the Token Program. The maker signs as authority for their token account. Decimal precision must match the token mint's specification. Errors propagate correctly through Anchor's error handling.

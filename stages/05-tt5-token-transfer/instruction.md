In this stage, you'll implement a reusable token transfer function using Cross-Program Invocation (CPI). This is a critical component of your swap program, enabling secure token transfers between accounts.

## Understanding Cross-Program Invocation (CPI)

Cross-Program Invocation allows your Solana program to call instructions from other programs. In your swap program, you'll use CPI to:
- Transfer tokens using the SPL Token Program
- Create associated token accounts using the Associated Token Program
- Perform other operations provided by system and token programs

CPI is essential because:
- Your swap program doesn't implement token operations itself
- The SPL Token Program handles all token-related functionality
- CPI enables composable, modular smart contracts

## Prerequisite Reading

To understand CPI and token transfers, review:

- **Anchor CPI Guide**: Read the [Cross-Program Invocation Documentation](https://www.anchor-lang.com/docs/cpi) to understand how Anchor simplifies CPI calls.
- **SPL Token Program**: Review the [SPL Token Program Documentation](https://spl.solana.com/token) to understand token operations.
- **Transfer vs TransferChecked**: Learn about the difference in the [Token Program Instructions](https://spl.solana.com/token#instructions).
- **Token Decimals**: Understand how token amounts work in the [Solana Token FAQ](https://solanacookbook.com/tokens/tokens.html).

## Implement the transfer_tokens Function

Add this function to `instructions/shared.rs`:

```rust
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

pub fn transfer_tokens<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: &u64,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &Signer<'info>,
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    let transfer_accounts_options = TransferChecked {
        from: from.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };

    let cpi_context = CpiContext::new(token_program.to_account_info(), transfer_accounts_options);

    transfer_checked(cpi_context, *amount, mint.decimals)
}
```

## Understanding the Implementation

Let's examine each component of the function:

### Function Parameters

- **`from: &InterfaceAccount<'info, TokenAccount>`**: The source token account (tokens will be debited from this account)
  - Reference (`&`) because we're reading from it
  - `InterfaceAccount` supports both Token and Token-2022 programs

- **`to: &InterfaceAccount<'info, TokenAccount>`**: The destination token account (tokens will be credited to this account)
  - Similar structure to `from`
  - Can be the vault, maker's account, or taker's account depending on context

- **`amount: &u64`**: The amount of tokens to transfer
  - Reference to a 64-bit unsigned integer
  - Represents the token amount in the smallest unit (like wei in Ethereum)

- **`mint: &InterfaceAccount<'info, Mint>`**: The mint account for the token being transferred
  - Required for `transfer_checked` to validate decimals
  - Ensures the amount is valid for this token's decimal precision

- **`authority: &Signer<'info>`**: The account that owns the source token account
  - Must sign the transaction
  - Authorizes the transfer from their account
  - Can be a user or a PDA (when transferring from vault)

- **`token_program: &Interface<'info, TokenInterface>`**: The token program to use
  - `Interface` allows compatibility with both Token and Token-2022
  - Required for the CPI call

### TransferChecked Accounts

- **`TransferChecked` struct**: Defines the accounts required by the SPL Token Program's `transfer_checked` instruction
  - `from`: Source token account
  - `mint`: Token mint account
  - `to`: Destination token account
  - `authority`: Account authorized to transfer from the source

### CPI Context

- **`CpiContext::new(...)`**: Creates the context for the Cross-Program Invocation
  - First argument: The program being called (token program)
  - Second argument: The accounts required by that program

### Transfer Execution

- **`transfer_checked(cpi_context, *amount, mint.decimals)`**: Executes the token transfer
  - `cpi_context`: The CPI context with all required accounts
  - `*amount`: Dereferences the amount (converts from reference to value)
  - `mint.decimals`: Validates that the amount is valid for the token's decimal precision

## Why TransferChecked Instead of Transfer?

The SPL Token Program provides two transfer instructions:

1. **`transfer`**: Simpler, but less safe
   - Doesn't validate the mint
   - Doesn't check decimal precision
   - Potentially vulnerable to precision errors

2. **`transfer_checked`**: More secure, recommended
   - Validates the mint account
   - Checks decimal precision
   - Prevents precision errors
   - **Always use `transfer_checked` for security**

## Understanding Token Decimals

Token decimals are crucial for correct amount handling:

- **USDC**: 6 decimals (1 USDC = 1,000,000 base units)
- **SOL**: 9 decimals (1 SOL = 1,000,000,000 lamports)
- **Generic tokens**: Can have 0-18 decimals

Example: To transfer 1.5 USDC:
```rust
let amount = 1_500_000;  // 1.5 × 10^6
transfer_checked(cpi_context, amount, 6);
```

The `transfer_checked` instruction validates that the amount is within the valid range for the given decimals.

## CPI Security Considerations

When using CPI, keep these security principles in mind:

1. **Account Validation**: Always validate accounts before passing them to CPI
   - Anchor's account constraints help with this
   - Double-check mint addresses and authorities

2. **Authority Control**: Ensure only authorized accounts can transfer
   - The `authority` must sign the transaction
   - For PDAs, the program signs via CPI

3. **Amount Validation**: Use `transfer_checked` to validate amounts
   - Prevents overflow/underflow
   - Ensures correct decimal precision

4. **Program Compatibility**: Use `Interface` for maximum compatibility
   - Supports both Token and Token-2022 programs
   - Future-proofing for token program upgrades

## Test Cases

| Test | Expected Result | Purpose |
|------|-----------------|---------|
| Function compiles | No syntax errors | Ensures proper Rust syntax |
| CPI accounts | Correct account setup | Validates proper CPI configuration |
| Amount validation | Decimal handling correct | Confirms proper token amount handling |
| Transfer execution | Tokens moved correctly | Verifies CPI transfer works |
| Authority check | Only owner can transfer | Confirms security validation |

## Notes

- Always use `transfer_checked` instead of `transfer` for security
- The `authority` parameter must be a `Signer` for user-owned accounts
- For PDA-owned accounts, you'll use `CpiContext::new_with_signer` in later stages
- `InterfaceAccount` and `Interface` enable compatibility with both Token and Token-2022 programs
- Token amounts are always in the smallest unit (like cents for dollars)
- The mint's decimals field is critical for correct amount interpretation

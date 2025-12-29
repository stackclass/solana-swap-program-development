# SPL Token Basics

This stage introduces SPL tokens, Solana's standard for creating and managing fungible tokens. Understanding tokens is essential for building swap programs that exchange value between parties.

## What are SPL Tokens

SPL (Solana Program Library) tokens are the standard token implementation on Solana. Unlike native SOL, which is built into the protocol, SPL tokens are implemented as programs that follow a standardized interface. This standardization enables interoperability—any program can work with any SPL token following the standard.

Tokens on Solana differ fundamentally from accounts. A token account does not store SOL directly; instead, it maintains a balance of a specific token type. The token type is defined by a token mint, which serves as the template for all tokens of that type.

The Token Program manages all SPL token operations including minting new tokens, transferring between accounts, burning tokens, and freezing accounts. Your swap program will interact with this program extensively.

## Token Mints

A token mint is the authoritative source for a token type. The mint defines the token's properties: how many decimal places it uses, who can mint new tokens, whether accounts can be frozen, and the current supply.

Every token has a unique mint address. When you see an address like `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`, this is a mint address representing a specific token type—in this case, USD Coin (USDC).

Mints do not store individual balances. They track aggregate statistics like total supply and maintain authority over minting new tokens. To receive tokens, users create token accounts associated with a specific mint.

## Token Accounts

A token account is a specialized account type managed by the Token Program. Each token account holds a balance of exactly one token type, specified by its associated mint. The account also tracks the delegate (if any) authorized to transfer tokens on behalf of the owner.

Token accounts contain several key pieces of information. The mint field references which token type this account holds. The owner field specifies the public key that controls the account. The amount field holds the current token balance. Optional fields include delegate authority and locked balance for pending transfers.

To receive tokens of a specific type, a user needs a token account associated with that mint. Multiple users can hold token accounts for the same mint, each maintaining their own independent balance.

## Associated Token Accounts

Associated Token Accounts (ATA) provide a standardized way to derive token account addresses. Rather than requiring users to specify arbitrary addresses, ATAs are deterministically derived from a wallet address and token mint:

```
ATA = find_associated_token_address(wallet, mint)
```

This derivation follows a predictable formula: the ATA is always at address `quote` + wallet + mint, where `quote` is a specific program-derived address.

The Associated Token Program provides a convenient instruction to create these accounts automatically:

```rust
#[account(
    init,
    payer = user,
    associated_token::mint = token_mint,
    associated_token::authority = user
)]
pub user_token_account: InterfaceAccount<'info, TokenAccount>,
```

ATAs are the recommended approach for user token accounts because they provide predictable addresses, simplify user experience, and enable wallets and interfaces to discover user holdings automatically.

## Token Interfaces

Modern Solana development uses token interfaces (`Mint`, `TokenAccount`, `TokenInterface`) instead of the legacy Token Program types. These interfaces work with both the original Token Program and the newer Token Extensions Program.

```rust
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct SwapContext<'info> {
    pub token_mint_a: InterfaceAccount<'info, Mint>,
    pub token_mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = user
    )]
    pub user_token_a: InterfaceAccount<'info, TokenAccount>,
}
```

The `Mint` interface provides access to mint-level information like decimal count and supply. The `TokenAccount` interface exposes balance and owner information. The `TokenInterface` represents the token program itself, enabling CPI calls.

## Token Program ID

SPL tokens can be managed by either the original Token Program or the newer Token Extensions Program (TEP). Your program should specify which program it expects:

```rust
pub token_program: Interface<'info, TokenInterface>,
```

The token program ID is embedded in the `TokenInterface` type, allowing the framework to validate program matches during account validation.

## Creating Tokens for Testing

During development, you will need test tokens to exercise your swap program. The Solana CLI can create token mints and mint tokens for testing:

```bash
spl-token create-token
spl-token create-account <MINT_ADDRESS>
spl-token mint <MINT_ADDRESS> <AMOUNT>
```

These commands create a new token type, an associated token account for your wallet, and mint tokens to your account. You can then use these tokens in local testing.

## Practical Exercise

Create an Anchor program that accepts two token accounts as input and prints their balances. Deploy to devnet, create test token accounts with the CLI, and run your program to observe how token account data is accessed.

Experiment with different token types (USDC, SOL wrapped as spl-token) to understand how the same interface handles different token mints.

## Key Takeaways

SPL tokens extend Solana's account model to support arbitrary fungible assets. Understanding the relationship between mints (token definitions) and accounts (token holdings) is crucial for any program handling user value. Associated Token Accounts provide the standard approach for user token holdings, and token interfaces enable your program to work with both legacy and extended token programs.

# Solana Account Model

This stage explores Solana's distinctive account model, which differs significantly from other blockchain platforms. Understanding accounts is fundamental to writing correct Solana programs.

## What is an Account

An account on Solana is a record stored on the blockchain that holds data and SOL balance. Every piece of state in a Solana program exists within an account. Accounts are identified by their public key (address) and can hold up to 10 megabytes of data, though most programs use much smaller accounts.

Each account has several key properties: the public key identifying it, the lamport balance (SOL holdings), the data field containing program-specific information, the owning program (which can modify the data), and a boolean indicating whether the account is executable like a program itself.

Accounts consume rent in lamports proportional to the data they store. Programs can make accounts rent-exempt by funding them with enough SOL to cover two years of rent minimum at current rates.

## Types of Accounts

Solana distinguishes between several account types that serve different purposes in program execution.

**System Accounts** are created and controlled by the SystemProgram. They hold no custom data (the data field is empty) and exist primarily to hold SOL balance. These accounts are used to hold user funds and pay for transaction fees.

**Program Accounts** contain executable bytecode. When you deploy a Solana program, the compiled bytecode is stored in an executable account. These accounts have the executable flag set to true and cannot hold lamport balances (rent is paid from a separate account).

**Data Accounts** store program-specific state. These accounts are created by programs and hold structured data specific to the program's logic. The swap program you will build creates data accounts to store offer information.

**Token Accounts** are specialized accounts managed by the Token Program. They hold token balances for specific token mints and are essential for any program that needs to handle SPL tokens.

## Account Structure in Anchor

Anchor provides the `#[account]` attribute to define account structures in Rust. This macro generates serialization code and account type information:

```rust
#[account]
pub struct Offer {
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_b_wanted_amount: u64,
    pub bump: u8,
}
```

The `#[account]` attribute automatically adds an 8-byte discriminator to the account data, preventing type confusion and enabling Anchor to verify account types at runtime.

## Account Validation

Anchor's `#[derive(Accounts)]` macro generates validation code that checks account relationships and constraints:

```rust
#[derive(Accounts)]
pub struct CreateOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        init,
        payer = maker,
        space = 8 + std::mem::size_of::<Offer>(),
        seeds = [b"offer", maker.key().as_ref()],
        bump
    )]
    pub offer: Account<'info, Offer>,
    pub system_program: Program<'info, System>,
}
```

The `#[account(...)]` attribute specifies validation constraints. The `mut` constraint marks accounts that will be modified. The `init` constraint creates the account if it does not exist. The `payer` specifies who pays for account creation costs. The `space` determines data allocation. Seeds and bump work together to derive program-derived addresses.

## Account Lifecycle

Accounts follow a specific lifecycle in Solana programs.

**Creation** occurs when a program calls SystemProgram's create_account instruction. The creating transaction specifies the new account's address, lamport transfer, and data initialization. Anchor's `init` constraint handles this automatically.

**Modification** happens when the owning program updates account data. Only the program specified in the account's owner field can modify its data. Other programs can read the data but cannot change it.

**Closing** releases the account's lamports back to a specified destination. The account data is zeroed out, and the account ceases to exist. Anchor's `close` constraint handles cleanup:

```rust
#[account(mut, close = maker)]
pub offer: Account<'info, Offer>,
```

## Rent and Rent-Exemption

Accounts must maintain a minimum lamport balance proportional to their data size. This "rent" is collected periodically, and accounts that fall below the minimum are purged from the ledger.

Modern Solana development typically makes accounts rent-exempt by funding them with enough lamports for two years of rent. This one-time funding eliminates ongoing rent payments and ensures the account persists indefinitely.

Anchor calculates required space automatically for simple accounts. For complex accounts with variable-size data, you must account for the 8-byte discriminator and field sizes.

## Practical Exercise

Create a simple Anchor program that initializes and modifies a custom account. Observe how the account appears in Solana Explorer. Notice the account's data field contains your serialized struct, and the owner field points to your deployed program.

Experiment with different account sizes and observe how the rent requirement changes. Try closing an account and verify the lamports return to the destination.

## Key Takeaways

Solana's account model separates code (programs) from state (accounts), enabling efficient parallel execution and upgradeable logic. Understanding account creation, ownership, validation, and lifecycle is essential before building more complex programs like the swap application.

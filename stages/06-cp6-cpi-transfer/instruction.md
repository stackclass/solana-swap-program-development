# CPI Transfer

This stage explores Cross-Program Invocation (CPI), the mechanism enabling Solana programs to interact with each other. You will learn how to invoke the Token Program to transfer tokens from within your own program.

## What is Cross-Program Invocation

Cross-Program Invocation allows a Solana program to call functions in another program. This capability is essential because it enables composition—your program can leverage functionality provided by other programs without reimplementing it.

The Token Program provides token transfer functionality, but your custom program cannot access this directly. Instead, your program invokes the Token Program via CPI, passing the appropriate accounts and data. The Token Program executes the transfer and returns control to your program.

CPI is fundamental to Solana's design. Programs are intentionally isolated; they cannot directly modify another program's state. CPI provides the only sanctioned mechanism for inter-program communication, and it only works when the called program provides public instruction entrypoints.

## CPI Mechanics

When your program performs CPI, several things happen. Your program prepares accounts and instruction data, then invokes the target program. The runtime executes the target program's instruction, potentially modifying accounts. Upon completion, control returns to your program with any error status.

The critical insight is that CPI maintains the transaction atomicity guarantee. Either the entire transaction succeeds, including all CPI calls, or everything reverts. This means if a CPI fails, all preceding instructions in the transaction also fail.

CPI calls also support program-derived address signing. When a PDA-controlled account needs to perform actions that require authority, the calling program can authorize the PDA to act on its own behalf.

## Transfer Checked Instruction

The Token Program's `transfer_checked` instruction is the preferred method for token transfers. Unlike basic `transfer`, `transfer_checked` verifies the decimal precision of the amount being transferred, preventing common mistakes with token amounts.

The instruction requires these accounts: the source token account holding tokens to transfer, the token mint (for decimal verification), the destination token account receiving tokens, and the authority (signer) authorizing the transfer.

```rust
use anchor_spl::token_interface::{
    transfer_checked, TransferChecked, Mint, TokenAccount, TokenInterface
};

let cpi_accounts = TransferChecked {
    from: source_account.to_account_info(),
    mint: mint.to_account_info(),
    to: destination.to_account_info(),
    authority: authority.to_account_info(),
};

let cpi_context = CpiContext::new(
    token_program.to_account_info(),
    cpi_accounts,
);

transfer_checked(cpi_context, amount, mint.decimals)?;
```

The `CpiContext::new` function creates the CPI context with the called program's ID inferred from the first account. The `mint.decimals` field ensures the amount is interpreted correctly based on the token's precision.

## Complete Transfer Helper

Most programs encapsulate CPI logic in helper functions for reuse and clarity:

```rust
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

This helper abstracts the CPI complexity behind a clean interface. Callers specify source, destination, amount, mint, and authority without understanding CPI internals.

## CPI with Program-Derived Addresses

When transferring tokens from a PDA-controlled vault, the vault itself (the PDA) must sign the transfer. Since a PDA has no private key, the calling program must provide the PDA's signing seeds:

```rust
let seeds = &[
    b"offer",
    maker.key().as_ref(),
    &[bump],
];
let signer_seeds = [&[seeds[..]]];

let cpi_context = CpiContext::new_with_signer(
    token_program.to_account_info(),
    cpi_accounts,
    signer_seeds,
);
```

The `CpiContext::new_with_signer` function adds PDA signing capability. The runtime uses the provided seeds to derive the PDA and sign on its behalf. This pattern is essential for vault operations where the program controls tokens on users' behalf.

## Error Propagation

CPI calls return `Result` types that may contain errors from the called program. These errors propagate to your caller, enabling proper error handling:

```rust
match transfer_tokens(...) {
    Ok(()) => println!("Transfer successful"),
    Err(ProgramError::InsufficientFunds) => {
        println!("Not enough tokens to transfer");
    }
    Err(e) => {
        println!("Transfer failed: {:?}", e);
    }
}
```

Anchor's `?` operator automatically propagates errors, causing your instruction to fail if the CPI fails. This maintains the atomicity guarantee—all-or-nothing transactions.

## Practical Exercise

Create a program with a transfer_tokens helper function. Deploy it and write a test that creates token accounts, mints tokens, and calls your program to transfer between accounts. Verify the balance changes on-chain.

Experiment with insufficient balance errors to understand how Token Program errors propagate through your program.

## Key Takeaways

Cross-Program Invocation enables your program to leverage existing infrastructure like the Token Program. The `transfer_checked` instruction provides secure token transfers with decimal verification. Helper functions abstract CPI complexity. PDA signing via `new_with_signer` enables program-controlled token operations. Error propagation maintains transaction atomicity.

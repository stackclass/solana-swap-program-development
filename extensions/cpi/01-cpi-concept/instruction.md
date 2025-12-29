# CPI Concept

This stage introduces Cross-Program Invocation (CPI), the mechanism for Solana programs to communicate and invoke functionality in other programs.

## What is Cross-Program Invocation

Cross-Program Invocation allows a Solana program to call functions in another deployed program. This is how programs like your swap program can use the Token Program to transfer tokens without reimplementing token logic.

CPI is essential because Solana programs are isolated. A program cannot directly access another program's state or call its functions directly. Instead, it must make a CPI request, which the runtime processes by invoking the target program.

## How CPI Works

When your program makes a CPI:

1. Your program prepares accounts and instruction data
2. Your program invokes the target program via CPI
3. The runtime executes the target program's instruction
4. The target program returns success or error
5. Control returns to your program

The critical feature is atomicity. If the CPI fails, your entire transaction fails. This prevents partial execution where some state changes but others do not.

## CPI vs Regular Function Calls

Unlike regular function calls, CPI involves separate programs that may be written in different languages and deployed independently. The calling program does not have direct access to the callee's memory or state.

```rust
// Regular function call - same program
helper_function()?;

// CPI - different program
token_program.transfer()?;
```

The CPI call serializes instruction data, sets up accounts, and invokes the target program through the runtime.

## Common CPI Targets

Your swap program will frequently CPI to:

**Token Program**: For all token operations (transfer, mint, burn, close)

**SystemProgram**: For account creation and lamport operations

**AssociatedTokenProgram**: For creating associated token accounts

**Your own program**: For complex instructions that delegate to themselves

## The CPI Interface

CPI uses account information and instruction data:

```rust
let cpi_accounts = TransferChecked {
    from: source_account.to_account_info(),
    mint: mint.to_account_info(),
    to: destination.to_account_info(),
    authority: authority.to_account_info(),
};

let cpi_context = CpiContext::new(
    target_program.to_account_info(),
    cpi_accounts,
);
```

The `to_account_info()` conversion provides the account metadata the runtime needs.

## Practical Exercise

Identify all the CPI calls in your swap program. For each one, identify the target program, the purpose, and what accounts are involved. Understanding this will help you debug issues and design new features.

## Key Takeaways

CPI enables programs to invoke other programs. CPI calls are atomic—all or nothing. The Token Program is a common CPI target. CPI requires preparing accounts and context. Understanding CPI is essential for any Solana program.

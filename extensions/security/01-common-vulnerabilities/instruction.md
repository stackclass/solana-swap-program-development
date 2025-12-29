# Common Vulnerabilities

This stage explores common smart contract vulnerabilities that affect Solana programs, helping you identify and prevent security issues in your swap program.

## Introduction to Vulnerabilities

Smart contracts manage valuable assets and must be designed with security as a primary concern. Unlike traditional applications where bugs cause inconvenience, smart contract bugs can lead to permanent loss of funds.

Understanding common vulnerabilities is essential before writing production code. This knowledge helps you design defensively and recognize potential issues during code review.

## Reentrancy Attacks

Reentrancy occurs when a function makes an external call to an untrusted contract, and that contract recursively calls back into the original function before it completes. This can drain funds by repeatedly withdrawing before state updates.

In Solana, reentrancy is less common than in Ethereum due to the lack of callback mechanisms, but it can still occur through CPI calls. If your program calls another program that could invoke your program again, reentrancy is possible.

The mitigation is simple: update all state before making external calls:

```rust
// BAD: External call before state update
pub fn vulnerable_function(ctx: Context<Vulnerable>) -> Result<()> {
    external_program_call()?;
    ctx.accounts.user.balance -= amount;  // Too late!
    Ok(())
}

// GOOD: State update before external call
pub fn safe_function(ctx: Context<Safe>) -> Result<()> {
    ctx.accounts.user.balance -= amount;
    external_program_call()?;
    Ok(())
}
```

## Access Control Flaws

Access control vulnerabilities occur when unauthorized users can access restricted functions. Always verify the caller's identity and authorization level.

```rust
// BAD: No access control
pub fn admin_action(ctx: Context<AdminOnly>) -> Result<()> {
    // Anyone can call this!
    ctx.accounts.data.value = 42;
    Ok(())
}

// GOOD: Signer verification
pub fn admin_action(ctx: Context<AdminOnly>) -> Result<()> {
    require!(ctx.accounts.admin.is_signer, ErrorCode::NotAuthorized);
    ctx.accounts.data.value = 42;
    Ok(())
}
```

## Integer Overflow and Underflow

Rust prevents integer overflow by default in debug builds (panics) and wraps in release builds. For financial calculations, explicit handling is important:

```rust
// Using checked arithmetic
let new_balance = old_balance.checked_sub(amount)
    .ok_or(ErrorCode::InsufficientBalance)?;

let total = a.checked_add(b)
    .ok_or(ErrorCode::Overflow)?;
```

Anchor's `overflow-checks = true` in Cargo.toml enables runtime checks in release builds.

## Validation Bypass

Failing to validate account relationships or input values creates attack surfaces:

```rust
// BAD: No validation
#[derive(Accounts)]
pub struct Transfer<'info> {
    pub from: Account<'info, TokenAccount>,
    pub to: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
}

// GOOD: Proper validation
#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(
        mut,
        has_one = authority
    )]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
}
```

## Practical Exercise

Review your swap program code for these vulnerabilities. Identify any places where state updates happen after external calls. Verify all access control is properly implemented. Ensure all inputs are validated.

## Key Takeaways

Reentrancy is mitigated by updating state before external calls. Access control must verify identity before allowing actions. Use checked arithmetic for financial calculations. Validation constraints prevent invalid state.

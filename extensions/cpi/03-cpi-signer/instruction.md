# CPI Signer

This stage teaches how to use PDA signing in CPI calls, enabling program-controlled accounts to authorize operations.

## The PDA Signing Challenge

When a regular user transfers tokens, they sign the transaction with their private key. But when the offer PDA needs to authorize a vault transfer, it has no private key. The program must provide the signing capability.

CPI signing provides this capability. By passing the PDA's derivation seeds to the runtime, the program can sign on behalf of the PDA.

## Creating a CPI Context with Signer

```rust
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
```

The key difference is `CpiContext::new_with_signer` instead of `CpiContext::new`.

## Seed Array Structure

The `signer_seeds` parameter expects a specific structure:

```rust
let signer_seeds: [&[u8]; 1] = [&[b"offer", maker_key, id_bytes, &[bump]][..]];
```

Each element is a seed slice. For a single PDA, this is a one-element array containing all seeds concatenated.

For multiple PDAs, you would provide multiple seed slices:

```rust
let signer_seeds = [
    &inner_pda_seeds[..],
    &outer_pda_seeds[..],
];
```

## Authority Account

The authority in the CPI accounts must be the PDA's account info:

```rust
authority: ctx.accounts.offer.to_account_info(),
```

The runtime verifies that the provided authority can be derived from the signer seeds. If not, the CPI fails.

## Common Mistakes

**Wrong seeds**: Using different seeds than during PDA derivation causes signature failure.

**Missing bump**: Forgetting the bump in seeds produces wrong address.

**Wrong account as authority**: Using the wrong account as authority causes validation failure.

**Slice vs array**: Using `[seeds[..]]` vs `[&seeds[..]]` incorrectly.

## Practical Exercise

Implement vault withdrawal with PDA signing. Verify that without correct seeds, the transfer fails. Experiment with wrong seeds to see the error.

## Key Takeaways

CpiContext::new_with_signer enables PDA signing. Seeds must exactly match PDA derivation. The authority must be the PDA account. Incorrect seeds cause signature failures.

# Withdraw from Vault

This stage teaches how to implement the withdrawal from the vault to the taker, using PDA signing to authorize the transfer.

## Vault Withdrawal Challenge

The vault is controlled by the offer PDA, which has no private key. The program must sign on behalf of the PDA to authorize the transfer. This is where PDA signing comes in.

The transfer moves tokens from the vault to the taker's token account. The offer PDA is the authority for the vault, so the program must provide PDA signing seeds.

## Implementing PDA-Signed Transfer

```rust
use anchor_spl::token_interface::{
    transfer_checked, TransferChecked, Mint, TokenAccount, TokenInterface
};

pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
    // First, transfer payment (covered in previous extension)
    
    // Then, transfer from vault to taker with PDA signing
    let seeds = &[
        b"offer",
        ctx.accounts.maker.key().as_ref(),
        &ctx.accounts.offer.id.to_le_bytes(),
        &[ctx.accounts.offer.bump],
    ];
    let signer_seeds = [&seeds[..]];
    
    let vault_transfer = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        mint: ctx.accounts.token_mint_a.to_account_info(),
        to: ctx.accounts.taker_token_account_a.to_account_info(),
        authority: ctx.accounts.offer.to_account_info(),
    };
    
    let vault_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        vault_transfer,
        signer_seeds,
    );
    
    transfer_checked(
        vault_ctx,
        ctx.accounts.vault.amount,
        ctx.accounts.token_mint_a.decimals,
    )?;
    
    Ok(())
}
```

## CPI with Signer

The `CpiContext::new_with_signer` function creates a context that includes PDA signing:

```rust
let vault_ctx = CpiContext::new_with_signer(
    token_program_account_info,
    transfer_accounts,
    signer_seeds,
);
```

The `signer_seeds` parameter is a slice of seed slices. Each inner slice represents one signing derivation. For our case, we provide the offer PDA seeds.

## Transfer Amount

The transfer amount is the vault's current balance:

```rust
ctx.accounts.vault.amount
```

This should equal the amount deposited during make_offer, but reading from the account ensures accuracy. If any fees were deducted, the current balance reflects that.

## Closing the Vault

After transferring all tokens, close the vault account to recover remaining lamports:

```rust
use anchor_spl::token_interface::{close_account, CloseAccount};

let close_accounts = CloseAccount {
    account: ctx.accounts.vault.to_account_info(),
    destination: ctx.accounts.taker.to_account_info(),
    authority: ctx.accounts.offer.to_account_info(),
};

let close_ctx = CpiContext::new_with_signer(
    ctx.accounts.token_program.to_account_info(),
    close_accounts,
    signer_seeds,
);

close_account(close_ctx)?;
```

The close account instruction zeroes out the account data and transfers remaining lamports to the destination.

## Practical Exercise

Implement the vault withdrawal in your take_offer instruction. Use PDA signing with correct seeds. Transfer the full vault balance. Close the vault account after transfer.

Test that the taker receives the offered tokens. Verify the vault is properly closed.

## Key Takeaways

Vault withdrawal requires PDA signing because the offer controls the vault. CpiContext::new_with_signer provides the signing capability. Seeds must exactly match those used during offer creation. The full vault balance is transferred. close_account recovers remaining lamports.

# Offer PDA Integration

This stage teaches how to integrate the offer PDA with the overall swap program architecture, including vault control and state management.

## Offer as Vault Authority

The offer PDA serves as the authority for the associated vault account:

```rust
#[account(
    init,
    payer = maker,
    associated_token::mint = token_mint_a,
    associated_token::authority = offer
)]
pub vault: InterfaceAccount<'info, TokenAccount>,
```

When the offer is created, the vault is initialized with the offer as its authority. This means only the offer (via PDA signing) can authorize transfers from the vault.

The architecture creates a clear hierarchy:
- The maker creates the offer
- The offer controls the vault
- The program authorizes actions on behalf of the offer

## Storing PDA Information

The offer must store information needed for future PDA operations:

```rust
#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_b_wanted_amount: u64,
    pub bump: u8,  // Canonical bump for PDA signing
}
```

The `bump` field is critical—without it, the program cannot reconstruct the PDA signing authority for vault operations. Anchor's `bump` constraint automatically stores the canonical bump when creating the offer.

## Reconstructing PDA in Handlers

During take_offer, the program reconstructs the PDA to sign for vault transfers:

```rust
pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
    let seeds = &[
        b"offer",
        ctx.accounts.maker.key().as_ref(),
        &ctx.accounts.offer.id.to_le_bytes(),
        &[ctx.accounts.offer.bump],
    ];
    let signer_seeds = [&seeds[..]];
    
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
        signer_seeds,
    );
    
    transfer_checked(cpi_ctx, /* ... */)?;
    
    Ok(())
}
```

The seeds must exactly match those used during offer creation. Any difference produces a different PDA address, and the signature will be rejected.

## Offer State Transitions

The offer account tracks the swap's progress through its lifecycle:

**Open State**: The offer exists and tokens are held in the vault. Any taker can accept the offer.

**Taken State**: The offer is closed after a successful swap. The maker has received payment and the taker has received the offered tokens.

**Canceled State**: The offer is closed without a swap. The maker recovers tokens from the vault.

The current implementation uses account closure to indicate state transitions. A closed offer cannot be taken again.

## Integration Checklist

When integrating offer PDA with your program, ensure:

- [ ] Seeds uniquely identify the offer (maker + ID)
- [ ] The bump is stored in the offer struct
- [ ] The vault uses offer as authority
- [ ] take_offer reconstructs PDA with correct seeds
- [ ] Validation uses has_one constraints
- [ ] The offer closes properly after swap

## Common Integration Issues

**Seed mismatch**: Using different seeds in make_offer vs take_offer causes validation failure. Double-check seed composition.

**Bump not stored**: Forgetting to add the bump field or not using it in CPI causes signature failures. Ensure bump is stored and retrieved correctly.

**Authority mismatch**: If the vault's authority does not match the offer PDA, transfers fail. Verify associated_token::authority = offer in account struct.

## Practical Exercise

Trace the complete flow of offer creation and acceptance. Identify where the PDA is derived, where the bump is stored, and where the PDA is reconstructed for signing. Draw a diagram showing the relationships between maker, offer, vault, and taker.

Implement each integration point in your program and test thoroughly.

## Key Takeaways

The offer PDA controls the associated vault account. Store the canonical bump for later PDA signing. Seeds must match exactly between creation and usage. The offer lifecycle is tracked through account closure. Integration requires attention to seeds, bump, and authority.

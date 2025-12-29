# Vault Creation

This stage teaches how to create vault accounts and initialize them with deposited tokens during the make_offer flow.

## Creating the Vault Account

The vault account is an Associated Token Account owned by the offer PDA. Anchor's account constraints handle the creation automatically:

```rust
#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    
    pub token_mint_a: InterfaceAccount<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker
    )]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        init,
        payer = maker,
        space = 8 + std::mem::size_of::<Offer>(),
        seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub offer: Account<'info, Offer>,
    
    #[account(
        init,
        payer = maker,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
```

The `associated_token::authority = offer` constraint is crucial—it tells Anchor to set the offer PDA as the vault's authority. This means only the program (via PDA signing) can authorize transfers from the vault.

## Account Dependencies

Vault creation has dependencies that must be satisfied. The token mint must exist before creating the associated token account. The offer account must be created before it can serve as authority for the vault. The maker must have a token account with sufficient balance for the deposit.

Anchor validates these dependencies during instruction processing. If any required account is missing or invalid, the entire instruction fails before any state changes occur.

## Depositing Tokens to the Vault

After creating the vault, tokens must be transferred from the maker's token account. This requires a CPI to the Token Program:

```rust
pub fn make_offer(
    ctx: Context<MakeOffer>,
    id: u64,
    token_a_offered_amount: u64,
    token_b_wanted_amount: u64,
) -> Result<()> {
    // Transfer tokens from maker to vault
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

The maker signs the transfer as the authority of their token account. This is a regular user signature, not PDA signing—the maker authorizes moving their own tokens.

## Space and Rent Considerations

Token accounts have a fixed size determined by the Token Program. You do not need to specify space for the vault—Anchor calculates the correct size automatically based on the associated token program.

However, the vault account must be rent-exempt. The lamports needed for rent are deducted from the maker's account during creation. The maker pays for both the offer account space and the vault account rent.

For high-volume applications, consider accumulating rent in a dedicated account rather than requiring makers to pay for each vault.

## Verification and Validation

After deposit, verify the transfer succeeded by checking the vault balance:

```rust
let vault_balance = ctx.accounts.vault.amount;
require!(
    vault_balance >= token_a_offered_amount,
    ErrorCode::DepositFailed
);
```

In practice, the CPI will fail if the transfer cannot complete, so explicit balance checks are optional but can provide clearer error messages.

## Practical Exercise

Implement the vault creation and token deposit in your make_offer instruction. Use Anchor's associated token constraints for account creation. Implement the transfer_checked CPI to move tokens from maker to vault.

Test with sufficient balance to verify deposit works. Test with insufficient balance to verify proper error handling.

## Key Takeaways

Vault creation uses associated_token constraints with offer PDA as authority. Token deposit requires CPI to the Token Program with maker signature. The maker pays rent for both offer and vault accounts. CPI failure properly propagates errors to the caller.

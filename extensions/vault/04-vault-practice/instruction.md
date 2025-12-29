# Vault Practice

This stage provides hands-on exercises to implement secure vault handling in your swap program.

## Exercise 1: Create Vault Account Definition

Define the vault account in your MakeOffer struct. The vault should be an Associated Token Account with the offer as authority:

```rust
#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    
    pub token_mint_a: InterfaceAccount<'info, Mint>,
    pub token_mint_b: InterfaceAccount<'info, Mint>,
    
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
    
    // Add vault account here
    // Hint: Use associated_token constraints
}
```

Consider what mint the vault should use and who should be its authority.

## Exercise 2: Implement Token Deposit

Implement the token deposit logic in your make_offer instruction. The deposit should:

1. Create a TransferChecked CPI context
2. Use the maker as signing authority
3. Transfer the offered amount to the vault
4. Use the correct decimal precision from the mint

```rust
pub fn make_offer(
    ctx: Context<MakeOffer>,
    id: u64,
    token_a_offered_amount: u64,
    token_b_wanted_amount: u64,
) -> Result<()> {
    // Implement token transfer from maker to vault
    // Use transfer_checked for proper decimal handling
    
    Ok(())
}
```

## Exercise 3: Validate Vault in Take Offer

Define the vault account in your TakeOffer struct with proper validation:

```rust
#[derive(Accounts)]
pub struct TakeOffer<'info> {
    // ... other accounts
    
    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        // Add validation to ensure this is the correct vault
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
}
```

The vault validation should ensure it matches the offer and holds the expected token.

## Exercise 4: Implement Vault Closing

Implement the vault closing logic in take_offer. After transferring tokens to the taker, close the vault:

```rust
pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
    // 1. Transfer tokens from taker to maker (payment)
    // 2. Transfer tokens from vault to taker (offered tokens)
    // 3. Close the vault account
    
    // Hint: Use the close constraint in the account struct
    // Or manually call system_program's close_account
    
    Ok(())
}
```

Consider where the remaining lamports should go after closing.

## Verification Criteria

Your implementation is complete when:

1. The vault account is created with the offer as authority
2. Tokens are successfully deposited from maker to vault
3. The vault is validated correctly in take_offer
4. Tokens transfer from vault to taker during swap
5. The vault account is properly closed after the swap

## Common Mistakes to Avoid

Forgetting to include the associated_token_program in the account struct. This program is required for creating associated token accounts.

Using the wrong mint in vault creation. The vault must use token_mint_a (the offered token), not token_mint_b.

Not providing the correct signer for deposit. The maker must sign the transfer because the maker's token account is being debited.

Closing the vault before transfers complete. Ensure all token movements happen before closing the account.

## Next Steps

With vault implementation complete, proceed to the Offer extension to learn about designing and managing the offer data structure.

# Error Messages

This stage teaches how to write effective error messages that help with debugging and provide good user experience.

## Writing Clear Messages

Error messages should be:

**Specific**: Tell exactly what went wrong

```rust
// Vague
#[msg("Invalid data")]
InvalidData,

// Specific
#[msg("Amount must be greater than zero")]
InvalidAmount,
```

**Actionable**: Tell the user what to do

```rust
// Not actionable
#[msg("Transfer failed")]
TransferFailed,

// Actionable  
#[msg("Insufficient token balance. Please deposit more tokens before making an offer")]
InsufficientBalance,
```

**Technical but readable**: Include relevant details for debugging

```rust
#[msg("Token mint mismatch. Expected {expected}, found {found}")]
TokenMintMismatch { expected: Pubkey, found: Pubkey },
```

## Dynamic Error Messages

Anchor supports dynamic error messages with parameters:

```rust
#[error_code]
pub enum SwapError {
    #[msg("Offer {offer_id} not found")]
    OfferNotFound { offer_id: u64 },
    
    #[msg("Expected mint {expected}, got {found}")]
    MintMismatch { expected: Pubkey, found: Pubkey },
}
```

Usage:

```rust
return Err(SwapError::OfferNotFound { offer_id: id }.into());
```

## Error Categories

Organize errors by category:

**Validation Errors**: Invalid inputs or amounts

**Account Errors**: Missing or invalid accounts

**State Errors**: Invalid state transitions

**Token Errors**: Transfer failures, insufficient balance

**Security Errors**: Unauthorized access attempts

## Common Error Patterns

```rust
#[error_code]
pub enum SwapError {
    // Validation
    #[msg("Amount must be greater than zero")]
    InvalidAmount,
    #[msg("Offer ID cannot be zero")]
    InvalidOfferId,
    
    // Account
    #[msg("Offer account not found")]
    OfferNotFound,
    #[msg("Vault account mismatch")]
    VaultMismatch,
    
    // State
    #[msg("Offer is not open")]
    OfferNotOpen,
    #[msg("Offer already taken")]
    OfferTaken,
    
    // Token
    #[msg("Insufficient token balance")]
    InsufficientBalance,
    #[msg("Token transfer failed")]
    TransferFailed,
    
    // Security
    #[msg("Unauthorized: must be offer maker")]
    NotMaker,
    #[msg("Cannot trade with yourself")]
    SelfTrade,
}
```

## Error Message Best Practices

1. Start with the problem, not the solution
2. Include relevant values when possible
3. Keep messages concise but complete
4. Use consistent terminology
5. Avoid technical jargon for user-facing messages

## Practical Exercise

Review your error codes and messages. Are they specific enough? Can users understand what went wrong? Add parameters to messages that would benefit from specific values.

## Key Takeaways

Specific messages help debugging. Actionable messages help users. Dynamic messages include relevant values. Organize errors by category. Follow consistent patterns and terminology.

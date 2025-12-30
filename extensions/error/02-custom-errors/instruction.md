# Custom Errors

This stage teaches how to define and use custom error codes for your swap program.

## Defining Error Codes

Anchor uses enums to define error codes:

```rust
#[error_code]
pub enum SwapError {
    #[msg("Token amount must be greater than zero")]
    InvalidAmount,
    #[msg("Offer has already been taken")]
    OfferAlreadyTaken,
    #[msg("Cannot trade with yourself")]
    SelfTrade,
    #[msg("Vault has insufficient balance")]
    InsufficientVaultBalance,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Token mint mismatch")]
    TokenMintMismatch,
    #[msg("Transfer failed")]
    TransferFailed,
}
```

The `#[error_code]` attribute generates the necessary boilerplate for Anchor to handle these errors.

## The #[msg] Attribute

Each error can have a human-readable message:

```rust
#[msg("Token amount must be greater than zero")]
InvalidAmount,
```

This message is displayed to users when the error occurs. Keep messages clear and helpful.

## Using Custom Errors

Use custom errors with require!:

```rust
require!(amount > 0, SwapError::InvalidAmount);
require!(offer.status == OfferStatus::Open, SwapError::OfferAlreadyTaken);
require!(taker.key() != maker.key(), SwapError::SelfTrade);
```

## Error Return Type

Update your instruction signature to use the custom error:

```rust
use crate::error::SwapError;

pub fn make_offer(...) -> Result<(), SwapError> {
    require!(amount > 0, SwapError::InvalidAmount);
    Ok(())
}
```

Anchor infers the error type from context, so explicit typing is optional but can improve clarity.

## Converting from Other Errors

When a CPI fails, you may want to convert the error:

```rust
transfer_tokens(...).map_err(|_| SwapError::TransferFailed)?;
```

The `map_err` transforms the original error into your custom error.

## Error Module Organization

Keep errors in a separate module:

```rust
// programs/swap-program/src/error.rs
use anchor_lang::prelude::*;

#[error_code]
pub enum SwapError {
    // error definitions
}

// programs/swap-program/src/lib.rs
mod error;
use error::SwapError;
```

This keeps your error definitions organized and easy to find.

## Practical Exercise

Define appropriate error codes for your swap program. Identify all failure conditions and map them to specific errors. Use these errors in your validation.

## Key Takeaways

Custom errors use #[error_code] enum. #[msg] provides user-facing messages. Use require! with custom errors. map_err converts other errors. Keep errors in a separate module.

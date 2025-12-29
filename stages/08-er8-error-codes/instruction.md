In this stage, you'll implement custom error codes for your swap program. Proper error handling is crucial for creating a robust, user-friendly smart contract that provides clear feedback when things go wrong.

## Understanding Error Handling in Solana Programs

Error handling is a critical aspect of smart contract development. In Solana:

- **Errors are returned as Result types**: Functions return `Result<()>` where errors can be propagated
- **Custom error codes provide clarity**: Instead of generic errors, specific error codes help users understand what went wrong
- **Error messages are user-facing**: The error messages you define will be displayed to users when transactions fail
- **Error codes are numeric**: Each error variant is assigned a unique numeric code for programmatic handling

## Prerequisite Reading

To understand error handling in Anchor, review:

- **Anchor Error Handling**: Read the [Anchor Error Documentation](https://www.anchor-lang.com/docs/error-handling) to understand how errors work in Anchor.
- **Rust Result Type**: Review the [Rust Result Documentation](https://doc.rust-lang.org/std/result/) to understand error propagation.
- **Error Codes in Solana**: Learn about error codes in the [Solana Program Documentation](https://solana.com/docs/programs/errors).
- **Best Practices**: Review error handling best practices in the [Anchor Best Practices Guide](https://www.anchor-lang.com/docs/best-practices#error-handling).

## Implement Custom Error Codes

Add the following error enum definition to your program's `error.rs` file:

```rust
#[error_code]
pub enum SwapError {
    #[msg("Offer not found")]
    OfferNotFound,

    #[msg("Invalid token amount")]
    InvalidTokenAmount,

    #[msg("Insufficient token balance")]
    InsufficientBalance,

    #[msg("Invalid offer")]
    InvalidOffer,

    #[msg("Unauthorized access")]
    Unauthorized,
}
```

## Understanding the Error Implementation

Let's examine each component:

### Error Code Attribute

- **`#[error_code]`**: This attribute macro tells Anchor that this enum represents custom error codes
  - Automatically generates error code numbers (starting from 6000)
  - Implements the `Error` trait for the enum
  - Enables error propagation with the `?` operator

### Error Enum

- **`pub enum SwapError`**: Defines a set of possible errors that can occur in your swap program
  - `pub` makes it accessible from other modules
  - `enum` allows multiple error variants
  - Each variant represents a specific error condition

### Error Messages

- **`#[msg("...")]`**: Provides a user-friendly error message
  - Displayed to users when this error occurs
  - Should be clear, concise, and actionable
  - Helps users understand what went wrong and how to fix it

## Common Error Types for Swap Programs

Here are common error types you should implement:

### 1. OfferNotFound

```rust
#[msg("Offer not found")]
OfferNotFound,
```

**When to use**: When trying to access an offer that doesn't exist or has already been closed.

**Example scenario**: A taker tries to take an offer that was already accepted by someone else.

### 2. InvalidTokenAmount

```rust
#[msg("Invalid token amount")]
InvalidTokenAmount,
```

**When to use**: When the token amount is zero or exceeds valid limits.

**Example scenario**: A user tries to create an offer with zero tokens.

### 3. InsufficientBalance

```rust
#[msg("Insufficient token balance")]
InsufficientBalance,
```

**When to use**: When a user doesn't have enough tokens to complete the operation.

**Example scenario**: A taker tries to accept an offer but doesn't have enough Token B.

### 4. InvalidOffer

```rust
#[msg("Invalid offer")]
InvalidOffer,
```

**When to use**: When the offer data is corrupted or doesn't match expected values.

**Example scenario**: The offer references the wrong token mints.

### 5. Unauthorized

```rust
#[msg("Unauthorized access")]
Unauthorized,
```

**When to use**: When someone tries to perform an operation they're not authorized for.

**Example scenario**: A user tries to withdraw tokens from an offer they didn't create.

## Using Custom Errors in Your Program

Once you've defined your error codes, you can use them in your program:

```rust
pub fn make_offer(
    context: Context<MakeOffer>,
    id: u64,
    token_a_offered_amount: u64,
    token_b_wanted_amount: u64,
) -> Result<()> {
    // Validate token amounts
    if token_a_offered_amount == 0 {
        return Err(SwapError::InvalidTokenAmount.into());
    }

    if token_b_wanted_amount == 0 {
        return Err(SwapError::InvalidTokenAmount.into());
    }

    // Check if maker has sufficient balance
    if context.accounts.maker_token_account_a.amount < token_a_offered_amount {
        return Err(SwapError::InsufficientBalance.into());
    }

    // Continue with offer creation...
}
```

## Error Code Numbers

Anchor automatically assigns numeric codes to your error variants:

- First error: 6000
- Second error: 6001
- Third error: 6002
- And so on...

You can also specify custom error codes:

```rust
#[error_code]
pub enum SwapError {
    #[msg("Offer not found")]
    OfferNotFound = 100,  // Custom error code

    #[msg("Invalid token amount")]
    InvalidTokenAmount = 200,  // Custom error code
}
```

## Why Custom Error Codes Matter

### 1. User Experience

Clear, descriptive messages help users understand what went wrong:
- **Bad**: "Program failed: 0x1"
- **Good**: "Error: Insufficient token balance. You need 1000 USDC but only have 500."

### 2. Debugging

Specific error types make debugging easier:
- You can quickly identify which part of the code failed
- Error codes provide programmatic access to error types
- Logs become more meaningful

### 3. Testing

Well-defined errors enable comprehensive test coverage:
```typescript
it("Should fail with insufficient balance", async () => {
    try {
        await program.methods.makeOffer(...).rpc();
        assert.fail("Should have thrown an error");
    } catch (err) {
        assert.strictEqual(err.error.errorMessage, "Insufficient token balance");
    }
});
```

### 4. Maintenance

Organized error handling makes code easier to maintain:
- Centralized error definitions
- Consistent error messages
- Easy to add new error types

## Error Handling Best Practices

### 1. Be Specific

Define specific errors for different failure modes:
```rust
// Good
#[msg("Insufficient token balance")]
InsufficientBalance,

#[msg("Insufficient SOL for transaction fees")]
InsufficientFunds,

// Bad
#[msg("Not enough funds")]
NotEnoughFunds,
```

### 2. Provide Actionable Messages

Tell users what they can do to fix the problem:
```rust
// Good
#[msg("Insufficient token balance. You need at least 1000 tokens.")]
InsufficientBalance,

// Bad
#[msg("Error: balance")]
BalanceError,
```

### 3. Use Consistent Naming

Follow a consistent naming convention:
```rust
// Good
OfferNotFound
InvalidTokenAmount
InsufficientBalance

// Bad
OfferDoesntExist
BadAmount
NoTokens
```

### 4. Document Error Conditions

Add comments explaining when each error is triggered:
```rust
/// Triggered when trying to access a non-existent offer
#[msg("Offer not found")]
OfferNotFound,

/// Triggered when the token amount is zero or exceeds limits
#[msg("Invalid token amount")]
InvalidTokenAmount,
```

## Test Cases

| Test | Expected Result | Purpose |
|------|-----------------|---------|
| Error enum compiles | No syntax errors | Ensures proper Rust syntax |
| `#[error_code]` attribute present | Proper annotation | Confirms Anchor recognizes this as error enum |
| Error message defined | Clear user feedback | Validates descriptive error messaging |
| Error propagation | Errors propagate correctly | Verifies error handling works |

## Notes

- Anchor error codes start from 6000 (custom programs) to avoid conflicts with system errors
- Use `.into()` to convert custom errors to `Result` errors
- Error messages are limited to 100 characters in Solana
- Always validate inputs and return specific errors
- Consider adding error codes for edge cases and security checks
- Test error paths thoroughly to ensure proper error handling
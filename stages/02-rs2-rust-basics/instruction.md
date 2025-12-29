# Rust Basics for Solana

This stage introduces Rust programming concepts essential for Solana smart contract development. While Rust is a complex language with many features, this course focuses on the specific subset needed for blockchain program development.

## Why Rust for Blockchain

Rust combines low-level control with high-level safety guarantees, making it ideal for systems programming including blockchain development. Its ownership system prevents memory safety issues without garbage collection, and its type system catches many errors at compile time rather than runtime.

For Solana specifically, Rust provides deterministic behavior essential for consensus, compile-time verification of program correctness, and efficient execution that minimizes compute unit consumption.

## Variables and Mutability

Variables in Rust are immutable by default, meaning once assigned a value, that value cannot change. This default promotes safer code by preventing unintended modifications:

```rust
let x = 5;
println!("The value of x is: {}", x);
// x = 6; // This would cause a compile error
```

To make a variable mutable, use the `mut` keyword:

```rust
let mut y = 10;
println!("Initial value: {}", y);
y = 15;
println!("Changed value: {}", y);
```

In Solana programs, mutability often relates to accounts marked with `#[account(mut)]`, indicating the account data will be modified during instruction execution.

## Data Types

Rust provides several primitive types relevant to Solana development:

**Integer types** include `u8`, `u16`, `u32`, `u64`, `u128` for unsigned values and `i8` through `i128` for signed values. Solana frequently uses `u64` for amounts and `u32` for counts. The `usize` type represents a pointer-sized unsigned integer.

**Boolean type** is `bool` with values `true` and `false`:

```rust
let is_active: bool = true;
let has_balance: bool = false;
```

**String types** include `&str` for string slices and `String` for owned strings. Program IDs and public keys often appear as string representations.

**Array types** store fixed-size collections of the same type:

```rust
let public_key_bytes: [u8; 32] = [0u8; 32];
```

This pattern appears frequently when working with Solana's 32-byte public keys.

## Functions and Result Types

Functions in Rust return values explicitly using the return type syntax:

```rust
fn calculate_amount(quantity: u64, price: u64) -> u64 {
    quantity * price
}
```

The `-> u64` specifies the return type. When a function ends without a semicolon, that expression becomes the return value.

The `Result` type handles operations that can fail:

```rust
fn parse_amount(input: &str) -> Result<u64, std::num::ParseIntError> {
    input.parse::<u64>()
}
```

`Result<T, E>` is either `Ok(T)` containing success value or `Err(E)` containing error information. The question mark operator `?` propagates errors:

```rust
fn process_amount(input: &str) -> Result<u64, std::num::ParseIntError> {
    let amount = parse_amount(input)?;
    Ok(amount * 100)
}
```

Solana programs return `Result<()>` where `()` (unit type) indicates success with no return value.

## Structs and Enums

Structs group related data fields:

```rust
struct TokenAmount {
    mint: Pubkey,
    amount: u64,
}
```

Instances are created and accessed:

```rust
let token = TokenAmount {
    mint: Pubkey::default(),
    amount: 1000,
};
println!("Amount: {}", token.amount);
```

Enums represent one of several possible variants:

```rust
enum SwapDirection {
    Forward,
    Reverse,
}
```

Enums with data variants are powerful for modeling state:

```rust
enum OfferStatus {
    Open,
    Filled,
    Cancelled,
}
```

## Ownership and Borrowing

Rust's ownership system ensures memory safety without garbage collection. Each value has a single owner, and when the owner goes out of scope, the value is dropped. This prevents double-free errors and memory leaks.

When passing values to functions, ownership transfers:

```rust
fn process_offer(offer: Offer) {
    // offer is owned here
}
// offer is dropped here
```

Borrowing allows references without transferring ownership:

```rust
fn validate_offer(offer: &Offer) -> bool {
    // offer is borrowed here, original owner unchanged
    offer.amount > 0
}
```

Mutable borrowing requires explicit `&mut`:

```rust
fn update_offer(offer: &mut Offer, new_amount: u64) {
    offer.amount = new_amount;
}
```

Solana's `Context` type contains borrowed references to all accounts, leveraging Rust's borrowing system for efficient account access.

## Traits and Derive Macros

Traits define shared behavior across types:

```rust
pub trait Initialize {
    fn init(&mut self);
}
```

The `#[derive]` macro automatically implements common traits:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
struct Amount(u64);
```

Anchor uses derive macros extensively for account validation:

```rust
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32)]
    pub data_account: Account<'info, Data>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

## Practical Exercise

Create a new Anchor program and examine the generated code structure. Focus on understanding how the `#[program]` attribute defines instruction handlers, how `#[derive(Accounts)]` validates account structures, and how `Result<()>` indicates instruction success or failure.

Modify the default instruction to accept parameters and return values, observing how Rust's type system ensures correctness at compile time.

## Key Takeaways

Rust's type system and ownership model provide safety guarantees essential for financial smart contracts. While the learning curve is steeper than some languages, the compile-time error catching prevents vulnerabilities that could result in lost funds.

The patterns covered here—immutable defaults, explicit mutability, Result types for error handling, struct organization, and trait-based polymorphism—appear throughout Solana program development.

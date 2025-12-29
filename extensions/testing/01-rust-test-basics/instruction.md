# Rust Test Basics

This stage introduces Rust's built-in testing capabilities and how they apply to Solana program testing.

## The #[test] Attribute

Rust's `#[test]` attribute marks functions as tests:

```rust
#[test]
fn test_sample() {
    assert!(true);
}

#[test]
fn test_math() {
    assert_eq!(2 + 2, 4);
}
```

These tests run with `cargo test` and report pass or fail.

## Assertion Macros

Rust provides several assertion macros:

**assert!**: Fails if expression is false

```rust
assert!(condition, "optional message");
```

**assert_eq!**: Fails if two values are not equal

```rust
assert_eq!(expected, actual);
assert_eq!(expected, actual, "custom message");
```

**assert_ne!**: Fails if two values are equal

```rust
assert_ne!(value1, value2);
```

## Test Organization

Tests can be in the same file as code or in separate files:

```rust
// In lib.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_something() {
        // test code
    }
}
```

The `#[cfg(test)]` attribute only compiles test code during testing.

## Expected Panics

Use `#[should_panic]` to expect a test to panic:

```rust
#[test]
#[should_panic(expected = "InvalidAmount")]
fn test_invalid_amount_panics() {
    // This should panic with "InvalidAmount"
}
```

Use with expected substring to verify the panic message.

## Running Tests

Run all tests:

```rust
cargo test
```

Run a specific test by name:

```rust
cargo test test_name
```

Run tests matching a pattern:

```rust
cargo test offer  // runs all tests with "offer" in name
```

## Practical Exercise

Create simple unit tests for your data structures. Test that the Offer struct can be created with valid values. Test that assertions fail correctly with invalid values.

## Key Takeaways

#[test] marks test functions. assert!, assert_eq!, assert_ne! provide verification. Tests can be in-module or separate files. #[should_panic] expects panics. cargo test runs tests.

# PDA Concept

This stage introduces Program Derived Addresses (PDAs), one of Solana's most powerful features for secure account management. PDAs enable programs to control accounts without private keys, forming the foundation of secure token custody in swap programs.

## What is a Program Derived Address

A Program Derived Address is a public key derived deterministically from a program ID and additional seed inputs. Unlike regular keypairs where a private key controls the address, PDAs have no corresponding private key. Only the program that derived them can sign on their behalf.

Regular addresses are Ed25519 curve points generated from private keys. PDAs are derived differently—they exist off the curve and cannot be signed for by any private key. This property makes them ideal for accounts that should only be controlled by program logic.

When you derive a PDA, you provide seeds (byte arrays) and optionally a bump seed. The derivation function combines your program ID, seeds, and bump to generate the address. If the result happens to fall on the Ed25519 curve, you must try a different bump value until you get an off-curve address.

## Why PDAs Matter for Swap Programs

Swap programs handle user funds and must ensure only authorized operations occur. PDAs provide the security foundation for this trust.

Consider a vault holding user deposits. If the vault were controlled by a regular keypair, whoever held that keypair could steal all funds. Instead, we make the vault a PDA controlled by our swap program. Only our program can authorize transfers from the vault, and program logic enforces all authorization rules.

PDAs also provide deterministic addressing. Given the same seeds, you always derive the same address. This enables users to know in advance where their vault or offer account will exist, even before it is created. No need to generate and store random addresses.

## PDA Derivation Process

Deriving a PDA involves trying different bump values until finding one that produces an off-curve address:

```rust
use anchor_lang::prelude::*;

let (pda_address, bump) = Pubkey::find_program_address(
    &[
        b"offer",
        maker.key().as_ref(),
        &id.to_le_bytes(),
    ],
    program_id,
);
```

The `find_program_address` function handles the bump search automatically. It tries bump values starting from 255 and descending, checking if each produces an off-curve result. The first successful derivation returns the address and the bump that worked.

The seeds can include static strings, public keys, and integer representations. For our swap program, we use the literal "offer" (to distinguish from other program accounts), the maker's public key (to ensure offers are per-user), and the offer ID (to allow multiple offers per user).

## PDA Signing Authority

When a PDA needs to perform actions requiring authority (like token transfers), the program provides signing seeds:

```rust
let seeds = &[
    b"offer",
    maker.key().as_ref(),
    &offer.id.to_le_bytes(),
    &[offer.bump],
];
let signer_seeds = [&seeds[..]];

let cpi_context = CpiContext::new_with_signer(
    token_program.to_account_info(),
    cpi_accounts,
    signer_seeds,
);
```

The runtime uses these seeds to reconstruct the PDA and sign on its behalf. This is the magic that allows program-controlled accounts to authorize token transfers—the program proves it derived the PDA and provides the canonical bump.

## Comparing Regular Addresses vs PDAs

Regular addresses have corresponding private keys that control them. Anyone with the private key has full authority over the account. These are suitable for user wallets but not for program-controlled vaults.

PDA addresses have no private keys. Only the program that derived them can authorize actions. The program must provide the exact seeds and bump used during derivation. This provides strong guarantees that only the intended program can control the account.

The derivation formula is deterministic but not reversible. Knowing a PDA address and seeds does not reveal the program ID or other seed values. However, given all seeds and the program ID, anyone can verify a PDA derivation.

## Practical Exercise

Use the `Pubkey::find_program_address` function to derive PDAs with different seed combinations. Observe how changing any seed changes the resulting address. Verify that the same inputs always produce the same output.

Create a simple program that stores a message in a PDA account. Implement a function to update the message, requiring PDA signing. Test that only the program can modify the account, not external signers.

## Key Takeaways

PDAs are addresses derived from program ID and seeds without corresponding private keys. Only the deriving program can control PDAs through provided seeds. PDAs enable secure program-controlled vaults and deterministic account addressing. PDA signing via `CpiContext::new_with_signer` authorizes actions on behalf of the PDA.

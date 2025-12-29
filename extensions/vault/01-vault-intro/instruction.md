# Vault Introduction

This stage introduces the vault concept for token custody in swap programs. Vaults enable secure escrow where deposited tokens are held safely until a taker completes the swap.

## What is a Token Vault

A token vault is a token account whose authority is a Program Derived Address rather than a user keypair. When a user deposits tokens into a vault, the tokens remain in the vault until the program authorizes their release. The program acts as an impartial escrow that releases funds only when swap conditions are met.

In a traditional centralized exchange, the exchange holds user funds in its own wallets. Users must trust the exchange to maintain custody and return funds upon withdrawal. In a decentralized swap program, vaults eliminate this trust requirement. No single party controls the vault—only the program logic can move tokens, and that logic is public and auditable.

For our swap program, each offer has an associated vault holding the tokens the maker has offered. When a taker accepts the offer, the program distributes tokens to both parties and closes the vault. If the offer is never taken, the maker can close the vault to recover their tokens.

## Why Vaults are Essential

Vaults provide several critical guarantees for decentralized swapping.

**Trustless Custody**: Tokens in a vault can only be moved according to program rules. Neither the maker nor the taker can unilaterally access funds—the program enforces all conditions.

**Conditional Release**: Tokens are released only when specific conditions are met. The vault does not release tokens based on trust but on verifiable on-chain conditions.

**Auditability**: Anyone can inspect the vault's balance and the program's logic. Users can verify that the program will release tokens only under expected conditions.

**Non-custodial**: The swap program never takes ownership of user tokens. Tokens move directly from maker vault to taker wallet and vice versa.

## Vault Architecture

The vault architecture involves three parties with distinct roles.

The **Maker** creates an offer, deposits tokens into the vault, and specifies swap conditions. The maker can recover tokens by canceling the offer (if the program allows) or if the offer expires.

The **Taker** reviews offers and decides whether to accept. When accepting, the taker provides the requested tokens and receives the offered tokens from the vault.

The **Program** enforces all rules. It validates that the taker provides correct payment before authorizing vault transfer. It ensures both parties receive their tokens simultaneously. It closes the vault when the swap completes.

## How Programs Control Vaults

A vault is a token account with the offer PDA as its authority. The token account's owner field points to the offer PDA address, not to a user keypair.

```rust
#[account(
    init,
    payer = maker,
    associated_token::mint = token_mint_a,
    associated_token::authority = offer  // Offer PDA controls the vault
)]
pub vault: InterfaceAccount<'info, TokenAccount>,
```

Since the offer is a PDA with no private key, only the program can authorize transfers from the vault. The program proves ownership by providing the PDA's signing seeds during CPI.

## Vault Lifecycle

A vault goes through a predictable lifecycle in the swap program.

**Creation** occurs during make_offer when the maker deposits tokens. The program creates a token account owned by the offer PDA and transfers tokens into it.

**Holding** is the period between creation and swap completion. The vault holds tokens while the offer remains open. Anyone can view the balance but cannot access the funds.

**Release** happens during take_offer when a taker accepts the offer. The program transfers tokens from the vault to the taker while simultaneously receiving payment from the taker.

**Closing** concludes the vault's lifecycle. After all tokens are distributed, the vault account is closed and its remaining lamports are returned to the specified destination.

## Practical Exercise

Examine the template's vault account definition. Identify which account serves as the vault authority. Understand how the vault relates to the offer account.

Draw a diagram showing the relationship between maker, offer, vault, and taker. Trace the token flow during make_offer and take_offer.

## Key Takeaways

Vaults are PDA-controlled token accounts enabling trustless escrow. The offer PDA serves as vault authority, allowing only program-controlled transfers. Vaults hold deposited tokens until program logic authorizes release. The vault lifecycle spans creation, holding, release, and closing.

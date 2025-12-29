# Solana Swap Program Development

Learn to build a peer-to-peer token swap program on Solana blockchain from scratch. This comprehensive course takes you from environment setup to deploying a secure, production-ready decentralized exchange smart contract.

## What You Will Build

Throughout this course, you will build a complete peer-to-peer swap program that enables users to exchange tokens without requiring a centralized intermediary. The program implements a classic escrow pattern where:

- **Makers** create swap offers by specifying which tokens they want to offer and what tokens they want to receive
- **Takers** accept these offers by providing the requested tokens
- **The Program** acts as a trusted escrow, holding offered tokens safely until a taker completes the swap

By the end of this course, you will have implemented a fully functional swap program with proper security measures, comprehensive testing, and deployment ready for production use.

## Course Structure

This course is divided into two main paths: foundational stages and advanced extensions. Each component focuses on a single concept, explained thoroughly before moving to the next topic.

### Foundational Stages (7 Stages)

The stages provide essential background knowledge required before diving into the swap program implementation. Complete these stages in order to build a solid understanding of Solana development fundamentals.

**Stage 1: Environment Setup** - Install and configure Rust, Solana CLI, and Anchor framework. Set up your development environment with proper tooling for building and deploying Solana programs.

**Stage 2: Rust Basics for Solana** - Learn Rust syntax and concepts essential for blockchain development. Understand structs, enums, Result types, and how Anchor macros simplify Solana program development.

**Stage 3: Solana Account Model** - Comprehend Solana's unique account model, the difference between system accounts and program accounts, and how account lifecycle management works on-chain.

**Stage 4: Anchor First Try** - Create your first Anchor project, deploy it to devnet, and interact with it using basic transactions to understand the development workflow.

**Stage 5: SPL Token Basics** - Explore SPL tokens (Solana's token standard), understand mints and token accounts, and learn about Associated Token Accounts (ATA) for user token holdings.

**Stage 6: CPI Transfer** - Master Cross-Program Invocation (CPI) to interact with the Token Program. Learn how to transfer tokens programmatically from within your own program.

**Stage 7: Token Transfer Transaction** - Build complete transactions that perform token transfers. Understand instruction ordering, account requirements, and transaction construction.

### Advanced Extensions (10 Topics × 4 Steps Each = 40 Extensions)

After completing the foundational stages, proceed through the extensions to build the complete swap program step by step. Each topic contains four sub-steps: concept explanation, implementation details, practice exercise, and comprehensive review.

**PDA (Program Derived Addresses)** - Learn how PDAs work, how to derive them using seeds, understand bump seeds for canonical derivation, and implement PDA-based account addressing for your swap program.

**Vault Management** - Understand token vault creation as PDA-owned token accounts, explore custody patterns where the program controls tokens on behalf of users, and learn critical security considerations for vault implementation.

**Offer Data Structure** - Design the Offer account that stores swap proposal details including maker address, offered token mint, wanted token amount, and exchange rate. Learn space calculation and initialization patterns.

**Make Offer Instruction** - Implement the complete make_offer instruction including token deposit to vault, offer account creation, PDA derivation with proper seeds, and validation of all inputs and accounts.

**Take Offer Instruction** - Build the take_offer instruction to execute swaps, receive wanted tokens from taker, transfer offered tokens from vault to taker, and properly close all temporary accounts.

**Security Best Practices** - Study common smart contract vulnerabilities including reentrancy attacks, learn validation techniques, implement access control checks, and understand defensive programming patterns for production code.

**Cross-Program Invocation Deep Dive** - Master CPI for complex token operations including transfer_checked for precision transfers, understand CPI signer authorization for vault operations, and learn error propagation through CPI calls.

**Error Handling** - Define custom error codes specific to swap operations, write descriptive error messages for debugging, implement proper error handling patterns, and create user-friendly error feedback.

**Rust Testing** - Write comprehensive tests using Rust's native test attributes and Anchor's program testing framework. Learn test setup patterns, assertion strategies, and how to test both happy paths and error cases.

**Deployment** - Deploy your program to local validator for development testing, deploy to devnet for realistic testing with live tokens, and understand considerations for mainnet deployment including upgrade authority and verification.

## Prerequisites

This course assumes basic programming experience. Prior experience with blockchain development is helpful but not required. You should be comfortable with:

- Command line navigation and basic shell commands
- Text editor or IDE usage
- Basic programming concepts (variables, functions, control flow)
- Some exposure to object-oriented programming

No prior Rust or Solana experience is expected. The course builds all necessary knowledge progressively.

## Learning Approach

Each component in this course focuses on exactly one concept, explained thoroughly before introducing the next topic. This approach allows you to:

- Master each concept before moving forward
- Build mental models progressively
- Understand not just "how" but "why" things work
- Apply concepts immediately through practice exercises

Complete the foundational stages first, then work through the extensions sequentially. Each extension builds upon previous knowledge, creating a complete understanding of swap program development.

## Project Overview

The swap program you will build enables peer-to-peer token exchanges. Consider this scenario: Alice has 1000 USDC and wants 1 SOL. Bob has 1 SOL and wants 1000 USDC. Through your swap program:

1. Alice creates an offer specifying she offers 1000 USDC and wants 1 SOL
2. The program creates a vault account to hold Alice's USDC
3. Alice deposits 1000 USDC into the vault
4. Bob sees the offer and decides to accept it
5. Bob provides 1 SOL worth of tokens to Alice
6. The program transfers Alice's USDC from vault to Bob
7. The program transfers Bob's tokens to Alice
8. All temporary accounts are closed

This escrow pattern eliminates counterparty risk - neither party can cheat because the program holds the deposited tokens securely until both sides have fulfilled their obligations.

## Getting Started

Begin with Stage 1: Environment Setup to prepare your development environment. Follow the stages in order, completing each instruction and exercise before proceeding. The foundational stages typically take 2-3 hours for those new to Solana, while the extensions require approximately 10-15 hours to complete fully.

Each stage and extension includes:
- Detailed instruction explaining the concept
- Implementation guidance with code examples
- Practice exercises to apply your knowledge
- Expected outcomes and validation criteria

## Course Philosophy

This course follows a "one concept at a time" philosophy. Rather than overwhelming you with the complete swap program upfront, we break down every component into digestible pieces. You will understand each building block thoroughly before seeing how it fits into the larger system.

This approach mirrors how professional Solana developers think about program architecture. By the end, you will not only have a working swap program but also a deep understanding of why each component exists and how to extend or modify it for your own projects.

## Next Steps

Start with the first stage: [Environment Setup](stages/01-be1-env-setup/instruction.md)

If you already have your environment configured, you may skip to the foundational stage that matches your current knowledge level.

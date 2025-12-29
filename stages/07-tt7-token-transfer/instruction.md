# Token Transfer Transaction

This stage guides you through building complete transactions that perform token transfers, teaching the practical aspects of transaction construction and account ordering.

## Transaction Fundamentals

A Solana transaction is a collection of instructions that execute atomically—either all instructions succeed, or the entire transaction reverts. Each instruction specifies a program to call, the accounts involved, and instruction data.

When constructing transactions, several rules govern account ordering. The fee payer must appear first if paying for transaction costs. Writable accounts must appear before read-only accounts that depend on them. The order should reflect data dependencies to maximize parallel execution.

Transaction size is limited to 1232 bytes, limiting how many instructions and accounts a single transaction can contain. Complex operations requiring many accounts may need to span multiple transactions.

## Building a Transfer Transaction

Consider a complete token transfer from Alice to Bob. The transaction requires multiple instructions and accounts working together.

First, ensure Bob has a token account for the receiving mint. If not, create one via the Associated Token Program:

```typescript
const bobTokenAccount = await getAssociatedTokenAddress(
    tokenMint,
    bob.publicKey
);

const createAccountIx = createAssociatedTokenAccountInstruction(
    payer.publicKey,
    bobTokenAccount,
    bob.publicKey,
    tokenMint
);
```

The create account instruction uses the Associated Token Program to derive and initialize Bob's token account in a single transaction.

Second, add the transfer instruction calling the Token Program:

```typescript
const transferIx = createTransferCheckedInstruction(
    aliceTokenAccount,
    tokenMint,
    bobTokenAccount,
    alice.publicKey,
    amount,
    decimals
);
```

The transfer instruction specifies source account, mint (for verification), destination account, authority, and amount with decimal precision.

Finally, combine instructions into a transaction and send:

```typescript
const transaction = new Transaction()
    .add(createAccountIx)
    .add(transferIx);

await sendAndConfirmTransaction(
    connection,
    transaction,
    [payer, aliceKeypair]  // Signers: fee payer + authority
);
```

## Account Ordering Considerations

Proper account ordering affects both transaction size and execution performance. Solana's runtime uses account ordering to parallelize non-conflicting instructions.

Place the fee payer first:

```typescript
const transaction = new Transaction()
    .add( /* fee payer is implicit */ )
```

For multiple instructions, consider dependencies:

```typescript
const transaction = new Transaction()
    .add(createBobAccountIx)    // Create account first
    .add(transferFromAliceIx)   // Then transfer to it
    .add(transferFromCarolIx)   // Independent transfer
```

Read-only accounts that do not depend on earlier instructions can appear anywhere. Writable accounts that will be modified must appear before they are used.

## Multiple Signers

Transactions requiring multiple signatures must include all signers when sending:

```typescript
const transaction = new Transaction()
    .add(transferFromAliceIx)
    .add(transferFromBobIx);

await sendAndConfirmTransaction(
    connection,
    transaction,
    [aliceKeypair, bobKeypair]  // Both must sign
);
```

The runtime verifies each signer corresponds to a required account and that the transaction includes their signature. Without all required signatures, the transaction fails.

## Handling Large Transfers

For transfers exceeding single-transaction limits or requiring additional security, consider splitting across multiple transactions or adding timelock mechanisms.

Large atomic swaps often use a two-phase approach: the first transaction prepares the swap (deposits to escrow), and a second transaction completes it (withdraws from escrow). This pattern enables cancellation if one party fails to complete their part.

## Understanding Compute Units

Each instruction consumes compute units (CU) based on its complexity. The Token Program's transfer instruction typically consumes around 10,000 CU. Complex programs may require more.

The default compute unit limit (1.4 million CU) is sufficient for most transactions. If your transaction fails with compute unit exceeded errors, you can request additional units:

```typescript
const transaction = new Transaction()
    .add(instruction)
    .setComputeUnitLimit(300000);  // Increase limit
```

Monitor compute unit usage during development to identify inefficient code paths.

## Practical Exercise

Build a transaction that creates a token account and performs a transfer in a single atomic transaction. Test with insufficient balances to verify proper error handling. Experiment with multiple transfers in a single transaction and observe parallel execution benefits.

Add compute unit logging to understand consumption patterns for different transaction structures.

## Key Takeaways

Transactions bundle instructions atomically with specific account ordering rules. Token transfers often require multiple instructions (account creation + transfer). Multiple signers must all be included when sending transactions. Compute unit limits may require tuning for complex operations. Understanding transaction construction is essential for building production-grade Solana applications.

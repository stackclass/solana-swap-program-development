In this final stage, you'll create comprehensive tests for your swap program and deploy it to Solana devnet. Testing is crucial for ensuring your program works correctly before deploying to a live network.

## Understanding Testing and Deployment

Testing and deployment are the final steps in the smart contract development lifecycle:

- **Testing**: Validates that your program works as expected under various conditions
- **Deployment**: Publishes your program to a live network where users can interact with it
- **Devnet**: A test network that mimics mainnet but uses test tokens (no real value at risk)

## Prerequisite Reading

To understand this stage, review:

- **Anchor Testing**: Read the [Anchor Testing Guide](https://www.anchor-lang.com/docs/testing) to learn how to write and run tests.
- **Solana Devnet**: Learn about devnet in the [Solana Networks Documentation](https://solana.com/docs/core/clusters#devnet).
- **Anchor Deploy**: Review deployment in the [Anchor Deployment Guide](https://www.anchor-lang.com/docs/deployment).
- **TypeScript Testing**: Learn about testing with TypeScript in the [Anchor TypeScript Guide](https://www.anchor-lang.com/docs/testing#typescript).

## Create Comprehensive Tests

Create or update `tests/swap.ts` with comprehensive tests:

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Swap } from "../target/types/swap";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  createAccount,
  mintTo,
} from "@solana/spl-token";
import { assert } from "chai";

describe("swap", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Swap as Program<Swap>;

  // Token mints
  let tokenMintA: anchor.web3.PublicKey;
  let tokenMintB: anchor.web3.PublicKey;

  // Token accounts
  let makerTokenAccountA: anchor.web3.PublicKey;

  before(async () => {
    // Create token mints
    tokenMintA = await createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      6
    );

    tokenMintB = await createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      6
    );

    // Create maker's token account for Token A
    makerTokenAccountA = await createAccount(
      provider.connection,
      provider.wallet.payer,
      tokenMintA,
      provider.wallet.publicKey
    );

    // Mint tokens to maker
    await mintTo(
      provider.connection,
      provider.wallet.payer,
      tokenMintA,
      makerTokenAccountA,
      provider.wallet.publicKey,
      1_000_000_000
    );
  });

  it("Makes an offer", async () => {
    const offerId = new anchor.BN(1);
    const tokenAOfferedAmount = new anchor.BN(1_000_000);
    const tokenBWantedAmount = new anchor.BN(1_000_000);

    // Derive offer PDA
    const [offerPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("offer"),
        provider.wallet.publicKey.toBuffer(),
        offerId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    // Make offer
    await program.methods
      .makeOffer(offerId, tokenAOfferedAmount, tokenBWantedAmount)
      .accounts({
        maker: provider.wallet.publicKey,
        tokenMintA: tokenMintA,
        tokenMintB: tokenMintB,
        makerTokenAccountA: makerTokenAccountA,
        offer: offerPDA,
      })
      .rpc();

    // Fetch and verify offer
    const offer = await program.account.offer.fetch(offerPDA);
    assert.equal(offer.id.toString(), offerId.toString());
    assert.equal(offer.maker.toString(), provider.wallet.publicKey.toString());
    assert.equal(
      offer.tokenMintA.toString(),
      tokenMintA.toString()
    );
    assert.equal(
      offer.tokenMintB.toString(),
      tokenMintB.toString()
    );
    assert.equal(
      offer.tokenBWantedAmount.toString(),
      tokenBWantedAmount.toString()
    );
  });

  it("Takes an offer", async () => {
    const offerId = new anchor.BN(1);

    // Create taker's token accounts
    const takerTokenAccountA = await createAccount(
      provider.connection,
      provider.wallet.payer,
      tokenMintA,
      provider.wallet.publicKey
    );

    const takerTokenAccountB = await createAccount(
      provider.connection,
      provider.wallet.payer,
      tokenMintB,
      provider.wallet.publicKey
    );

    // Mint Token B to taker
    await mintTo(
      provider.connection,
      provider.wallet.payer,
      tokenMintB,
      takerTokenAccountB,
      provider.wallet.publicKey,
      2_000_000
    );

    // Derive accounts
    const [offerPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("offer"),
        provider.wallet.publicKey.toBuffer(),
        offerId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    const [vaultPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("offer"),
        provider.wallet.publicKey.toBuffer(),
        offerId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    // Take offer
    await program.methods
      .takeOffer()
      .accounts({
        taker: provider.wallet.publicKey,
        maker: provider.wallet.publicKey,
        tokenMintA: tokenMintA,
        tokenMintB: tokenMintB,
        takerTokenAccountA: takerTokenAccountA,
        takerTokenAccountB: takerTokenAccountB,
        makerTokenAccountB: takerTokenAccountB,
        offer: offerPDA,
        vault: vaultPDA,
      })
      .rpc();

    // Verify offer was closed
    try {
      await program.account.offer.fetch(offerPDA);
      assert.fail("Offer should have been closed");
    } catch (err) {
      assert.include(err.toString(), "Account does not exist");
    }
  });

  it("Fails with insufficient balance", async () => {
    const offerId = new anchor.BN(2);
    const tokenAOfferedAmount = new anchor.BN(10_000_000_000); // More than we have
    const tokenBWantedAmount = new anchor.BN(1_000_000);

    const [offerPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("offer"),
        provider.wallet.publicKey.toBuffer(),
        offerId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    try {
      await program.methods
        .makeOffer(offerId, tokenAOfferedAmount, tokenBWantedAmount)
        .accounts({
          maker: provider.wallet.publicKey,
          tokenMintA: tokenMintA,
          tokenMintB: tokenMintB,
          makerTokenAccountA: makerTokenAccountA,
          offer: offerPDA,
        })
        .rpc();
      assert.fail("Should have thrown an error");
    } catch (err) {
      assert.include(err.toString(), "InsufficientBalance");
    }
  });
});
```

## Understanding the Test Structure

### Test Setup

```typescript
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
```

- Creates a provider using the local environment
- Sets the provider for the test suite
- Uses the local validator for testing

### Token Setup

```typescript
tokenMintA = await createMint(...);
makerTokenAccountA = await createAccount(...);
await mintTo(...);
```

- Creates two token mints (Token A and Token B)
- Creates token accounts for the maker
- Mints tokens to the maker's account

### Test 1: Make Offer

```typescript
it("Makes an offer", async () => {
  // Derive PDA
  const [offerPDA] = anchor.web3.PublicKey.findProgramAddressSync(...);

  // Call make_offer
  await program.methods.makeOffer(...).accounts({...}).rpc();

  // Verify offer was created
  const offer = await program.account.offer.fetch(offerPDA);
  assert.equal(offer.id.toString(), offerId.toString());
  // ... more assertions
});
```

**What it tests**:
- Offer creation works correctly
- PDA derivation is correct
- Offer data is stored properly

### Test 2: Take Offer

```typescript
it("Takes an offer", async () => {
  // Create taker's token accounts
  // Mint tokens to taker

  // Call take_offer
  await program.methods.takeOffer().accounts({...}).rpc();

  // Verify offer was closed
  try {
    await program.account.offer.fetch(offerPDA);
    assert.fail("Offer should have been closed");
  } catch (err) {
    assert.include(err.toString(), "Account does not exist");
  }
});
```

**What it tests**:
- Offer acceptance works correctly
- Tokens are transferred properly
- Offer account is closed after swap

### Test 3: Error Handling

```typescript
it("Fails with insufficient balance", async () => {
  try {
    await program.methods.makeOffer(...).rpc();
    assert.fail("Should have thrown an error");
  } catch (err) {
    assert.include(err.toString(), "InsufficientBalance");
  }
});
```

**What it tests**:
- Error handling works correctly
- Appropriate errors are returned
- Invalid transactions are rejected

## Deploy to Devnet

### 1. Configure for Devnet

Update your `Anchor.toml` to use devnet:

```toml
[provider]
cluster = "devnet"
wallet = "~/.config/solana/id.json"

[programs.devnet]
swap = "YOUR_PROGRAM_ID_HERE"
```

**What this does**:
- Sets the cluster to devnet (test network)
- Specifies the wallet to use for deployment
- Defines the program ID for devnet deployment

### 2. Switch Solana CLI to Devnet

```bash
solana config set --url devnet
```

**What this does**:
- Configures the Solana CLI to use devnet
- All CLI commands will now interact with devnet

### 3. Fund Your Wallet

```bash
solana airdrop 2
```

**What this does**:
- Requests 2 SOL from the devnet faucet
- Provides SOL for paying transaction fees and rent
- Devnet SOL has no real value

### 4. Deploy Your Program

```bash
anchor deploy
```

**What this does**:
- Compiles your program
- Uploads the program bytecode to devnet
- Returns the program ID

**Expected output**:
```
Deploying cluster: https://api.devnet.solana.com
Upgrade authority: YourPublicKey
Deploying program "swap"...
Program path: /path/to/swap.so...
Program Id: YOUR_PROGRAM_ID_HERE
```

### 5. Verify Deployment

```bash
solana program show YOUR_PROGRAM_ID_HERE
```

**What this does**:
- Displays information about your deployed program
- Confirms the program is live on devnet

## Running Tests

### Run Local Tests

```bash
anchor test
```

**What this does**:
- Starts a local validator
- Runs all tests against the local validator
- Stops the validator after tests complete

### Run Devnet Tests

```bash
anchor test --skip-local-validator
```

**What this does**:
- Runs tests against devnet instead of local validator
- Useful for testing against a live network
- Requires devnet SOL for transaction fees

## Test Cases

| Test | Expected Result | Purpose |
|------|-----------------|---------|
| Test compilation | TypeScript compiles successfully | Ensures test syntax is correct |
| Make offer test | Offer created and verified | Validates offer creation |
| Take offer test | Offer accepted and closed | Validates offer acceptance |
| Error handling test | Appropriate error returned | Validates error handling |
| Deployment | Program deploys to devnet | Confirms successful deployment |
| Program ID | Correct program ID in Anchor.toml | Ensures proper configuration |

## Best Practices

### 1. Test All Code Paths

- Test success cases
- Test error cases
- Test edge cases
- Test boundary conditions

### 2. Use Descriptive Test Names

```typescript
// Good
it("Makes an offer with valid parameters", async () => {...});

// Bad
it("test1", async () => {...});
```

### 3. Clean Up After Tests

```typescript
after(async () => {
  // Clean up test accounts
});
```

### 4. Use Assertions Effectively

```typescript
// Good
assert.equal(offer.id.toString(), offerId.toString());
assert.include(err.toString(), "InsufficientBalance");

// Bad
console.log("Offer created"); // Not a real test
```

### 5. Test on Devnet Before Mainnet

- Always test on devnet first
- Verify all functionality works
- Check gas costs
- Ensure no critical bugs

## Congratulations!

You've now completed the full swap program development lifecycle:

1. ✅ Environment setup
2. ✅ Account structure design
3. ✅ Context implementation
4. ✅ PDA initialization
5. ✅ Token transfer functionality
6. ✅ Make offer logic
7. ✅ Take offer context
8. ✅ Error handling
9. ✅ Take offer logic
10. ✅ Testing and deployment

Your swap program is now ready for real-world use on the Solana blockchain!

## Next Steps

- Deploy to mainnet (when ready)
- Build a frontend interface
- Add more features (cancel offer, update offer, etc.)
- Audit your code for security
- Document your API for users
# Devnet Deployment

This stage teaches how to deploy your swap program to Solana devnet for realistic testing with live tokens.

## Configure for Devnet

Update Anchor.toml to target devnet:

```toml
[provider]
cluster = "devnet"
wallet = "~/.config/solana/id.json"

[programs.devnet]
swap = "YourProgramIdHere"
```

Switch your Solana config:

```bash
solana config set --url devnet
```

## Generate a Program Keypair

If you don't have a program keypair, generate one:

```bash
solana-keygen new -o target/deploy/swap-keypair.json
```

This creates a keypair file. The public key from this file will be your program ID.

Update your lib.rs:

```rust
declare_id!("YourProgramIdFromKeypair");
```

## Deploy to Devnet

Build and deploy:

```bash
anchor build
anchor deploy
```

The deploy command:
- Compiles your program (if not already built)
- Uploads bytecode to devnet
- Sets the program as executable
- Returns the program ID

## Airdrop on Devnet

Get devnet SOL for deployment:

```bash
solana airdrop 2
```

Devnet has rate limits on airdrops. If you need more SOL, request from a faucet or wait between requests.

## Verify Deployment

Check your program is deployed:

```bash
solana account YourProgramId
```

You should see the account with executable=true and the program data.

## Update Client for Devnet

Update client-side code to use devnet:

```typescript
const connection = new Connection(
    "https://api.devnet.solana.com",
    "confirmed"
);
```

Or use the cluster specification:

```typescript
const provider = anchor.AnchorProvider.env();
anchor.setProvider(anchor.AnchorProvider.devnet());
```

## Common Devnet Issues

**Insufficient funds**: Airdrop more SOL. Deployment costs about 3-5 SOL.

**Network errors**: Devnet may be slow. Retry deployment.

**Program ID mismatch**: Ensure declared ID matches deployed program.

**Upgrade authority**: By default, only the deployer can upgrade. Configure upgrade authority if needed.

## Practical Exercise

Configure Anchor.toml for devnet. Generate a program keypair. Deploy your swap program. Verify the deployment. Run integration tests against devnet.

## Key Takeaways

Configure Anchor.toml for devnet. Generate a program keypair. Use anchor deploy to upload. Airdrop devnet SOL for costs. Verify deployment with solana account.

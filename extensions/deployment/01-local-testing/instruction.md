# Local Testing

This stage teaches how to test your swap program on a local Solana validator before deploying to devnet or mainnet.

## Local Validator

Anchor's `anchor test` command automatically starts a local validator:

```bash
anchor test
```

The local validator:
- Runs in memory (stops when command exits)
- Uses a fresh ledger each time
- Provides fast iteration for development
- Has unlimited SOL for testing

## Manual Validator Control

For more control, start the validator manually:

```bash
solana-test-validator --reset
```

This starts a validator and resets the ledger. Keep this terminal open.

In another terminal, run your tests:

```bash
anchor test --skip-local-validator
```

The `--skip-local-validator` flag tells Anchor to use the already-running validator.

## Configuring Anchor.toml

Configure your local validator settings:

```toml
[provider]
cluster = "Localnet"
wallet = "~/.config/solana/id.json"

[programs.localnet]
swap = "YourProgramIdHere"

[test]
startup_wait = 10000  # Wait 10 seconds for validator startup
```

## Using the Correct Cluster

Anchor.toml must specify Localnet for local testing:

```toml
[provider]
cluster = "Localnet"  # Not "devnet" or "mainnet"
```

If you accidentally use devnet, tests will attempt to use real tokens and may fail.

## Airdrop on Localnet

Get SOL for testing:

```bash
solana airdrop 10
```

Localnet airdrops are unlimited and instant. Use them freely for testing.

## Common Local Testing Issues

**Validator not starting**: Check port 8899 is not in use. Kill any existing validators.

**Timeout errors**: Increase startup_wait in Anchor.toml.

**Account not found**: Ensure your program is deployed to the local validator.

**Type mismatch**: Rebuild with `anchor build` after code changes.

## Practical Exercise

Start a local validator. Deploy your program. Run tests. Verify all tests pass. Experiment with different test scenarios.

## Key Takeaways

anchor test starts a local validator automatically. Manual validator gives more control. Localnet has unlimited SOL. Configure Anchor.toml for local testing. Rebuild after code changes.

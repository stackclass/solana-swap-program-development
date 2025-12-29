# Deployment Practice

This stage provides hands-on exercises to practice deploying and testing your swap program on different networks.

## Exercise 1: Localnet Deployment

Deploy to local validator:

```bash
# 1. Start local validator
solana-test-validator --reset

# 2. Configure Anchor for localnet
anchor config set provider.cluster localnet

# 3. Build and deploy
anchor build
anchor deploy

# 4. Verify deployment
solana account YourProgramId
```

Expected: Program deployed with executable=true.

## Exercise 2: Write Local Tests

Write comprehensive tests for local testing:

```rust
#[program_test]
async fn test_local_swap_flow(
    mut test_ctx: ProgramTestContext,
) {
    // Setup test environment
    let (mut test_ctx, maker, taker) = setup_local_environment(&mut test_ctx).await;
    
    // Test complete swap flow
    test_complete_swap(&mut test_ctx, &maker, &taker).await;
    
    // Verify results
    assert!(verify_swap_results(&mut test_ctx, &maker, &taker).await);
}
```

Run tests:

```bash
anchor test --skip-local-validator
```

## Exercise 3: Devnet Deployment

Deploy to devnet:

```bash
# 1. Configure for devnet
anchor config set provider.cluster devnet

# 2. Airdrop for deployment
solana airdrop 5

# 3. Build and deploy
anchor build
anchor deploy

# 4. Update declared ID in lib.rs
# 5. Deploy again with updated ID
```

## Exercise 4: Devnet Integration Test

Write a TypeScript test for devnet:

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Connection, PublicKey } from "@solana/web3.js";

describe("swap on devnet", async () => {
    const connection = new Connection("https://api.devnet.solana.com");
    const provider = anchor.AnchorProvider.devnet();
    anchor.setProvider(provider);
    
    const program = anchor.workspace.Swap as Program<Swap>;
    const payer = provider.wallet.payer;
    
    it("Complete swap on devnet", async () => {
        // Create token mints
        const [mintA, mintB] = await createTestMints(provider);
        
        // Create user accounts
        const [maker, taker] = await createTestUsers(provider);
        
        // Airdrop tokens to users
        await airdropTokens(provider, mintA, maker, 1000000);
        await airdropTokens(provider, mintB, taker, 1000000);
        
        // Test make_offer
        const offerId = new anchor.BN(1);
        await program.methods
            .makeOffer(offerId, new anchor.BN(100000), new anchor.BN(50000))
            .accounts({
                maker: maker.publicKey,
                tokenMintA: mintA,
                tokenMintB: mintB,
                makerTokenAccountA: /* ATA for maker */,
                offer: /* derived offer address */,
                vault: /* derived vault address */,
            })
            .signers([maker])
            .rpc();
        
        // Test take_offer
        await program.methods
            .takeOffer()
            .accounts({
                taker: taker.publicKey,
                maker: maker.publicKey,
                tokenMintA: mintA,
                tokenMintB: mintB,
                takerTokenAccountA: /* ATA for taker */,
                takerTokenAccountB: /* ATA for taker */,
                makerTokenAccountB: /* ATA for maker */,
                offer: /* offer address */,
                vault: /* vault address */,
            })
            .signers([taker])
            .rpc();
        
        // Verify results
        console.log("Swap completed successfully!");
    });
});
```

## Exercise 5: Deployment Checklist

Create and complete a deployment checklist:

```markdown
# Mainnet Deployment Checklist

## Pre-Deployment
- [ ] Code review completed
- [ ] Security audit passed
- [ ] All tests passing on localnet
- [ ] All tests passing on devnet
- [ ] Documentation complete
- [ ] Monitoring configured

## Deployment
- [ ] Generate program keypair
- [ ] Update declared_id in lib.rs
- [ ] Build program
- [ ] Airdrop SOL for deployment
- [ ] Deploy to mainnet
- [ ] Verify deployment

## Post-Deployment
- [ ] Verify all accounts
- [ ] Run integration tests
- [ ] Monitor for errors
- [ ] Communicate with users
```

## Verification Criteria

Your implementation is complete when:

1. Program deploys successfully to localnet
2. Tests pass on local validator
3. Program deploys successfully to devnet
4. Integration tests pass on devnet
5. Deployment checklist is complete

## Common Mistakes to Avoid

Using wrong cluster configuration. Double-check Anchor.toml.

Forgetting to update declared_id. Program addresses must match.

Skipping security review. Never skip for mainnet.

Not testing on devnet first. Always test thoroughly before mainnet.

## Next Steps

With deployment complete, you have finished the Solana Swap Program Development course. Review the complete implementation and consider adding features or improvements.

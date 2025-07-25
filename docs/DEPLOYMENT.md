# Presale Smart Contract Deployment Guide

This guide will walk you through deploying and configuring the Solana token presale smart contract.

## Prerequisites

1. **Install Solana CLI**
   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
   ```

2. **Install Anchor Framework**
   ```bash
   npm install -g @coral-xyz/anchor-cli
   ```

3. **Install Node.js and Yarn**
   ```bash
   # Install Node.js from https://nodejs.org/
   npm install -g yarn
   ```

4. **Install Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

## Quick Setup

### 1. Clone and Install Dependencies

```bash
git clone https://github.com/topsecretagent007/token-presale-smart-contract
cd token-presale-smart-contract
yarn install
```

### 2. Generate Admin Wallet

```bash
solana-keygen new -o admin.json
```

### 3. Run Setup Script

```bash
yarn setup
```

This will:
- Check your SOL balance and request airdrop if needed
- Build the program
- Initialize the global state
- Display deployment information

## Manual Deployment

### 1. Build the Program

```bash
anchor build
```

### 2. Get Program Address

```bash
solana-keygen pubkey ./target/deploy/presale-keypair.json
```

### 3. Update Program ID

Update the program ID in these files:

**`programs/presale/src/lib.rs`:**
```rust
declare_id!("YOUR_PROGRAM_ID_HERE");
```

**`Anchor.toml`:**
```toml
[programs.localnet]
presale = "YOUR_PROGRAM_ID_HERE"
```

### 4. Deploy to Devnet

```bash
anchor deploy --provider.cluster devnet
```

### 5. Initialize Global State

```bash
anchor test --provider.cluster devnet
```

## Configuration

### Set Vault Address

```bash
yarn script set-vault -v <VAULT_ADDRESS>
```

### Start Presale

```bash
yarn script start-presale -t <TOKEN_ADDRESS>
```

### Set Stage

```bash
yarn script set-stage -s <STAGE_NUMBER>
```

## Testing

### Run Tests

```bash
anchor test
```

### Test on Devnet

```bash
anchor test --provider.cluster devnet
```

## Network Configuration

### Devnet (Recommended for Testing)

```toml
[provider]
cluster = "devnet"
wallet = "admin.json"
```

### Testnet

```toml
[provider]
cluster = "testnet"
wallet = "admin.json"
```

### Mainnet

```toml
[provider]
cluster = "mainnet-beta"
wallet = "admin.json"
```

## Troubleshooting

### Common Issues

1. **Insufficient SOL Balance**
   ```bash
   solana airdrop 2 <YOUR_WALLET_ADDRESS> --url devnet
   ```

2. **Program ID Mismatch**
   - Ensure the program ID in `lib.rs` and `Anchor.toml` match
   - Rebuild after changing program ID: `anchor build`

3. **Account Already Initialized**
   - Delete the account and redeploy
   - Or use a different program ID

4. **RPC Connection Issues**
   - Try different RPC endpoints
   - Check your internet connection

### Debug Commands

```bash
# Check SOL balance
solana balance <WALLET_ADDRESS> --url devnet

# Check program account
solana account <PROGRAM_ID> --url devnet

# View transaction
solana confirm <TRANSACTION_SIGNATURE> --url devnet
```

## Security Considerations

1. **Admin Wallet Security**
   - Keep your admin wallet secure
   - Use a hardware wallet for mainnet
   - Never share private keys

2. **Vault Address**
   - Use a multi-signature wallet for vault
   - Verify vault address before setting

3. **Token Verification**
   - Verify token address before starting presale
   - Ensure sufficient token supply

## Support

For questions or issues:

- **Telegram**: [@topsecretagent_007](https://t.me/topsecretagent_007)
- **GitHub**: [Create an issue](https://github.com/topsecretagent007/token-presale-smart-contract/issues)
- **Test Project**: [https://wild-go-presale-pumpfun-frontend.vercel.app/](https://wild-go-presale-pumpfun-frontend.vercel.app/)

## License

This project is open source and available under the MIT License. 
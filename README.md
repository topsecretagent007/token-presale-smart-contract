# Solana Token Presale Smart Contract

A complete Solana token presale smart contract built with Anchor framework. This is a full-working project that provides a robust foundation for token presales on Solana blockchain.

## Features

- **Multi-stage presale** with configurable pricing
- **Dual payment support** - SOL and stable coins (USDC/USDT)
- **Admin controls** for managing presale stages and settings
- **User state tracking** for individual purchase history
- **Pyth price feed integration** for real-time SOL/USD pricing
- **Comprehensive testing suite**
- **CLI tools** for easy deployment and management

## Contact Information

- **Telegram**: [@topsecretagent_007](https://t.me/topsecretagent_007)
- **GitHub**: [https://github.com/topsecretagent007/token-presale-smart-contract](https://github.com/topsecretagent007/token-presale-smart-contract)
- **Test Project**: [https://wild-go-presale-pumpfun-frontend.vercel.app/](https://wild-go-presale-pumpfun-frontend.vercel.app/)

## Prerequisites

### Install Dependencies

- Install `node` and `yarn`
- Install `rust`, `solana` and `anchor`

    https://www.anchor-lang.com/docs/installation

## How to Deploy

### 1. Clone and Setup

```bash
git clone https://github.com/topsecretagent007/token-presale-smart-contract
cd token-presale-smart-contract
yarn install
```

### 2. Build the Program

```bash
anchor build
```

### 3. Get Program Address

```bash
solana-keygen pubkey ./target/deploy/presale-keypair.json
```

This will give you the program pubkey (e.g., `BE4G...5qhv`)

### 4. Update Program Address

Update the program address in these files:

**In `programs/presale/src/lib.rs`:**
```rust
declare_id!("YOUR_PROGRAM_ID_HERE");
```

**In `Anchor.toml`:**
```toml
[programs.localnet]
presale = "YOUR_PROGRAM_ID_HERE"
```

### 5. Configure Provider Settings

Update `Anchor.toml`:
```toml
[provider]
cluster = "localnet"  # or "devnet", "testnet", "mainnet-beta"
wallet = "./admin.json"
```

### 6. Deploy

```bash
anchor build
anchor deploy
```

## Testing

### Run Tests

```bash
anchor test
```

### Test on Devnet

1. **Initialize project:**
   ```bash
   yarn script init -t <TOKEN_ADDRESS>
   ```

2. **Set DAO wallet address:**
   ```bash
   yarn script set-vault -v <DAO_WALLET_ADDRESS>
   ```

3. **Deposit tokens to the program:**
   ```bash
   yarn script deposit-token -t <TOKEN_ADDRESS> -a <DEPOSIT_AMOUNT>
   ```

4. **Start presale:**
   ```bash
   yarn script start-presale -t <TOKEN_ADDRESS>
   ```

5. **Set stage:**
   ```bash
   yarn script set-stage -s <STAGE_NUMBER>
   ```

## Smart Contract Features

### Presale Stages
- 10 configurable stages with different pricing
- Automatic stage progression based on token sales
- Admin-controlled stage management

### Payment Methods
- **SOL payments** with real-time price feeds
- **USDC/USDT payments** for stable coin purchases
- Automatic price conversion and token allocation

### Admin Functions
- Initialize global state
- Set vault addresses
- Start/pause presale
- Manage presale stages
- Transfer admin rights

### User Functions
- Initialize user state
- Purchase tokens with SOL
- Purchase tokens with stable coins
- Track purchase history

## Security Features

- Admin-only functions with proper access control
- Input validation and error handling
- Pyth price feed integration for accurate pricing
- Comprehensive state management
- Secure token transfer mechanisms

## Project Structure

```
├── programs/presale/src/
│   ├── lib.rs              # Main program entry point
│   ├── state.rs            # Account state definitions
│   ├── error.rs            # Custom error types
│   ├── constant.rs         # Program constants and stages
│   ├── util.rs             # Utility functions
│   └── instructions/       # Instruction implementations
├── tests/                  # Test files
├── cli/                    # Command line tools
└── migrations/             # Deployment scripts
```

## License

This project is open source and available under the MIT License.

## Support

For questions, custom requirements, or support:
- **Telegram**: [@topsecretagent_007](https://t.me/topsecretagent_007)
- **GitHub Issues**: [Create an issue](https://github.com/topsecretagent007/token-presale-smart-contract/issues)

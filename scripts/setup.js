const anchor = require("@coral-xyz/anchor");
const { Connection, Keypair, PublicKey } = require("@solana/web3.js");
const fs = require("fs");

/**
 * Setup script for the presale smart contract
 * This script helps with initial deployment and configuration
 */

async function setupPresale() {
  console.log("🚀 Setting up presale smart contract...");
  
  try {
    // Check if admin.json exists
    if (!fs.existsSync("./admin.json")) {
      console.log("❌ admin.json not found. Please create it first.");
      console.log("You can generate it using: solana-keygen new -o admin.json");
      return;
    }

    // Load admin wallet
    const adminKeypair = Keypair.fromSecretKey(
      new Uint8Array(JSON.parse(fs.readFileSync("./admin.json", "utf-8")))
    );
    console.log("👛 Admin wallet loaded:", adminKeypair.publicKey.toString());

    // Connect to devnet
    const connection = new Connection("https://api.devnet.solana.com", "confirmed");
    
    // Check SOL balance
    const balance = await connection.getBalance(adminKeypair.publicKey);
    console.log("💰 SOL balance:", balance / 1e9, "SOL");
    
    if (balance < 2 * 1e9) {
      console.log("⚠️  Low SOL balance. Requesting airdrop...");
      const airdropSig = await connection.requestAirdrop(adminKeypair.publicKey, 2 * 1e9);
      await connection.confirmTransaction(airdropSig);
      console.log("✅ Airdrop received");
    }

    // Build the program
    console.log("🔨 Building program...");
    await anchor.build();
    
    // Get program ID
    const programId = new PublicKey("6En2g3XUQgZSBEgBE1DF1sVeRik4KNvug151Zswz8oR5");
    console.log("📋 Program ID:", programId.toString());

    // Create provider
    const provider = new anchor.AnchorProvider(connection, new anchor.Wallet(adminKeypair), {});
    anchor.setProvider(provider);

    // Get program
    const program = anchor.workspace.Presale;

    // Initialize global state
    console.log("🔧 Initializing global state...");
    const initTx = await program.methods.initialize()
      .accounts({
        admin: adminKeypair.publicKey,
      })
      .signers([adminKeypair])
      .rpc();
    
    console.log("✅ Global state initialized!");
    console.log("Transaction:", initTx);

    console.log("\n📋 Setup Information:");
    console.log("Program ID:", programId.toString());
    console.log("Admin:", adminKeypair.publicKey.toString());
    console.log("Network: Devnet");
    console.log("RPC: https://api.devnet.solana.com");
    
    console.log("\n🎯 Next steps:");
    console.log("1. Set vault address: yarn script set-vault -v <VAULT_ADDRESS>");
    console.log("2. Start presale: yarn script start-presale -t <TOKEN_ADDRESS>");
    console.log("3. Set stage: yarn script set-stage -s 1");
    
    console.log("\n✅ Setup completed successfully!");

  } catch (error) {
    console.error("❌ Setup failed:", error);
    throw error;
  }
}

// Run setup if called directly
if (require.main === module) {
  setupPresale().catch(console.error);
}

module.exports = { setupPresale }; 
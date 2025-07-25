const anchor = require("@coral-xyz/anchor");

module.exports = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  console.log("🚀 Starting presale deployment...");
  
  try {
    // Get the program
    const program = anchor.workspace.Presale;
    
    // Initialize the global state
    console.log("🔧 Initializing global state...");
    const tx = await program.methods.initialize()
      .accounts({
        admin: provider.wallet.publicKey,
      })
      .signers([provider.wallet])
      .rpc();
    
    console.log("✅ Global state initialized!");
    console.log("Transaction signature:", tx);
    
    console.log("📋 Deployment Information:");
    console.log("Program ID:", program.programId.toString());
    console.log("Admin:", provider.wallet.publicKey.toString());
    
    console.log("✅ Deployment completed successfully!");
  } catch (error) {
    console.error("❌ Deployment failed:", error);
    throw error;
  }
}; 
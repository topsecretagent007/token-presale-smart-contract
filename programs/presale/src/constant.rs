use solana_program::{pubkey, pubkey::Pubkey};

/// Seeds for PDA derivation
pub const GLOBAL_SEED: &[u8] = b"presale-global";
pub const USER_SEED: &[u8] = b"presale-user";

/// Stable coin addresses
/// Mainnet addresses (commented out for reference)
// pub const USDC_ADDRESS: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
// pub const USDT_ADDRESS: Pubkey = pubkey!("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB");

/// Test addresses for development
pub const USDC_ADDRESS: Pubkey = pubkey!("usdRLypwfSeEUw4DhUcscCcju6zzBviXymFBRjcBXTw");
pub const USDT_ADDRESS: Pubkey = pubkey!("usderEuWoVkjMcc3bEYkGopx78La8mHzt6YGdmErrpz");

/// Number of presale stages
pub const NUM_STAGES: u8 = 10;

/// Token decimals (6 decimals for most SPL tokens)
pub const TOKEN_DECIMALS: u64 = 1_000_000;

/// Pyth price feed for SOL/USD on Solana mainnet-beta
pub const SOL_USD_FEED: Pubkey = pubkey!("H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG");

/// Staleness threshold for price feeds (in seconds)
pub const STALENESS_THRESHOLD: u64 = 60;

/// Stage configuration structure
#[derive(Debug, Clone)]
pub struct Stage {
    pub index: u8,
    pub price: u64,  // Price in USD (with 6 decimals)
    pub amount: u64, // Token amount available for this stage
}

/// Presale stages configuration
/// Each stage has a different price and token allocation
pub const STAGES: [Stage; 10] = [
    Stage { 
        index: 1,  
        price: 2_000_000,    // $2.00 per token
        amount: 2_500_000     // 2.5M tokens
    },
    Stage { 
        index: 2,  
        price: 3_000_000,    // $3.00 per token
        amount: 2_500_000     // 2.5M tokens
    },
    Stage { 
        index: 3,  
        price: 4_000_000,    // $4.00 per token
        amount: 6_250_000     // 6.25M tokens
    },
    Stage { 
        index: 4,  
        price: 5_000_000,    // $5.00 per token
        amount: 27_500_000    // 27.5M tokens
    },
    Stage { 
        index: 5,  
        price: 5_500_000,    // $5.50 per token
        amount: 37_500_000    // 37.5M tokens
    },
    Stage { 
        index: 6,  
        price: 6_000_000,    // $6.00 per token
        amount: 41_250_000    // 41.25M tokens
    },
    Stage { 
        index: 7,  
        price: 6_500_000,    // $6.50 per token
        amount: 37_500_000    // 37.5M tokens
    },
    Stage { 
        index: 8,  
        price: 7_000_000,    // $7.00 per token
        amount: 35_000_000    // 35M tokens
    },
    Stage { 
        index: 9,  
        price: 8_000_000,    // $8.00 per token
        amount: 7_500_000     // 7.5M tokens
    },
    Stage { 
        index: 10, 
        price: 9_000_000,    // $9.00 per token
        amount: 2_500_000     // 2.5M tokens
    },
];

/// Get stage by index
pub fn get_stage_by_index(index: u8) -> Option<&'static Stage> {
    if index > 0 && index <= NUM_STAGES {
        Some(&STAGES[(index - 1) as usize])
    } else {
        None
    }
}

/// Get total tokens across all stages
pub fn get_total_tokens() -> u64 {
    STAGES.iter().map(|stage| stage.amount).sum()
}

/// Get total USD value if all tokens are sold
pub fn get_total_usd_value() -> u64 {
    STAGES.iter().map(|stage| stage.price * stage.amount).sum()
}

use crate::*;

/**
 * Stores global state of the presale program
 * This account holds all the configuration and statistics for the presale
 */
#[account]
#[derive(Default)]
pub struct GlobalState {
    /// Admin address of this program
    pub admin: Pubkey,

    /// Address of the vault/multi-sig wallet for receiving payments
    pub vault: Pubkey,

    /// Total tokens sold across all stages
    pub token_sold: u64,

    /// Total USD value received from all sales
    pub token_sold_usd: u64,

    /// Whether the presale is currently live (active)
    pub is_live: bool,

    /// Current stage number (0-9, where 0 means not started)
    pub stage_iterator: u8,

    /// Remaining token amounts for each stage
    pub remain_tokens: [u64; NUM_STAGES as usize]
}

impl GlobalState {
    /// Update remain_tokens from STAGES configuration
    pub fn update_remain_tokens(&mut self) {
        for (i, stage) in STAGES.iter().enumerate() {
            self.remain_tokens[i] = stage.amount;
        }
    }

    /// Check if the presale is active
    pub fn is_presale_active(&self) -> bool {
        self.is_live && self.stage_iterator > 0
    }

    /// Get current stage info
    pub fn get_current_stage(&self) -> Option<&Stage> {
        if self.stage_iterator > 0 && self.stage_iterator <= NUM_STAGES {
            Some(&STAGES[(self.stage_iterator - 1) as usize])
        } else {
            None
        }
    }

    /// Check if current stage is sold out
    pub fn is_current_stage_sold_out(&self) -> bool {
        if let Some(stage_index) = self.stage_iterator.checked_sub(1) {
            if stage_index < NUM_STAGES {
                return self.remain_tokens[stage_index as usize] == 0;
            }
        }
        true
    }

    /// Get remaining tokens for current stage
    pub fn get_current_stage_remaining(&self) -> u64 {
        if let Some(stage_index) = self.stage_iterator.checked_sub(1) {
            if stage_index < NUM_STAGES {
                return self.remain_tokens[stage_index as usize];
            }
        }
        0
    }
}

/**
 * Stores user information and purchase history
 * Each user has their own state account to track their participation
 */
#[account]
#[derive(Default)]
pub struct UserState {
    /// User's wallet address
    pub user: Pubkey,

    /// Total token amount user has purchased
    pub tokens: u64,

    /// Total SOL amount user has paid
    pub paid_sol: u64,

    /// Total stable coin amount user has paid (in USD equivalent)
    pub paid_usd: u64
}

impl UserState {
    /// Get total USD value of user's purchases
    pub fn get_total_paid_usd(&self) -> u64 {
        self.paid_usd
    }

    /// Check if user has made any purchases
    pub fn has_purchases(&self) -> bool {
        self.tokens > 0 || self.paid_sol > 0 || self.paid_usd > 0
    }

    /// Add a SOL purchase
    pub fn add_sol_purchase(&mut self, sol_amount: u64, token_amount: u64) {
        self.paid_sol = self.paid_sol.saturating_add(sol_amount);
        self.tokens = self.tokens.saturating_add(token_amount);
    }

    /// Add a stable coin purchase
    pub fn add_stable_purchase(&mut self, stable_amount: u64, token_amount: u64) {
        self.paid_usd = self.paid_usd.saturating_add(stable_amount);
        self.tokens = self.tokens.saturating_add(token_amount);
    }
}

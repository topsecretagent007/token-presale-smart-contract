use crate::*;

/// Initialize the presale program
/// This instruction can only be called once by the admin to set up the global state
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// Admin wallet that will control the presale
    #[account(mut)]
    pub admin: Signer<'info>,

    /// Global state account to store presale configuration
    #[account(
        init,
        space = 8 + std::mem::size_of::<GlobalState>(),
        seeds = [GLOBAL_SEED],
        bump,
        payer = admin
    )]
    pub global_state: Account<'info, GlobalState>,

    /// System program for account creation
    pub system_program: Program<'info, System>,
}

impl Initialize<'_> {
    /// Process the initialize instruction
    pub fn process_instruction(ctx: Context<Self>) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;

        msg!("Initializing presale program");
        msg!("Global state address: {}", global_state.key());

        // Validate stage configuration
        require!(validate_stages(), PresaleError::NotEnoughToken);

        // Initialize global state with default values
        global_state.admin = ctx.accounts.admin.key();
        global_state.vault = Pubkey::default(); // Will be set later

        // Initialize statistics
        global_state.token_sold = 0;
        global_state.token_sold_usd = 0;
        
        // Presale starts as inactive
        global_state.is_live = false;
        global_state.stage_iterator = 0;

        // Initialize remaining tokens for all stages
        global_state.remain_tokens = [0; NUM_STAGES as usize];
        global_state.update_remain_tokens();

        msg!("Presale initialized successfully");
        msg!("Admin: {}", global_state.admin);
        msg!("Total tokens available: {}", get_total_tokens());
        msg!("Total USD value if all sold: ${}", get_total_usd_value() as f64 / 1_000_000.0);

        Ok(())
    }
}

use crate::*;

use pyth_sdk_solana::state::SolanaPriceAccount;
use solana_program::native_token::LAMPORTS_PER_SOL;

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(
        mut
    )]
    pub user: Signer<'info>,

    //  Global state
    #[account(
        mut,
        seeds = [GLOBAL_SEED],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,

    //  User pool stores user's buy info
    #[account(
        mut,
        seeds = [user.key().as_ref(), USER_SEED.as_ref()],
        bump
    )]
    pub user_state: Account<'info, UserState>,
    
    #[account(
        mut
    )]
    /// CHECK: vault address(multi-sig wallet)
    pub vault: AccountInfo<'info>,

    
    #[account(address = SOL_USD_FEED @PresaleError::InvalidPriceFeed)]
    /// CHECK:
    pub price_feed: AccountInfo<'info>,

    //  Needed to transfer SOL
    pub system_program: Program<'info, System>
}

impl Buy<'_> {
    pub fn process_instruction(ctx: Context<Self>, mut sol_amount: u64) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;
        let stage_iterator = global_state.stage_iterator;

        //  check presale is live
        if global_state.is_live == false {
            match stage_iterator {
                0 => return Err(error!(PresaleError::PresaleNotStarted)),
                stage if stage > NUM_STAGES => return Err(error!(PresaleError::PresaleEnded)),
                _ => return Err(error!(PresaleError::PresalePaused)),
            }
        }

        // Validate stage iterator is within bounds
        if stage_iterator == 0 || stage_iterator > NUM_STAGES {
            return Err(error!(PresaleError::InvalidStageNumber));
        }

        let stage_index = (stage_iterator - 1) as usize;
        if stage_index >= STAGES.len() {
            return Err(error!(PresaleError::InvalidStageNumber));
        }

        //  get SOL price
        // Retrieve Pyth price with proper error handling
        let price_account_info = &ctx.accounts.price_feed;
        let price_feed = SolanaPriceAccount::account_info_to_feed(price_account_info)
            .map_err(|_| error!(PresaleError::InvalidPriceFeed))?;
        
        let timestamp = Clock::get()?.unix_timestamp;
        let price_data = price_feed.get_price_no_older_than(timestamp, STALENESS_THRESHOLD)
            .ok_or(error!(PresaleError::StalePriceFeed))?;
        
        let asset_price = price_data.price;

        msg!("SOL/USD price: {}", asset_price);

        //  calculate token amount from sol_amount and token price
        let mut token_amount = (asset_price as f64 / 100.0 / STAGES[stage_index].price as f64 * (sol_amount / LAMPORTS_PER_SOL) as f64) as u64;
        msg!("token amount: {}", token_amount);

        //  check if enough token remain
        if global_state.remain_tokens[stage_index] < token_amount {
            msg!("not enough tokens in this stage");

            //  fix token_amount and sol_amount
            token_amount = global_state.remain_tokens[stage_index];
            sol_amount = (STAGES[stage_index].price as f64 / (asset_price as f64 / 100.0) * (LAMPORTS_PER_SOL * token_amount) as f64) as u64;

            //  go to next stage
            global_state.stage_iterator = stage_iterator + 1;

            //  end presale if current stage is the last one
            if stage_iterator == NUM_STAGES {
                global_state.is_live = false;
            }
        }

        //  minus remain tokens in the current stage
        global_state.remain_tokens[stage_index] -= token_amount;

        //  add total tokens sold
        global_state.token_sold += token_amount;

        //  add total USD received
        global_state.token_sold_usd += asset_price as u64 * sol_amount;
        
        //  add user info
        let user_state = &mut ctx.accounts.user_state;
        user_state.tokens += token_amount;
        user_state.paid_sol += sol_amount;

        //  transfer SOL to vault
        sol_transfer_user(
            ctx.accounts.user.to_account_info().clone(),
            ctx.accounts.vault.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            sol_amount
        )
    }
}

use crate::*;

#[error_code]
pub enum PresaleError {
    #[msg("Admin address mismatch")]
    InvalidAdmin,

    #[msg("Token address mismatch")]
    InvalidToken,

    #[msg("Token amount is not enough for all stages")]
    NotEnoughToken,

    #[msg("Presale stage number is not correct")]
    PresaleNumberInvalid,
    
    #[msg("Presale is not started")]
    PresaleNotStarted,
    
    #[msg("Presale is ended")]
    PresaleEnded,
    
    #[msg("Presale is paused")]
    PresalePaused,
    
    #[msg("Pyth feed address is not correct")]
    InvalidPriceFeed,
    
    #[msg("Stable token address is not correct")]
    InvalidStableToken,

    #[msg("Invalid stage number")]
    InvalidStageNumber,

    #[msg("Stage is sold out")]
    StageSoldOut,

    #[msg("Insufficient payment amount")]
    InsufficientPayment,

    #[msg("Invalid vault address")]
    InvalidVaultAddress,

    #[msg("User state not initialized")]
    UserNotInitialized,

    #[msg("Global state not initialized")]
    GlobalStateNotInitialized,

    #[msg("Invalid token amount")]
    InvalidTokenAmount,

    #[msg("Price feed is stale")]
    StalePriceFeed,

    #[msg("Invalid stable coin amount")]
    InvalidStableCoinAmount,

    #[msg("Presale has already ended")]
    PresaleAlreadyEnded,
}

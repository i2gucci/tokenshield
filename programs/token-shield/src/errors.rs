use anchor_lang::prelude::*;

#[error_code]
pub enum TokenShieldError {
    #[msg("Token does not meet eligibility criteria")]
    TokenNotEligible,
    
    #[msg("Invalid coverage level (must be between 30% and 70%)")]
    InvalidCoverageLevel,
    
    #[msg("Invalid duration (must be 7-30 days)")]
    InvalidDuration,
    
    #[msg("Position size exceeds maximum ($50k)")]
    PositionTooLarge,
    
    #[msg("Insufficient pool liquidity for policy")]
    InsufficientLiquidity,
    
    #[msg("Policy has expired")]
    PolicyExpired,
    
    #[msg("Policy has already been paid out")]
    PolicyAlreadyPaidOut,
    
    #[msg("Trigger conditions not met")]
    TriggerNotMet,
    
    #[msg("Invalid oracle data")]
    InvalidOracleData,
    
    #[msg("VRF proof verification failed")]
    InvalidVRFProof,
    
    #[msg("Arithmetic overflow")]
    MathOverflow,
    
    #[msg("Pool collateral ratio below minimum")]
    InsufficientCollateral,
    
    #[msg("Team not authorized for this token")]
    UnauthorizedTeam,
    
    #[msg("Holder has opted out of team coverage")]
    HolderOptedOut,
    
    #[msg("Premium calculation error")]
    PremiumCalculationError,
    
    #[msg("Policy token mint failed")]
    PolicyTokenMintFailed,
    
    #[msg("Oracle staleness exceeded")]
    StaleOracleData,
}

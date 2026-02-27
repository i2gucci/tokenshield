use anchor_lang::prelude::*;
use crate::{state::*, constants::*, errors::*};

#[derive(Accounts)]
#[instruction(policy_id: [u8; 32])]
pub struct OptOutTeamCoverage<'info> {
    #[account(
        mut,
        seeds = [POLICY_SEED, &policy_id],
        bump = policy.bump,
        constraint = policy.is_team_policy @ TokenShieldError::UnauthorizedTeam,
        constraint = policy.status == PolicyStatus::Active
    )]
    pub policy: Account<'info, Policy>,
    
    /// Covered wallet owner (must be the holder)
    #[account(
        constraint = holder.key() == policy.covered_wallet
    )]
    pub holder: Signer<'info>,
}

pub fn opt_out_team_coverage(
    ctx: Context<OptOutTeamCoverage>,
    policy_id: [u8; 32],
) -> Result<()> {
    let policy = &mut ctx.accounts.policy;
    let clock = Clock::get()?;
    
    // Check opt-out window (48 hours from policy start)
    let opt_out_deadline = policy.start_time + (2 * SECONDS_PER_DAY);
    require!(
        clock.unix_timestamp <= opt_out_deadline,
        TokenShieldError::PolicyExpired
    );
    
    // Mark policy as opted out
    policy.opted_out = true;
    policy.status = PolicyStatus::Cancelled;
    
    msg!(
        "Holder {:?} opted out of team policy {:?}",
        policy.covered_wallet,
        policy_id
    );
    
    Ok(())
}

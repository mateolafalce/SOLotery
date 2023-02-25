use anchor_lang::prelude::*;
use instructions::*;
use crate::errors::ErrorCode;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("FMz7qxxUeqgCKZL2z96nBhp6mpyisdVEEuS4ppZG3bmH");

#[program]
pub mod solotery {
    use super::*;
    pub fn create_stake(ctx: Context<Create>) -> Result<()> {
        instructions::initialize::create_stake(ctx)
    }
    pub fn ticket(ctx: Context<Ticket>) -> Result<()> {
        instructions::ticket::ticket(ctx)
    }
}
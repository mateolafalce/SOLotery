use crate::{
    state::accounts::*,
    utils::consts::{DAY_OF_DEPLOY, SOLOTERY},
};
use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

pub fn create_stake(ctx: Context<Create>) -> Result<()> {
    let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
    let (_stake_pda, bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[SOLOTERY], ctx.program_id);
    // set SOLotery init variables
    solotery.set_bump_original(bump);
    solotery.reset_players();
    solotery.set_time_check(DAY_OF_DEPLOY);
    solotery.reset_winner_selected();
    solotery.init_ticket_sold();
    solotery.reset_winner_pubkey();
    Ok(())
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, seeds = [SOLOTERY], bump, payer = user, space = SoLotery::SIZE)]
    pub solotery: Account<'info, SoLotery>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

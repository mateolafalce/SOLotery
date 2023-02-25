use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
}; 
use std::str::FromStr;
use crate::state::accounts::*;

pub fn create_stake(
    ctx: Context<Create>
) -> Result<()> {
    let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
    let (_stake_pda, bump) = Pubkey::find_program_address(&[b"SOLotery"], ctx.program_id);
    solotery.bump_original = bump;
    solotery.players1 = [].to_vec();
    solotery.players2 = [].to_vec();
    solotery.time_check = 1662260159;
    solotery.players_state = false;
    solotery.winner1_selected = false;
    solotery.winner2_selected = false;
    solotery.tickets_sold = 0;
    solotery.winner_publickey = Pubkey::from_str("11111111111111111111111111111111").unwrap();
    Ok(())
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, seeds = [b"SOLotery"], bump, payer = user, space = SoLotery::SIZE + 8)]
    pub solotery: Account<'info, SoLotery>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
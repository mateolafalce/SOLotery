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
    // Set the SOLotery account's bump value to the value returned by find_program_address.
    solotery.bump_original = bump;
    // Initialize the SOLotery account's players1 and players2 arrays to empty arrays.
    solotery.players1 = [].to_vec();
    solotery.players2 = [].to_vec();
    solotery.time_check = 1662260159; // Set the SOLotery account's time_check value to a fixed timestamp (1662260159).
    // Set the SOLotery account's players_state, winner1_selected, and winner2_selected fields to false.
    solotery.players_state = false;
    solotery.winner1_selected = false;
    solotery.winner2_selected = false;
    solotery.tickets_sold = 0;
    // Set the system program id
    solotery.winner_publickey = Pubkey::from_str("11111111111111111111111111111111").unwrap();
    Ok(())
}

#[derive(Accounts)]
pub struct Create<'info> {
    // The SOLotery account to be created. It must be initialized with the SOLotery::SIZE + 8 bytes of space.
    #[account(init, seeds = [b"SOLotery"], bump, payer = user, space = SoLotery::SIZE + 8)]
    pub solotery: Account<'info, SoLotery>,
    // The user account that will pay for the SOLotery account's initialization.
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

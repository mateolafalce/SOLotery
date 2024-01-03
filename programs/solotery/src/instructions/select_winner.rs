use crate::state::SoLotery;
use anchor_lang::prelude::*;
use oorandom::Rand64;

pub fn select_winner(solotery: &mut Account<SoLotery>) -> Result<()> {
    let players_amount: u64 = solotery.players.len() as u64;
    let current_time: u128 = Clock::get().unwrap().unix_timestamp as u128;
    let mut rng: Rand64 = Rand64::new(current_time);
    let index_winner: usize = rng.rand_range(0..players_amount).try_into().unwrap();
    let winner: Pubkey = solotery.players[index_winner];
    solotery.set_winner_pubkey(winner);
    solotery.print_winner();
    Ok(())
}

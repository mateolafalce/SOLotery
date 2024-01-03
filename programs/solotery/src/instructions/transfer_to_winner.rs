use crate::{state::SoLotery, utils::consts::TICKET_PRICE};
use anchor_lang::prelude::*;

pub fn transfer_to_winner(
    solotery: &mut Account<SoLotery>,
    winner: &mut AccountInfo,
) -> Result<()> {
    let amount_players: u64 = solotery.players.len() as u64;
    let amount: u64 = amount_players * TICKET_PRICE;
    // Transfer the funds & reset the game
    **solotery.to_account_info().try_borrow_mut_lamports()? -= amount;
    **winner.to_account_info().try_borrow_mut_lamports()? += amount;
    solotery.add_a_month();
    solotery.reset_players();
    solotery.reset_winner_pubkey();
    solotery.reset_winner_selected();
    solotery.print_transfer_amount(amount);
    Ok(())
}

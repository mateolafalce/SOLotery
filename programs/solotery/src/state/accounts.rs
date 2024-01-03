use crate::utils::{
    consts::{MAX_PLAYERS, MONTH},
    functions::lamports_to_sol,
};
use anchor_lang::{prelude::*, system_program};

#[account]
pub struct SoLotery {
    pub bump_original: u8,     // 1
    pub winner_pubkey: Pubkey, // 32
    pub players: Vec<Pubkey>,  // 4 + (32 * MAX_PLAYERS)
    pub time_check: i64,       // 8
    pub winner_selected: bool, // 1
    pub tickets_sold: u64,     // 8
}

impl SoLotery {
    pub const SIZE: usize = 1 + 32 + 4 + (32 * MAX_PLAYERS) + 8 + 1 + 8;

    pub fn add_tickets_sold(&mut self) {
        self.tickets_sold += 1;
    }

    pub fn reset_winner_pubkey(&mut self) {
        self.winner_pubkey = system_program::ID;
    }

    pub fn reset_winner_selected(&mut self) {
        self.winner_selected = false;
    }

    pub fn reset_players(&mut self) {
        self.players = [].to_vec();
    }

    pub fn print_transfer_amount(&self, amount: u64) {
        msg!("Total amount: {} SOL", lamports_to_sol(amount));
    }

    pub fn add_a_month(&mut self) {
        self.time_check += MONTH;
    }

    pub fn set_winner_pubkey(&mut self, winner: Pubkey) {
        self.winner_pubkey = winner;
    }

    pub fn print_winner(&self) {
        msg!("Chosen winner: {}", self.winner_pubkey);
    }

    pub fn init_ticket_sold(&mut self) {
        self.tickets_sold = 0;
    }

    pub fn set_time_check(&mut self, day: i64) {
        self.time_check = day;
    }

    pub fn set_bump_original(&mut self, bump: u8) {
        self.bump_original = bump;
    }
}

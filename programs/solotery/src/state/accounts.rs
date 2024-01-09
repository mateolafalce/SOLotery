use crate::utils::consts::{ANCHOR_BUFFER, MAX_PLAYERS, MONTH, TICKET_PRICE};
use anchor_lang::{prelude::*, system_program};
use oorandom::Rand64;

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
    pub const SIZE: usize = 1 + 32 + 4 + (32 * MAX_PLAYERS) + 8 + 1 + 8 + ANCHOR_BUFFER;

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

    pub fn print_transfer_amount(&self, lamports: u64) {
        let amount: f64 = lamports as f64 / 1_000_000_000.0;
        msg!("Total amount: {} SOL", amount);
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

    pub fn select_winner(&mut self) -> Result<()> {
        let players_amount: u64 = self.players.len() as u64;
        let current_time: u128 = Clock::get().unwrap().unix_timestamp as u128;
        let mut rng: Rand64 = Rand64::new(current_time);
        let index_winner: usize = rng.rand_range(0..players_amount).try_into().unwrap();
        let winner: Pubkey = self.players[index_winner];
        self.set_winner_pubkey(winner);
        self.print_winner();
        Ok(())
    }

    pub fn transfer_to_winner(
        &mut self,
        solotery_ctx: AccountInfo,
        winner_ctx: AccountInfo,
    ) -> Result<()> {
        let amount_players: u64 = self.players.len() as u64;
        let amount: u64 = amount_players * TICKET_PRICE;
        // Transfer the funds & reset the game
        **solotery_ctx.to_account_info().try_borrow_mut_lamports()? -= amount;
        **winner_ctx.to_account_info().try_borrow_mut_lamports()? += amount;
        self.add_a_month();
        self.reset_players();
        self.reset_winner_pubkey();
        self.reset_winner_selected();
        self.print_transfer_amount(amount);
        Ok(())
    }
}

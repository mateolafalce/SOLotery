use anchor_lang::prelude::*;

#[account]
pub struct SoLotery { 
    pub bump_original: u8,       // 1
    pub winner_publickey: Pubkey,// 32
    pub players_state: bool,     // 1
    pub players1: Vec<Pubkey>,   // 4 + (32*300)
    pub players2: Vec<Pubkey>,   // 4 + 32 
    pub time_check: i64,         // 8
    pub winner1_selected: bool,  // 1
    pub winner2_selected: bool,  // 1
    pub tickets_sold: u64        // 8
}

impl SoLotery {
    pub const SIZE: usize =  1 + 32 + 1 + 4 + (32 * 300) + 4 + 32 + 8 + 1 + 1 + 8;
}

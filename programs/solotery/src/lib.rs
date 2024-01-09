use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("Caj6vkJqXNP5rKVkQin3QPecjvZSoyujNUd5HvGmxVGX");

#[program]
pub mod solotery {
    use super::*;

    pub fn create_stake(ctx: Context<Create>) -> Result<()> {
        instructions::initialize::create_stake(ctx)
    }
    pub fn sold_ticket(ctx: Context<Ticket>) -> Result<()> {
        instructions::ticket::ticket(ctx)
    }
}

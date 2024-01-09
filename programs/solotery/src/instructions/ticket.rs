use crate::{
    state::accounts::*,
    utils::consts::{MAX_PLAYERS, SOLOTERY, TICKET_PRICE},
};
use anchor_lang::{
    prelude::*,
    solana_program::{
        account_info::AccountInfo, program::invoke, pubkey::Pubkey, system_instruction::transfer,
    },
};

pub fn ticket(ctx: Context<Ticket>) -> Result<()> {
    // useful variables
    let stake_account: Pubkey = ctx.accounts.stake.key();
    let winner_pass_as_arg: Pubkey = ctx.accounts.winner_publickey.key();
    let real_winner: Pubkey = ctx.accounts.solotery.winner_pubkey.key();
    let lamport_from_account: u64 = ctx.accounts.from.to_account_info().lamports();
    let correct_pda = ctx.accounts.solotery.key();
    let solotery_ctx: AccountInfo = ctx.accounts.stake.clone();
    let winner_ctx: AccountInfo = ctx.accounts.winner_publickey.clone();
    // validations
    require_keys_eq!(stake_account, correct_pda);
    require_keys_eq!(winner_pass_as_arg, real_winner);
    let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
    if !solotery.winner_selected {
        // check lamports, transfer SOL & give a ticket
        require_gte!(lamport_from_account, TICKET_PRICE);
        let from_account: Pubkey = ctx.accounts.from.key();
        let solotery_account: Pubkey = solotery.key();
        let current_time: i64 = Clock::get().unwrap().unix_timestamp;
        let stablished_time: i64 = solotery.time_check;
        invoke(
            &transfer(&from_account, &solotery_account, TICKET_PRICE),
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.stake.to_account_info().clone(),
            ],
        )
        .expect("Error tranfering SOL from user to stake account");
        // update state of lotery
        solotery.add_tickets_sold();
        solotery.players.push(from_account);
        if solotery.players.len() == MAX_PLAYERS || current_time > stablished_time {
            solotery
                .select_winner()
                .expect("Error selecting SOL to winner");
        }
    } else {
        solotery
            .transfer_to_winner(solotery_ctx, winner_ctx)
            .expect("Error transfering SOL to winner");
    }
    Ok(())
}

#[derive(Accounts)]
pub struct Ticket<'info> {
    #[account(mut, seeds = [SOLOTERY], bump = solotery.bump_original)]
    pub solotery: Account<'info, SoLotery>,
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub stake: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    pub winner_publickey: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

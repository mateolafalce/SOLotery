use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::system_instruction,
    solana_program::pubkey::Pubkey,
};
use std::str::FromStr;
use oorandom;
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn ticket(ctx: Context<Ticket>) -> Result<()> {
    let winner: &mut AccountInfo = &mut ctx.accounts.winner_publickey;
    let (correct_pda, _bump) = Pubkey::find_program_address(
        &[b"SOLotery"],
        &Pubkey::from_str("FMz7qxxUeqgCKZL2z96nBhp6mpyisdVEEuS4ppZG3bmH"
    ).unwrap());
    // Ensure that the stake account provided in the context matches the correct PDA for the program.
    require!(ctx.accounts.stake.key() == correct_pda.key(), ErrorCode::WrongStake);
    // Ensure that the winner's account provided in the context matches the expected winner for the lottery.
    require!(winner.key() ==  ctx.accounts.solotery.winner_publickey.key(), ErrorCode::ThisIsNotTheWinner);
    // Ensure that the sender has provided enough SOL to redeem the ticket.
    require!(AccountInfo::lamports(&ctx.accounts.from.to_account_info()) >= 7777777, ErrorCode::AmountError);
    let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        if solotery.players_state == false {
            anchor_lang::solana_program::program::invoke(
                &system_instruction::transfer(&ctx.accounts.from.key(), &solotery.key(), 7777777),
                &[ctx.accounts.from.to_account_info(), ctx.accounts.stake.to_account_info().clone()],
            ).expect("Error");
            solotery.tickets_sold += 1;
            solotery.players1.push(ctx.accounts.from.key());
            let currents_players2: u64 = (solotery.players1.len() * 7777777).try_into().unwrap();
            if solotery.winner2_selected == true {
                select_winner2(
                    solotery,
                    winner
                ).unwrap();
            }
            if solotery.players1.len() == 300 {
                select_winner1(
                    solotery,
                    winner
                ).unwrap();
            }
            msg!(
                "SOL sent successfully. You are already participating for the current amount of: {} SOL",
                lamports_to_sol(currents_players2)
            );
            if Clock::get().unwrap().unix_timestamp > solotery.time_check.try_into().unwrap() {
                    dead_line(
                        solotery
                    )
                }
            } else {
                anchor_lang::solana_program::program::invoke(
                &system_instruction::transfer(&ctx.accounts.from.key(), &solotery.key(), 7777777),
                &[ctx.accounts.from.to_account_info(), ctx.accounts.stake.to_account_info().clone()],).expect("Error");
                solotery.tickets_sold += 1;
                solotery.players2.push(ctx.accounts.from.key());
                let currents_players1: u64 = (solotery.players2.len() * 7777777).try_into().unwrap();
                if solotery.winner1_selected == true {
                    let amount: u64 = (solotery.players1.len() * 7777777).try_into().unwrap();
                    **solotery.to_account_info().try_borrow_mut_lamports()? -= amount;
                    **winner.to_account_info().try_borrow_mut_lamports()? += amount;
                    solotery.players1 = [].to_vec();
                    solotery.winner_publickey = Pubkey::from_str("11111111111111111111111111111111").unwrap();
                    solotery.winner1_selected = false;
                    msg!("Total amount: {} SOL", lamports_to_sol(amount));
                }
                if solotery.players2.len() == 300 {
                    require!(solotery.winner2_selected == false, ErrorCode::WinnerChosen);
                    solotery.players_state = false;
                    let mut rng: oorandom::Rand64 = oorandom::Rand64::new((Clock::get().unwrap().unix_timestamp as u64).into());
                    let winner_choosed: usize = rng.rand_range(1..(solotery.players2.len() as u64)).try_into().unwrap();
                    solotery.winner_publickey =  solotery.players2[winner_choosed - 1];
                    solotery.winner2_selected = true;
                    solotery.time_check += 86398;
                    msg!("Chosen winner: {}", solotery.winner_publickey);
                }
                msg!("SOL sent successfully. You are already participating for the current amount of: {} SOL", lamports_to_sol(currents_players1));
                if Clock::get().unwrap().unix_timestamp > solotery.time_check.try_into().unwrap() {
                    require!(solotery.winner2_selected == false, ErrorCode::WinnerChosen);
                    solotery.players_state = false ;
                    let mut rng: oorandom::Rand64 = oorandom::Rand64::new((Clock::get().unwrap().unix_timestamp as u64).into());
                    let winner_choosed: usize = rng.rand_range(1..(solotery.players2.len() as u64)).try_into().unwrap();
                    solotery.winner_publickey =  solotery.players2[winner_choosed - 1];
                    solotery.winner2_selected = true;
                    solotery.time_check += 86398;
                    msg!("Chosen winner: {}", solotery.winner_publickey);
                }
            }
    Ok(())
}


pub fn lamports_to_sol(lamport: u64) -> f64 {
    let am: f64 = lamport as f64;
    // Divide the input value by 1 billion to convert from lamports to Sol, and return the result.
    return (am / 1000000000.0) as f64;
}

pub fn select_winner2(
    solotery: &mut Account<SoLotery>,
    winner: &mut AccountInfo
) -> Result<()> {
    let amount: u64 = (solotery.players2.len() * 7777777).try_into().unwrap();
    // Transfer the funds from the lottery account to the winner's account
    **solotery.to_account_info().try_borrow_mut_lamports()? -= amount;
    **winner.to_account_info().try_borrow_mut_lamports()? += amount;
    // Clear the list of players for the next round of the lottery
    solotery.players2 = [].to_vec();
    // Set the winner public key to a default value
    solotery.winner_publickey = Pubkey::from_str("11111111111111111111111111111111").unwrap();
    solotery.winner2_selected = false;
    msg!("Total amount: {} SOL", lamports_to_sol(amount)); // Print the total amount
}

pub fn select_winner1(
    solotery: &mut Account<SoLotery>,
    winner: &mut AccountInfo
) -> Result<()> {
    require!(solotery.winner1_selected == false, ErrorCode::WinnerChosen); // Check that a winner has not already been chosen.
    solotery.players_state = true;
    let mut rng: oorandom::Rand64 = oorandom::Rand64::new((Clock::get().unwrap().unix_timestamp as u64).into()); // Create a new random number generator.
    let winner_choosed: usize = rng.rand_range(1..(solotery.players1.len() as u64)).try_into().unwrap(); // Generate a random number within the range of players and convert it to a usize.
    solotery.winner_publickey =  solotery.players1[winner_choosed - 1]; // Assign the winner's public key to the lottery account.
    solotery.winner1_selected = true;
    solotery.time_check += 86398; // Add 23 hours and 59 minutes to the lottery's time check.
    msg!("Chosen winner: {}", solotery.winner_publickey);
}

pub fn dead_line(
    solotery: &mut Account<SoLotery>,
) -> <()> {
    require!(solotery.winner1_selected == false, ErrorCode::WinnerChosen);
    solotery.players_state = true ;
    let mut rng: oorandom::Rand64 = oorandom::Rand64::new((Clock::get().unwrap().unix_timestamp as u64).into());
    let winner_choosed: usize = rng.rand_range(1..(solotery.players1.len() as u64)).try_into().unwrap();
    solotery.winner_publickey =  solotery.players1[winner_choosed - 1];
    solotery.winner1_selected = true;
    solotery.time_check += 86398;
    msg!("Chosen winner: {}", solotery.winner_publickey);
}


#[derive(Accounts)]
pub struct Ticket<'info> {
    #[account(mut, seeds = [b"SOLotery"], bump = solotery.bump_original)]
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

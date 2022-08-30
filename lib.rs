use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::system_instruction,
    solana_program::pubkey::Pubkey,
}; 
use std::str::FromStr;
use oorandom;

declare_id!("4nDfK4a6Ag4cgckw9tdjC3qALsU2daxa2fbAF3a4tdCC");

#[program]
pub mod mateosolotery {
    use super::*;
    pub fn create_stake(
        ctx: Context<Create>
    ) -> Result<()> {
        if ctx.accounts.user.key() != Pubkey::from_str("9RDz3M796x25qXfVGUSau3rze3WH4z8KesZe3MMBYfrZ").unwrap() {
            return Err(ErrorCode::YouAreNotSOLotery.into());
        }
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        let (stake_pda, bump) = Pubkey::find_program_address(&[b"SOLotery", ctx.accounts.user.key().as_ref()], &Pubkey::from_str("4nDfK4a6Ag4cgckw9tdjC3qALsU2daxa2fbAF3a4tdCC").unwrap());
        solotery.authority = ctx.accounts.user.key();
        solotery.stake_pda = stake_pda;
        solotery.american_stake = 1;
        solotery.lifetime255 = 1;
        solotery.choose_winner_only_one_time = 0;
        solotery.bump_original = bump;
        solotery.secure_check = 1661810400;
        solotery.winner = 0;   
        Ok(())
    }
    pub fn ticket(
        ctx: Context<AmericanTicket>,
    ) -> Result<()> {
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        let ticket: &mut Account<TicketStruct> = &mut ctx.accounts.ticket;
        ticket.authority = ctx.accounts.from.key();
        if solotery.lifetime255 == 255 {
            return Err(ErrorCode::RestartTheProgram.into());
        } 
        if solotery.american_stake == 254 {
            return Err(ErrorCode::Limit.into());
        }
        solotery.american_stake += 1;
        if ctx.accounts.stake.key() != solotery.stake_pda {
            return Err(ErrorCode::WrongStake.into());
        }
        anchor_lang::solana_program::program::invoke(
            &system_instruction::transfer(&ctx.accounts.from.key(), &solotery.stake_pda, 7777777),
            &[ctx.accounts.from.to_account_info(), ctx.accounts.stake.to_account_info().clone()],).expect("Error");
        Ok(())
    }
    pub fn choose_winner(
        ctx: Context<Winner>,
    ) -> Result<()> {
        if ctx.accounts.solotery_authority.key() != Pubkey::from_str("9RDz3M796x25qXfVGUSau3rze3WH4z8KesZe3MMBYfrZ").unwrap() {
            return Err(ErrorCode::YouAreNotSOLotery.into());
        }
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        if solotery.choose_winner_only_one_time == 1 {
            return Err(ErrorCode::JustOnce.into());
        }
        if Clock::get().unwrap().unix_timestamp < solotery.secure_check {
            return Err(ErrorCode::IncorrectTimestamp.into());
        }
        solotery.choose_winner_only_one_time += 1;
        if solotery.american_stake == 1 {
            solotery.secure_check += 86398;
            solotery.choose_winner_only_one_time -= 1;
            solotery.lifetime255 += 1;
        }
        if solotery.american_stake > 1 {
            let mut rng: oorandom::Rand64 = oorandom::Rand64::new((Clock::get().unwrap().unix_timestamp as u64).into());
            let winner: usize = rng.rand_range(1..(solotery.american_stake as u64)).try_into().unwrap();
            solotery.winner =  winner as u8;
            let (winner_publickey, bump) = Pubkey::find_program_address(&[solotery.lifetime255.to_le_bytes().as_ref(), solotery.winner.to_le_bytes().as_ref()], &Pubkey::from_str("4nDfK4a6Ag4cgckw9tdjC3qALsU2daxa2fbAF3a4tdCC").unwrap());
            solotery.winner_publickey = winner_publickey;
            solotery.bump_winner = bump;
        }
        Ok(())
    }
    pub fn send_amount_to_winner(
        ctx: Context<SendAmountToWinner>
    ) -> Result<()> {
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        let winner: &mut AccountInfo = &mut ctx.accounts.winner;
        let creator_publickey: &mut AccountInfo = &mut ctx.accounts.creator_publickey;
        if solotery.choose_winner_only_one_time == 0 {
            return Err(ErrorCode::JustOnce.into());
        }
        if solotery.winner == 0 {
            return Err(ErrorCode::NoWinner.into());
        }
        if winner.key() != solotery.winner_publickey {
            return Err(ErrorCode::ThisIsNotTheWinner.into());
        }
        if ctx.accounts.winner_publickey.key() != ctx.accounts.ticket.authority.key() {
            return Err(ErrorCode::ThisIsNotTheWinner.into());
        }
        if ctx.accounts.solotery_authority.key() != Pubkey::from_str("9RDz3M796x25qXfVGUSau3rze3WH4z8KesZe3MMBYfrZ").unwrap() {
            return Err(ErrorCode::YouAreNotSOLotery.into());
        }
        if Clock::get().unwrap().unix_timestamp < solotery.secure_check {
            return Err(ErrorCode::IncorrectTimestamp.into());
        }
        fn to_f64(amount: u64) -> f64 {return amount as f64}
        fn percent(amount: f64) -> u64 {((amount / 100.0)* 2.0).round() as u64}  
        let fee_creator: u64 = percent(to_f64(AccountInfo::lamports(&solotery.to_account_info()))); 
        let winner_reward: u64 = AccountInfo::lamports(&solotery.to_account_info()) 
        - 1712170
        - fee_creator; 
        **solotery.to_account_info().try_borrow_mut_lamports()? -= fee_creator;
        **creator_publickey.try_borrow_mut_lamports()? += fee_creator;
        **solotery.to_account_info().try_borrow_mut_lamports()? -= winner_reward;
        **winner.to_account_info().try_borrow_mut_lamports()? += winner_reward;
        solotery.choose_winner_only_one_time = solotery.choose_winner_only_one_time - 1;
        solotery.winner = 0;   
        solotery.american_stake = (solotery.american_stake + 1) - solotery.american_stake;
        solotery.winner_publickey = Pubkey::from_str("11111111111111111111111111111111").unwrap();
        solotery.bump_winner = 0;
        solotery.lifetime255 += 1;
        solotery.secure_check += 86398;
        Ok(())
    }
    pub fn check_it(
        _ctx: Context<CheckIt>
    ) -> Result<()> {
        Ok(())
    }
    pub fn check_winner(
        _ctx: Context<CheckWinner>
    ) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, seeds = [b"SOLotery", user.key().as_ref()], bump, payer = user, space = 118)]
    pub solotery: Account<'info, SoLotery>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct AmericanTicket<'info> {
    #[account(mut, seeds = [b"SOLotery", solotery.authority.key().as_ref()], bump = solotery.bump_original)]
    pub solotery: Account<'info, SoLotery>,
    #[account(init, seeds = [solotery.lifetime255.to_le_bytes().as_ref(), solotery.american_stake.to_le_bytes().as_ref()], bump, payer = from, 
    space = 8 + 32)]
    pub ticket: Account<'info, TicketStruct>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub stake: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Winner<'info> {
    #[account(mut, seeds = [b"SOLotery", solotery.authority.key().as_ref()], bump = solotery.bump_original)]
    pub solotery: Account<'info, SoLotery>,
    #[account(mut)]
    pub solotery_authority: Signer<'info>
}
#[derive(Accounts)]
pub struct SendAmountToWinner<'info> {
    #[account(mut, seeds = [b"SOLotery", solotery.authority.key().as_ref()], bump = solotery.bump_original)]
    pub solotery: Account<'info, SoLotery>,
    #[account(mut, seeds = [solotery.lifetime255.to_le_bytes().as_ref(), solotery.winner.to_le_bytes().as_ref()], bump = solotery.bump_winner)]
    pub ticket: Account<'info, TicketStruct>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub creator_publickey: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub winner: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this accoun
    #[account(mut)]
    pub winner_publickey: AccountInfo<'info>,
    #[account(mut)]
    pub solotery_authority: Signer<'info>,
}
#[derive(Accounts)]
pub struct CheckIt<'info> {
    #[account(mut, seeds = [b"SOLotery", solotery.authority.key().as_ref()], bump = solotery.bump_original)]
    pub solotery: Account<'info, SoLotery>,
    #[account(mut)]
    pub user: Signer<'info>,
}
#[derive(Accounts)]
pub struct CheckWinner<'info> {
    #[account(mut, seeds = [b"SOLotery", solotery.authority.key().as_ref()], bump = solotery.bump_original)]
    pub solotery: Account<'info, SoLotery>,
    #[account(mut, seeds = [solotery.lifetime255.to_le_bytes().as_ref(), solotery.winner.to_le_bytes().as_ref()], bump = solotery.bump_winner)]
    pub ticket: Account<'info, TicketStruct>,
    #[account(mut)]
    pub user: Signer<'info>,
}
#[account]
pub struct SoLotery {
    pub authority: Pubkey, 
    pub stake_pda: Pubkey,
    pub bump_original: u8, 
    pub bump_winner: u8, 
    pub american_stake: u8, 
    pub winner: u8, 
    pub winner_publickey: Pubkey, 
    pub choose_winner_only_one_time: u8, 
    pub secure_check: i64, 
    pub lifetime255: u8
}
#[account]
pub struct TicketStruct {
    pub authority: Pubkey
}
#[error_code]
pub enum ErrorCode {
    #[msg("The winner can only be chosen once")]JustOnce, #[msg("You are not SOLotery key")]YouAreNotSOLotery, 
    #[msg("This is not the winner")]ThisIsNotTheWinner, #[msg("This is not the stake account")]WrongStake, 
    #[msg("No winner has been chosen")]NoWinner, #[msg("The player limit is 254")]Limit,
    #[msg("Restart the program, solotery bump reached its limit")]RestartTheProgram, #[msg("19:00 Argentine time")]IncorrectTimestamp, 
}
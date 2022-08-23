use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::system_instruction,
    solana_program::pubkey::Pubkey,
}; 
use std::str::FromStr;
use oorandom; 

declare_id!("AtXQU8WHCw5Y2QXXSoQhXBGAA5Zynu4QGKupRGNv9srF");

#[program]
pub mod solotery {
    use super::*;

    pub fn create_stake(
        ctx: Context<Create>,
        bump: u8
    ) -> Result<()> {
        let signer_key: Pubkey = ctx.accounts.user.key();
        let correct_payer: Pubkey = Pubkey::from_str("7ux4EQ186kzjYxFRPKRLSmNaiYbFK5xK55w6hxzaD7h8").unwrap();
        if signer_key != correct_payer {
            return Err(ErrorCode::YouAreNotSOLotery.into());
        }
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        solotery.authority = ctx.accounts.user.key();
        solotery.seed = 1;
        solotery.american_stake = 1;
        solotery.choose_winner_only_one_time = 0;
        solotery.bump_original = bump;
        solotery.secure_check = 1661119200;
        solotery.day = "22/08/2022".to_string();
        solotery.winner = 0;   
        Ok(())
    }
    pub fn ticket(
        ctx: Context<AmericanTicket>,
        modify_the_seed: u64,
    ) -> Result<()> {
        let program_id: Pubkey = Pubkey::from_str("7Y4eL4xckuRQSPoGGPKSG3Ghbg4LfmNcMFp6XkVFJiFJ").unwrap();
        let from: &anchor_lang::prelude::AccountInfo<'_> = &ctx.accounts.from;
        let stake: &anchor_lang::prelude::AccountInfo<'_> = &mut ctx.accounts.stake;
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        solotery.seed += modify_the_seed;
        if solotery.american_stake == 254 {
            return Err(ErrorCode::Limit.into());
        }
        solotery.american_stake += 1;
        let total: u64 = 33333333;
        let (stake_account, _bump) = Pubkey::find_program_address(&[b"SOLotery", solotery.authority.key().as_ref()], &program_id);
        if stake.key() != stake_account {
            return Err(ErrorCode::WrongStake.into());
        }
        anchor_lang::solana_program::program::invoke(
            &system_instruction::transfer(&ctx.accounts.from.key(), &stake_account, total),
            &[from.to_account_info(), stake.to_account_info().clone()],).expect("Error");
        Ok(())
    }
    pub fn choose_winner(
        ctx: Context<Winner>,
    ) -> Result<()> {
        //let clock: Clock = Clock::get().unwrap();
        let solotery_authority: Pubkey = ctx.accounts.solotery_authority.key();
        let correct_payer: Pubkey = Pubkey::from_str("7ux4EQ186kzjYxFRPKRLSmNaiYbFK5xK55w6hxzaD7h8").unwrap();
        let program_id: Pubkey = Pubkey::from_str("7Y4eL4xckuRQSPoGGPKSG3Ghbg4LfmNcMFp6XkVFJiFJ").unwrap();
        if solotery_authority != correct_payer {
            return Err(ErrorCode::YouAreNotSOLotery.into());
        }
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        if solotery.choose_winner_only_one_time == 1 {
            return Err(ErrorCode::JustOnce.into());
        }
        /*if clock.unix_timestamp < solotery.secure_check {
            return Err(ErrorCode::IncorrectTimestamp.into());
        }*/
        solotery.choose_winner_only_one_time += 1;
        if solotery.american_stake == 1 {
            //solotery.secure_check += 86400;
            solotery.choose_winner_only_one_time -= 1;
        }
        if solotery.american_stake > 1 {
            let plusone: u64 = (solotery.american_stake + 1).try_into().unwrap();
            let mut rng: oorandom::Rand64 = oorandom::Rand64::new(solotery.seed.into());
            let winner: usize = rng.rand_range(1..plusone).try_into().unwrap();
            solotery.winner =  winner as u8;
            let (winner, _bump) = Pubkey::find_program_address(&[solotery.day.as_bytes().as_ref(), solotery.winner.to_le_bytes().as_ref()], &program_id);
            solotery.winner_publickey = winner;
        }
        Ok(())
    }
    pub fn send_amount_to_winner(
        ctx: Context<SendAmountToWinner>,
    ) -> Result<()> {
        //let clock: Clock = Clock::get().unwrap();
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        let winner: &mut AccountInfo = &mut ctx.accounts.winner;
        let creator_publickey: &mut AccountInfo = &mut ctx.accounts.creator_publickey;
        let solotery_authority: Pubkey = ctx.accounts.solotery_authority.key();
        let correct_payer: Pubkey = Pubkey::from_str("7ux4EQ186kzjYxFRPKRLSmNaiYbFK5xK55w6hxzaD7h8").unwrap();
        let system_program: Pubkey = Pubkey::from_str("11111111111111111111111111111111").unwrap();
        if solotery.choose_winner_only_one_time == 0 {
            return Err(ErrorCode::JustOnce.into());
        }
        if solotery.winner == 0 {
            return Err(ErrorCode::NoWinner.into());
        }
        if winner.key() != solotery.winner_publickey.key() {
            return Err(ErrorCode::ThisIsNotTheWinner.into());
        }
        if solotery_authority != correct_payer {
            return Err(ErrorCode::YouAreNotSOLotery.into());
        }
        /*if clock.unix_timestamp < solotery.secure_check {
            return Err(ErrorCode::IncorrectTimestamp.into());
        }*/
        fn to_f64(amount: u64) -> f64 {return amount as f64}
        fn percent(amount: f64) -> u64 {((amount / 100.0)* 2.0).round() as u64}  
        let fee_creator: u64 = percent(to_f64(AccountInfo::lamports(&solotery.to_account_info()))); 
        let winner_reward: u64 = AccountInfo::lamports(&solotery.to_account_info()) 
        - 68444640
        - fee_creator; 
        **solotery.to_account_info().try_borrow_mut_lamports()? -= fee_creator;
        **creator_publickey.try_borrow_mut_lamports()? += fee_creator;
        **solotery.to_account_info().try_borrow_mut_lamports()? -= winner_reward;
        **winner.to_account_info().try_borrow_mut_lamports()? += winner_reward;
        solotery.seed = (solotery.seed + 1) - solotery.seed;
        solotery.choose_winner_only_one_time = solotery.choose_winner_only_one_time - 1;
        solotery.winner = 0;   
        solotery.american_stake = (solotery.american_stake + 1) - solotery.american_stake;
        solotery.winner_publickey = system_program;
        //solotery.secure_check += 86400;
        Ok(())
    }
    pub fn check_it(
        _ctx: Context<CheckIt>,
    ) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, seeds = [b"SOLotery", user.key().as_ref()], bump, payer = user, 
    space = 8 + 32 + 1 + 1 + 8 + 1 + 8)]
    pub solotery: Account<'info, SoLotery>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct AmericanTicket<'info> {
    #[account(mut, seeds = [b"SOLotery", solotery.authority.key().as_ref()], bump = solotery.bump_original)]
    pub solotery: Account<'info, SoLotery>,
    #[account(init, seeds = [solotery.day.as_bytes().as_ref(), solotery.american_stake.to_le_bytes().as_ref()], bump, payer = from, 
    space = 8 + 32 + 1 + 1 + 8 + 1 + 8 + 4 + 10)]
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
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub creator_publickey: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub winner: AccountInfo<'info>,
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
#[account]
pub struct SoLotery {
    pub authority: Pubkey,
    pub bump_original: u8,
    pub american_stake: u8,
    pub winner: u8,
    pub winner_publickey: Pubkey,
    pub seed: u64,
    pub choose_winner_only_one_time: u8,
    pub secure_check: i64,
    pub day: String
}
#[account]
pub struct TicketStruct {
    pub authority: Pubkey,
    pub bump: u8,
}
#[error_code]
pub enum ErrorCode {
    #[msg("The winner can only be chosen once")]JustOnce, #[msg("You are not SOLotery key")]YouAreNotSOLotery, 
    #[msg("This is not the winner")]ThisIsNotTheWinner, #[msg("This is not the stake account")]WrongStake, 
    #[msg("No winner has been chosen")]NoWinner, #[msg("The player limit is 255")]Limit
}
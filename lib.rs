use anchor_lang::{prelude::*,
solana_program::account_info::AccountInfo}; 
use std::str::FromStr;
use oorandom; 
declare_id!("AJNEUGzr2s6aDizpjyMbQPUuY3psL49nWsfn34GeE7NJ");
#[program]
pub mod so_lotery_source {
    use super::*;
    pub fn create_stake(
        ctx: Context<Create>,
        bump: u8
    ) -> Result<()> {
        let signer_key: Pubkey = ctx.accounts.user.key();
        let correct_payer: Pubkey = Pubkey::from_str("AbQWyJxGzmxC51t4EjYCg4b3rhS5sCUB4BHKMZWLcKdZ").unwrap();
        if signer_key != correct_payer {
            return Err(ErrorCode::YouAreNotSOLotery.into());
        }
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        solotery.authority = ctx.accounts.user.key();
        solotery.seed = 193;
        solotery.players = vec![];
        solotery.choose_winner_only_one_time = 0;
        solotery.bump_original = bump;
        Ok(())
    }
    pub fn ticket(
        ctx: Context<Ticket>,
        modify_the_seed: u64,
    ) -> Result<()> {
        let from: &anchor_lang::prelude::AccountInfo<'_> = &ctx.accounts.from;
        let stake: &anchor_lang::prelude::AccountInfo<'_> = &mut ctx.accounts.stake;
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        let v: &mut Vec<anchor_lang::prelude::Pubkey> = &mut solotery.players; v.push(ctx.accounts.from.key());
        solotery.seed += modify_the_seed;
        let total: u64 = 33333333;
        let stake_account: Pubkey = Pubkey::from_str("2JmcQz8gF8yqikS48jk7fpzrNpNUiLteuUA2vPkaLMUN").unwrap();
        let transfer: anchor_lang::solana_program::instruction::Instruction = 
        anchor_lang::solana_program::system_instruction::transfer(&ctx.accounts.from.key(), &stake_account, total);
        anchor_lang::solana_program::program::invoke(
            &transfer,
            &[from.to_account_info(), stake.to_account_info().clone()],).expect("Error");
        Ok(())
    }
    pub fn choose_winner(
        ctx: Context<Winner>,
    ) -> Result<()> {
        let signer_key: Pubkey = ctx.accounts.authority_info.key();
        let correct_payer: Pubkey = Pubkey::from_str("AbQWyJxGzmxC51t4EjYCg4b3rhS5sCUB4BHKMZWLcKdZ").unwrap();
        if signer_key != correct_payer {
            return Err(ErrorCode::YouAreNotSOLotery.into());
        }
        let solotery:&mut Account<SoLotery> = &mut ctx.accounts.solotery;
        if solotery.players.len() == 0 {
            return Err(ErrorCode::NoPlayers.into());
        }
        if solotery.choose_winner_only_one_time == 1 {
            return Err(ErrorCode::JustOnce.into());
        }
        solotery.choose_winner_only_one_time += 1;
        let plusone:u64 = (solotery.players.len() + 1).try_into().unwrap();
        let mut rng = oorandom::Rand64::new(solotery.seed.into());
        let winner: usize = rng.rand_range(0..plusone).try_into().unwrap();
        solotery.winner =  solotery.players[winner];
        Ok(())
    }
    pub fn send_amount_to_winner(
        ctx: Context<SendAmountToWinner>,
    ) -> Result<()> {
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        let winner: &mut AccountInfo = &mut ctx.accounts.winner;
        let creator_publickey: &mut AccountInfo = &mut ctx.accounts.creator_publickey;
        let signer_key: Pubkey = ctx.accounts.user.key();
        let correct_payer: Pubkey = Pubkey::from_str("AbQWyJxGzmxC51t4EjYCg4b3rhS5sCUB4BHKMZWLcKdZ").unwrap();
        if signer_key != correct_payer {
            return Err(ErrorCode::YouAreNotSOLotery.into());
        }
        if winner.key() != solotery.winner.key() {
            return Err(ErrorCode::ThisIsNotTheWinner.into());
        }
        fn to_f64(amount: u64) -> f64 {return amount as f64}
        fn percent(amount: f64) -> u64 {((amount / 100.0)* 2.0).round() as u64}  
        let fee_creator: u64 = percent(to_f64(AccountInfo::lamports(&solotery.to_account_info()))); 
        let winner_reward: u64 = AccountInfo::lamports(&solotery.to_account_info()) 
        - 68340240 
        - fee_creator; 
        **solotery.to_account_info().try_borrow_mut_lamports()? -= fee_creator;
        **creator_publickey.try_borrow_mut_lamports()? += fee_creator;
        **solotery.to_account_info().try_borrow_mut_lamports()? -= winner_reward;
        **winner.to_account_info().try_borrow_mut_lamports()? += winner_reward;
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
    #[account(init, seeds = [b"SOLotery", user.key().as_ref()], bump, payer = user, space = 9690)]
    pub solotery: Account<'info, SoLotery>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Ticket<'info> {
    #[account(mut, seeds = [b"SOLotery", from.key().as_ref()], bump = solotery.bump_original)]
    pub solotery: Account<'info, SoLotery>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
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
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
    pub authority_info: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
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
    pub user: Signer<'info>,
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
    pub players: Vec<Pubkey>,
    pub seed: u64,
    pub winner: Pubkey,
    pub choose_winner_only_one_time: u8,
}
#[error_code]
pub enum ErrorCode {
    #[msg("The winner can only be chosen once")]JustOnce, #[msg("There are no tickets at stake")]NoPlayers,
    #[msg("You are not SOLotery key")]YouAreNotSOLotery, #[msg("This is not the winner")]ThisIsNotTheWinner,
}
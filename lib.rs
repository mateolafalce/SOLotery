use anchor_lang::{prelude::*,
solana_program::account_info::AccountInfo
};
use oorandom; 
declare_id!("8FL8ZJpy1FjxyeUxFVjMcnZQpFDebzNP7Efh5SqcmyCA");
#[program]
pub mod so_lotery_source {
    use super::*;
    pub fn create_stake(
        ctx: Context<Create>,
        authority: Pubkey,
    ) -> Result<()> {
        let solotery:&mut Account<SoLotery> = &mut ctx.accounts.solotery;
        solotery.authority = authority;
        solotery.players = vec![];
        solotery.seed = 19963;
        solotery.choose_winner_only_one_time = 0;
        Ok(())
    }
    pub fn ticket(
        ctx: Context<Ticket>,
        pubkey: Pubkey,
    ) -> Result<()> {
        let from = &mut ctx.accounts.from;let stake = &mut ctx.accounts.stake;
        let solotery:&mut Account<SoLotery> = &mut ctx.accounts.solotery;
        if solotery.players.len() > 100 {
            return Err(ErrorCode::LimitPlayers.into());
        }
        let v = &mut solotery.players; v.push(pubkey);
        solotery.seed += 7;
        let total: u64 = 7777777;
        let stake_account: [u8; 32] = [
        67,69,236,90,175,187,119,37,177,53,210,48,31,195,115,85,58,99,172,69,228,69,208,147,126,131,154,123,131,89,119,152
        ];
        let wallet = anchor_lang::prelude::Pubkey::new_from_array(stake_account);
        let transfer = anchor_lang::solana_program::system_instruction::transfer(
            &pubkey, &wallet, total,
        );
        anchor_lang::solana_program::program::invoke(
            &transfer,
            &[from.to_account_info(), stake.to_account_info().clone()],
        ).expect("Error");
        Ok(())
    }
    pub fn choose_winner(
        ctx: Context<Winner>,
    ) -> Result<()> {
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
        stake: Pubkey,
    ) -> Result<()> {
        let key: [u8; 32] = [
            248,  78, 254, 126, 183, 33, 110, 247,
            102, 124,  26, 146, 187, 63,  71,  22,
             63,  62,  29, 147, 193, 13,   1, 198,
            236, 238, 117, 213,  59, 31,  57, 245
        ];
        let creator_key = anchor_lang::prelude::Pubkey::new_from_array(key);
        fn to_f64(amount: u64) -> f64 {return amount as f64}
        fn one_percent(amount: f64) -> u64 {((amount / 100.0)* 2.0).round() as u64}  
        let from: &mut AccountInfo = &mut ctx.accounts.from;
        let winner: &mut AccountInfo = &mut ctx.accounts.winner;
        let creator_publickey: &mut AccountInfo = &mut ctx.accounts.creator_publickey;
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        let fee_creator: u64 = one_percent(to_f64(AccountInfo::lamports(from))); 
        let winner_reward: u64 = AccountInfo::lamports(from) 
        - 890890 //Rent-exempt minimum
        - fee_creator; //Fee creator
        let creator_fee = anchor_lang::solana_program::system_instruction::transfer(
            &stake, &creator_key, fee_creator,
        );
        anchor_lang::solana_program::program::invoke(
            &creator_fee,
            &[from.to_account_info(), creator_publickey.to_account_info().clone()],
        ).expect("Error");
        let transfer_winner = anchor_lang::solana_program::system_instruction::transfer(
            &stake, &solotery.winner, winner_reward,
        );
        anchor_lang::solana_program::program::invoke(
            &transfer_winner,
            &[from.to_account_info(), winner.to_account_info().clone()],
        ).expect("Error");
        Ok(())
    }
    pub fn send_amount_to_fee_zero_winner(
        ctx: Context<SendAmountToWinner>,
        stake: Pubkey,
    ) -> Result<()> {
        let from: &mut AccountInfo = &mut ctx.accounts.from;
        let winner: &mut AccountInfo = &mut ctx.accounts.winner;
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        let winner_reward: u64 = AccountInfo::lamports(from) 
        - 890890; //Rent-exempt minimum
        let transfer_winner = anchor_lang::solana_program::system_instruction::transfer(
            &stake, &solotery.winner, winner_reward,
        );
        anchor_lang::solana_program::program::invoke(
            &transfer_winner,
            &[from.to_account_info(), winner.to_account_info().clone()],
        ).expect("Error");
        Ok(())
    }
    pub fn check_it(
        _ctx: Context<CheckIt>,
    ) -> Result<()> {
        Ok(())
    }
    pub fn delete(
        _ctx: Context<Delete>
    ) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = SoLotery::LEN)]
    pub solotery: Account<'info, SoLotery>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Ticket<'info> {
    #[account(mut)]
    pub solotery: Account<'info, SoLotery>,
    #[account(mut)]
    pub user: Signer<'info>,
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
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, has_one = authority)]
    pub solotery: Account<'info, SoLotery>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct SendAmountToWinner<'info> {
    #[account(mut)]
    pub solotery: Account<'info, SoLotery>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub creator_publickey: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub winner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Delete<'info> {
    #[account(mut, has_one = authority, close = authority)]
    pub solotery: Account<'info, SoLotery>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct CheckIt<'info> {
    #[account(mut)]
    pub solotery: Account<'info, SoLotery>,
    #[account(mut)]
    pub user: Signer<'info>,
}
#[account]
pub struct SoLotery {
    pub authority: Pubkey,
    pub players: Vec<Pubkey>,
    pub seed: u64,
    pub winner: Pubkey,
    pub choose_winner_only_one_time: u8,
}
impl SoLotery {
    const LEN: usize = DISCRIMINATOR
    + FIXED 
    + PUBKEY
    + SEED
    + WINNER
    + SECURE_CHECK;
}
const DISCRIMINATOR: usize = 8;
const PUBKEY: usize = 9600; //32 bytes * 300 players
const FIXED: usize = 4;
const SEED: usize = 8;
const WINNER: usize = 32;
const SECURE_CHECK: usize = 1;
#[error_code]
pub enum ErrorCode {
    #[msg("The winner can only be chosen once")]
    JustOnce,
    #[msg("Player limit is 100")]
    LimitPlayers,
    #[msg("There are no tickets at stake")]
    NoPlayers,
}

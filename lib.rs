use anchor_lang::{prelude::*,
solana_program::account_info::AccountInfo
};use core::mem::size_of; use std::str::FromStr;
use oorandom; 
declare_id!("FkxCCZQC9GSHEN5BUtYgAj5sbjSZvdmJRMzsUnxqky9F");
#[program]
pub mod so_lotery_source {
    use super::*;
    pub fn create_stake(
        ctx: Context<Create>,
        authority: Pubkey,
        bump: u8,
    ) -> Result<()> {
        let solotery:&mut Account<SoLotery> = &mut ctx.accounts.solotery;
        solotery.authority = ctx.accounts.user.key();
        //solotery.seed: u64 = 19963;
        solotery.players = 1;
        solotery.choose_winner_only_one_time = 0;
        Ok(())
    }
    pub fn ticket(
        ctx: Context<Ticket>,
        pubkey: Pubkey,
    ) -> Result<()> {
        let from = &mut ctx.accounts.from;let stake = &mut ctx.accounts.stake;
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        solotery.players += 1;
        //solotery.seed += 7;
        let total: u64 = 7777777;
        let stake_account: [u8; 32] = [67,69,236,90,175,187,119,37,177,53,210,48,31,195,115,85,58,99,172,69,228,69,208,147,126,131,154,123,131,89,119,152];
        let wallet: Pubkey = anchor_lang::prelude::Pubkey::new_from_array(stake_account);
        let transfer: anchor_lang::solana_program::instruction::Instruction = 
        anchor_lang::solana_program::system_instruction::transfer(&pubkey, &wallet, total,);
        anchor_lang::solana_program::program::invoke(
            &transfer,
            &[from.to_account_info(), stake.to_account_info().clone()],).expect("Error");
        Ok(())
    }
    pub fn choose_winner(
        ctx: Context<Winner>,
    ) -> Result<()> {
        
        let solotery:&mut Account<SoLotery> = &mut ctx.accounts.solotery;
        if solotery.players == 1 {
            return Err(ErrorCode::NoPlayers.into());
        }
        if solotery.choose_winner_only_one_time == 1 {
            return Err(ErrorCode::JustOnce.into());
        }
        let program_id: Pubkey = Pubkey::from_str("FkxCCZQC9GSHEN5BUtYgAj5sbjSZvdmJRMzsUnxqky9F").unwrap();
        solotery.choose_winner_only_one_time += 1;
        let plusone:u64 = (solotery.players + 1).try_into().unwrap();
        let mut rng = oorandom::Rand64::new(solotery.seed.into());
        let winner: usize = rng.rand_range(0..plusone).try_into().unwrap();
        let (pubkey_winner, bump_seed): (Pubkey, u8) = anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
            &[b"16/8/2022", winner.to_le_bytes().as_ref()],
            &program_id
        );
        solotery.bumpwinner = bump_seed;
        Ok(())
    }
    pub fn send_amount_to_winner(
        ctx: Context<SendAmountToWinner>,
    ) -> Result<()> {
        let rawkey: [u8; 32] = [248,  78, 254, 126, 183, 33, 110, 247,102, 124,  26, 146, 187, 63,  71,  22, 63,  62,  29, 147, 193, 13,   1, 198,236, 238, 117, 213,  59, 31,  57, 245];
        let creator_key: Pubkey = anchor_lang::prelude::Pubkey::new_from_array(rawkey);
        fn to_f64(amount: u64) -> f64 {return amount as f64}
        fn percent(amount: f64) -> u64 {((amount / 100.0)* 2.0).round() as u64}  
        let from: &mut AccountInfo = &mut ctx.accounts.from;
        let winner: &mut AccountInfo = &mut ctx.accounts.winner;
        let creator_publickey: &mut AccountInfo = &mut ctx.accounts.creator_publickey;
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        let program_id: Pubkey = Pubkey::from_str("FkxCCZQC9GSHEN5BUtYgAj5sbjSZvdmJRMzsUnxqky9F").unwrap();
        let (solotery_account, bump_seed): (Pubkey, u8) = anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
            &[b"SOLotery", solotery.authority.key().as_ref()],
            &program_id
        );
        let fee_creator: u64 = percent(to_f64(AccountInfo::lamports(from))); 
        let winner_reward: u64 = AccountInfo::lamports(from) 
        - 890890 //Rent-exempt minimum
        - fee_creator; //Fee creator

        Ok(())
    }
    
}
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, seeds = [b"SOLotery", from.key().as_ref()], bump, payer = user, 
    space = 8 + 1 + 32 + 8 + 8 + 32 + 1 + 1)]
    pub solotery: Account<'info, SoLotery>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: dick
    pub from: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct Ticket<'info> {
    #[account(seeds = [b"SOLotery", solotery.authority.key().as_ref()], bump = solotery.bump_original)]
    pub solotery: Account<'info, SoLotery>,
    #[account(init, payer = user, space = 8 + 32, seeds = [b"16/8/2022", solotery.players.to_le_bytes().as_ref()], bump)]
    pub ticket_data: Account<'info, PlayerPDA>,
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
    #[account(mut, seeds = [b"SOLotery", solotery.authority.key().as_ref()], bump = solotery.bump_original)]
    pub solotery: Account<'info, SoLotery>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct SendAmountToWinner<'info> {
    #[account(mut, seeds = [b"SOLotery", solotery.authority.key().as_ref()], bump = solotery.bump_original)]
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
    #[account(mut, payer = solotery.authority close = solotery.authority)]
    pub solotery: Account<'info, SoLotery>,
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
    pub bump_original: u8,
    pub players: u64,
    pub seed: u64,
    pub winner: Pubkey,
    pub choose_winner_only_one_time: u8,
    pub bumpwinner: u8,
}
#[account]
pub struct PlayerPDA {
    pub user: Pubkey,
}
#[error_code]
pub enum ErrorCode {
    #[msg("The winner can only be chosen once")]
    JustOnce,
    #[msg("There are no tickets at stake")]
    NoPlayers,
}

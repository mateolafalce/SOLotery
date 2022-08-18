use anchor_lang::{prelude::*,
solana_program::account_info::AccountInfo
};use core::mem::size_of; use std::str::FromStr;
use oorandom; 
declare_id!("7uzUgWB8BUQigpMTxiDhKtaru5MvziRtiM1BDFn3NHLe");
#[program]
pub mod so_lotery_source {
    use super::*;
    pub fn create_stake(
        ctx: Context<Create>,
    ) -> Result<()> {
        let signer_key: Pubkey = ctx.accounts.user.key();
        let correct_payer: Pubkey = Pubkey::from_str("AbQWyJxGzmxC51t4EjYCg4b3rhS5sCUB4BHKMZWLcKdZ").unwrap();
        if signer_key != correct_payer {
            return Err(ErrorCode::YouAreNotSOLotery.into());
        }
        let solotery:&mut Account<SoLotery> = &mut ctx.accounts.solotery;
        solotery.authority = ctx.accounts.user.key();
        solotery.seed = 193;
        solotery.players = 1;
        solotery.choose_winner_only_one_time = 0;
        solotery.bump_original = *ctx.bumps.get("user_stats").unwrap();
        Ok(())
    }
    pub fn ticket(
        ctx: Context<Ticket>,
        modify_the_seed: u64,
    ) -> Result<()> {
        let from: &anchor_lang::prelude::AccountInfo<'_> = &ctx.accounts.from;
        let stake: &anchor_lang::prelude::AccountInfo<'_> = &mut ctx.accounts.stake;
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        let pubkey: Pubkey = ctx.accounts.user.key();
        let player_ticket: &mut Pubkey = &mut ctx.accounts.ticket_data.user;
        *player_ticket = pubkey;
        solotery.players += 1;
        solotery.seed += modify_the_seed;
        let total: u64 = 7777777;
        let stake_account: [u8; 32] = [67,69,236,90,175,187,119,37,177,53,210,48,31,195,115,85,58,99,172,69,228,69,208,147,126,131,154,123,131,89,119,152];
        let wallet: Pubkey = anchor_lang::prelude::Pubkey::new_from_array(stake_account);
        let transfer: anchor_lang::solana_program::instruction::Instruction = 
        anchor_lang::solana_program::system_instruction::transfer(&pubkey, &wallet, total);
        anchor_lang::solana_program::program::invoke(
            &transfer,
            &[from.to_account_info(), stake.to_account_info().clone()],).expect("Error");
        Ok(())
    }
    pub fn choose_winner(
        ctx: Context<Winner>,
    ) -> Result<()> {
        let secure_check: &mut Account<SecureCheck> = &mut ctx.accounts.secure_check;
        let signer_key: Pubkey = ctx.accounts.authority.key();
        let correct_payer: Pubkey = Pubkey::from_str("AbQWyJxGzmxC51t4EjYCg4b3rhS5sCUB4BHKMZWLcKdZ").unwrap();
        secure_check.authority = correct_payer;
        if signer_key != correct_payer {
            return Err(ErrorCode::YouAreNotSOLotery.into());
        }
        let solotery:&mut Account<SoLotery> = &mut ctx.accounts.solotery;
        if solotery.players == 1 {
            return Err(ErrorCode::NoPlayers.into());
        }
        if solotery.choose_winner_only_one_time == 1 {
            return Err(ErrorCode::JustOnce.into());
        }
        let program_id: Pubkey = Pubkey::from_str("7uzUgWB8BUQigpMTxiDhKtaru5MvziRtiM1BDFn3NHLe").unwrap();
        solotery.choose_winner_only_one_time += 1;
        let plusone:u64 = (solotery.players + 1).try_into().unwrap();
        let mut rng = oorandom::Rand64::new(solotery.seed.into());
        let winner: usize = rng.rand_range(0..plusone).try_into().unwrap();
        let (pubkey_winner, bump_seed): (Pubkey, u8) = anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
            &[b"17/8/2022", winner.to_le_bytes().as_ref()],
            &program_id
        );
        solotery.bumpwinner = bump_seed;
        Ok(())
    }
    pub fn send_amount_to_winner(
        ctx: Context<SendAmountToWinner>,
    ) -> Result<()> {
        let secure_check: &mut Account<SecureCheck> = &mut ctx.accounts.secure_check;
        let signer_key: Pubkey = ctx.accounts.authority.key();
        let correct_payer: Pubkey = Pubkey::from_str("AbQWyJxGzmxC51t4EjYCg4b3rhS5sCUB4BHKMZWLcKdZ").unwrap();
        secure_check.authority = correct_payer;
        if signer_key != correct_payer {
            return Err(ErrorCode::YouAreNotSOLotery.into());
        }
        let rawkey: [u8; 32] = [248,  78, 254, 126, 183, 33, 110, 247,102, 124,  26, 146, 187, 63,  71,  22, 63,  62,  29, 147, 193, 13,   1, 198,236, 238, 117, 213,  59, 31,  57, 245];
        let creator_key: Pubkey = anchor_lang::prelude::Pubkey::new_from_array(rawkey);
        fn to_f64(amount: u64) -> f64 {return amount as f64}
        fn percent(amount: f64) -> u64 {((amount / 100.0)* 2.0).round() as u64}  
        let from: &mut AccountInfo = &mut ctx.accounts.from;
        let winner: &mut AccountInfo = &mut ctx.accounts.winner;
        let creator_publickey: &mut AccountInfo = &mut ctx.accounts.creator_publickey;
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        let program_id: Pubkey = Pubkey::from_str("7uzUgWB8BUQigpMTxiDhKtaru5MvziRtiM1BDFn3NHLe").unwrap();
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
    pub fn check_it(
        _ctx: Context<CheckIt>,
    ) -> Result<()> {
        Ok(())
    }
    pub fn delete(
        ctx: Context<Delete>
    ) -> Result<()> {
        let secure_check: &mut Account<SecureCheck> = &mut ctx.accounts.secure_check;
        let signer_key: Pubkey = ctx.accounts.authority.key();
        let correct_payer: Pubkey = Pubkey::from_str("AbQWyJxGzmxC51t4EjYCg4b3rhS5sCUB4BHKMZWLcKdZ").unwrap();
        secure_check.authority = correct_payer;
        if signer_key != correct_payer {
            return Err(ErrorCode::YouAreNotSOLotery.into());
        }
        Ok(())
    }
    pub fn delete_solotery_pda(
        ctx: Context<DeleteSOLoteryPDA>
    ) -> Result<()> {
        let secure_check: &mut Account<SecureCheck> = &mut ctx.accounts.secure_check;
        let signer_key: Pubkey = ctx.accounts.authority.key();
        let correct_payer: Pubkey = Pubkey::from_str("AbQWyJxGzmxC51t4EjYCg4b3rhS5sCUB4BHKMZWLcKdZ").unwrap();
        secure_check.authority = correct_payer;
        if signer_key != correct_payer {
            return Err(ErrorCode::YouAreNotSOLotery.into());
        }
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        let program_id: Pubkey = Pubkey::from_str("7uzUgWB8BUQigpMTxiDhKtaru5MvziRtiM1BDFn3NHLe").unwrap();
        secure_check.authority = correct_payer;
        let (pubkey_program, bump_seed): (Pubkey, u8) = anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
            &[b"SOLotery", solotery.authority.key().as_ref(), solotery.bump_original.to_le_bytes().as_ref()],
            &program_id
        );
        anchor_lang::solana_program::bpf_loader_upgradeable::close(&pubkey_program, &solotery.authority, &program_id);
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
    #[account(mut, seeds = [b"SOLotery", user.key().as_ref()], bump = solotery.bump_original)]
    pub solotery: Account<'info, SoLotery>,
    #[account(init, payer = user, space = 8 + 32, seeds = [b"17/8/2022", solotery.players.to_le_bytes().as_ref()], bump)]
    pub ticket_data: Account<'info, PlayerPDA>,
    #[account(mut)]
    pub user: Signer<'info>,
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
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, seeds = [b"SOLotery", solotery.authority.key().as_ref()], bump = solotery.bump_original)]
    pub solotery: Account<'info, SoLotery>,
    #[account(init, payer = authority, space = 8)]
    pub secure_check: Account<'info, SecureCheck>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    //#[account(signer)]
    #[account(mut)]
    pub authority_info: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub authority: Signer<'info>,
}
#[derive(Accounts)]
pub struct SendAmountToWinner<'info> {
    #[account(mut, seeds = [b"SOLotery", solotery.authority.key().as_ref()], bump = solotery.bump_original)]
    pub solotery: Account<'info, SoLotery>,
    #[account(init, payer = authority, space = 8)]
    pub secure_check: Account<'info, SecureCheck>,
    #[account(mut)]
    pub authority: Signer<'info>,
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
pub struct DeleteSOLoteryPDA<'info> {
    #[account(mut, seeds = [b"SOLotery", solotery.authority.key().as_ref()], bump = solotery.bump_original, 
    //close = solotery
)]
    pub solotery: Account<'info, SoLotery>,
    #[account(init, payer = authority, space = 8)]
    pub secure_check: Account<'info, SecureCheck>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
} 

#[derive(Accounts)]
pub struct Delete<'info> {
    #[account(mut, seeds = [b"SOLotery", solotery.authority.key().as_ref()], bump = solotery.bump_original)]
    pub solotery: Account<'info, SoLotery>,
    #[account(mut, has_one = authority, close = authority)]
    pub secure_check: Account<'info, SecureCheck>,
    pub authority: Signer<'info>,
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
#[account]
pub struct SecureCheck {
    pub authority: Pubkey,
}
#[error_code]
pub enum ErrorCode {
    #[msg("The winner can only be chosen once")]JustOnce,
    #[msg("There are no tickets at stake")]NoPlayers,
    #[msg("You are not SOLotery key")]YouAreNotSOLotery,
}

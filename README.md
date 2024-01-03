<div align="center">

![solotery](solotery.png)

<h1>SOLotery</h1>

A decentralized and autonomous lottery

</div>

---

This project aims to revolutionize the way lotteries are played, offering a more reliable, accessible, and globally scalable alternative.

SOLotery stands out for its security and technological advancement by adopting blockchain technology, which allows it to offer an unmatched level of transparency and reliability in the world of lotteries. In addition, thanks to its decentralized nature, the lottery is kept running thanks to the active participation of the players themselves, which makes it autonomous and sustainable in the long term.

With SOLotery, players can enjoy a fairer, more transparent and more accessible lottery experience, without having to worry about intermediaries or excessive costs. SOLotery represents a great step towards the democratization of lotteries and access to new technologies in this field. I invite you to learn more about this exciting project in my GitHub repository!

## 🏦Init the stake program

```rust
pub fn create_stake(
    ctx: Context<Create>
) -> Result<()> {
    let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
    let (_stake_pda, bump) = Pubkey::find_program_address(&[b"SOLotery"], ctx.program_id);
    // Set the SOLotery account's bump value to the value returned by find_program_address.
    solotery.bump_original = bump;
    // Initialize the SOLotery account's players1 and players2 arrays to empty arrays.
    solotery.players1 = [].to_vec();
    solotery.players2 = [].to_vec();
    solotery.time_check = 1662260159; // Set the SOLotery account's time_check value to a fixed timestamp (1662260159).
    // Set the SOLotery account's players_state, winner1_selected, and winner2_selected fields to false.
    solotery.players_state = false;
    solotery.winner1_selected = false;
    solotery.winner2_selected = false;
    solotery.tickets_sold = 0;
    // Set the system program id
    solotery.winner_publickey = Pubkey::from_str("11111111111111111111111111111111").unwrap();
    Ok(())
}

#[derive(Accounts)]
pub struct Create<'info> {
    // The SOLotery account to be created. It must be initialized with the SOLotery::SIZE + 8 bytes of space.
    #[account(init, seeds = [b"SOLotery"], bump, payer = user, space = SoLotery::SIZE + 8)]
    pub solotery: Account<'info, SoLotery>,
    // The user account that will pay for the SOLotery account's initialization.
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

The function is called "create_stake" and it takes a "ctx" argument which is the execution context. The function uses the Solana Rust SDK library and defines a structure called "Create" with three fields labeled with attributes. The fields are "solotery" which is a Solana account containing the lottery data, "user" which is a Solana account representing the user creating the lottery, and "system_program" which is a Solana object representing the system program.

The function sets the value of different fields of the "solotery" account with default values, such as the original "bump" number of the account, the list of players for the lottery winners, the status of the players, the time when that the draw is verified, the number of tickets sold, and the public key of the winner.

In particular, the function sets the original "bump" number of the "solotery" lottery account using the "find_program_address" function of the Solana Rust SDK library. In addition, it sets the default values ​​for the fields "players1", "players2", "time_check", "players_state", "winner1_selected", "winner2_selected", "tickets_sold", and "winner_publickey".

Finally, the function returns an "Ok(())" result if the update was successful.


## 🎫Buy a ticket/summary>

```rust
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
                transfer_winner2(solotery,winner).unwrap();
            }
            if solotery.players1.len() == 300 {
                select_winner1(solotery,winner).unwrap();
            }
            msg!(
                "SOL sent successfully. You are already participating for the current amount of: {} SOL",
                lamports_to_sol(currents_players2).unwrap();
            );
            if Clock::get().unwrap().unix_timestamp > solotery.time_check.try_into().unwrap() {
                    dead_line1(solotery).unwrap();
                }
            } else {
                anchor_lang::solana_program::program::invoke(
                &system_instruction::transfer(&ctx.accounts.from.key(), &solotery.key(), 7777777),
                &[ctx.accounts.from.to_account_info(), ctx.accounts.stake.to_account_info().clone()],).expect("Error");
                solotery.tickets_sold += 1;
                solotery.players2.push(ctx.accounts.from.key());
                let currents_players1: u64 = (solotery.players2.len() * 7777777).try_into().unwrap();
                if solotery.winner1_selected == true {
                    transfer_winner1(solotery,winner).unwrap();
                }
                if solotery.players2.len() == 300 {
                    select_winner2(solotery,winner).unwarp();
                }
                msg!("SOL sent successfully. You are already participating for the current amount of: {} SOL", lamports_to_sol(currents_players1));
                if Clock::get().unwrap().unix_timestamp > solotery.time_check.try_into().unwrap() {
                    dead_line2(solotery).unwrap();
                }
            }
    Ok(())
}

pub fn lamports_to_sol(lamport: u64) -> f64 {
    let am: f64 = lamport as f64;
    return (am / 1000000000.0) as f64; // Convert from lamports to Sol, and return the result.
}

pub fn transfer_winner1(
    solotery: &mut Account<SoLotery>,
    winner: &mut AccountInfo
) -> Result<()> {
    let amount: u64 = (solotery.players1.len() * 7777777).try_into().unwrap();
    // deduct the prize amount from the lottery account's lamports balance
    **solotery.to_account_info().try_borrow_mut_lamports()? -= amount;
    // add the prize amount to the winner's account's lamports balance
    **winner.to_account_info().try_borrow_mut_lamports()? += amount;
    solotery.players1 = [].to_vec();
    solotery.winner_publickey = Pubkey::from_str("11111111111111111111111111111111").unwrap(); // reset the winner public key
    solotery.winner1_selected = false; // indicate that a winner has not been chosen yet for the next round
    msg!("Total amount: {} SOL", lamports_to_sol(amount));
}


pub fn transfer_winner2(
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

pub fn select_winner2(
    solotery: &mut Account<SoLotery>,
    winner: &mut AccountInfo
) -> Result<()> {
    require!(solotery.winner2_selected == false, ErrorCode::WinnerChosen); // Ensure that the winner has not already been selected
    solotery.players_state = false;
    let mut rng: oorandom::Rand64 = oorandom::Rand64::new((Clock::get().unwrap().unix_timestamp as u64).into()); // Create a random number generator seeded with the current timestamp
    let winner_choosed: usize = rng.rand_range(1..(solotery.players2.len() as u64)).try_into().unwrap(); // Generate a random number between 1 and the number of players, inclusive
    solotery.winner_publickey =  solotery.players2[winner_choosed - 1];
    solotery.winner2_selected = true; // Update the lottery account to indicate that the winner has been selected
    solotery.time_check += 86398;
    msg!("Chosen winner: {}", solotery.winner_publickey); // Log the winner's public key
}


pub fn dead_line1(
    solotery: &mut Account<SoLotery>,
) -> <()> {
    require!(solotery.winner1_selected == false, ErrorCode::WinnerChosen);
    solotery.players_state = true;
    // initialize a random number generator using the current timestamp as the seed
    let mut rng: oorandom::Rand64 = oorandom::Rand64::new((Clock::get().unwrap().unix_timestamp as u64).into());
    // generate a random index to select the winner from the list of players
    let winner_choosed: usize = rng.rand_range(1..(solotery.players1.len() as u64)).try_into().unwrap();
    // update the lottery state with the selected winner's public key
    solotery.winner_publickey =  solotery.players1[winner_choosed - 1];
    solotery.winner1_selected = true;
    // add one day to the lottery's time check to prevent further actions on the account until the next day
    solotery.time_check += 86398;
    msg!("Chosen winner: {}", solotery.winner_publickey);
}


pub fn dead_line2(
    solotery: &mut Account<SoLotery>
) -> <()> {
    require!(solotery.winner2_selected == false, ErrorCode::WinnerChosen); // Ensure that the winner has not already been selected
    solotery.players_state = false ;
    let mut rng: oorandom::Rand64 = oorandom::Rand64::new((Clock::get().unwrap().unix_timestamp as u64).into());
    let winner_choosed: usize = rng.rand_range(1..(solotery.players2.len() as u64)).try_into().unwrap(); // Generate a random number between 1 and the number of players, inclusive
    solotery.winner_publickey =  solotery.players2[winner_choosed - 1]; // Assign the winner's public key based on the random number
    solotery.winner2_selected = true;
    solotery.time_check += 86398; // Update the time check to reflect that the game has ended
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
```

There are two states of the lottery: state 1 and state 2. In each state, users can buy a ticket for 7777777 lamports which is sent to the lottery smart contract. If a user sends a transaction with 7777777 lamports and meets all the requirements, they will be added to the list of participants in the current state of the lottery.

Once there are 300 participants in the state, the lottery goes into the next state. If there are no winners in the first state, a winner is randomly chosen from the list of participants, and they receive all the lamports that were sent by the participants in both states.

The program uses the Pubkey::find_program_address() function to generate a program derived address (PDA) from the program ID and the seed b"SOLotery". The PDA is used as a stake account for the lottery.

There are several requirements that the user must meet to participate in the lottery, such as having enough lamports, the winner must match the winner_publickey in the solotery account, and the stake account must match the PDA. If these requirements are not met, the program will return an error code.

The program uses the oorandom crate to generate a random number for selecting the winner from the list of participants.


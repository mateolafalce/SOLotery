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
pub fn create_stake(ctx: Context<Create>) -> Result<()> {
    let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
    let (_stake_pda, bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[SOLOTERY], ctx.program_id);
    // set SOLotery init variables
    solotery.set_bump_original(bump);
    solotery.reset_players();
    solotery.set_time_check(DAY_OF_DEPLOY);
    solotery.reset_winner_selected();
    solotery.init_ticket_sold();
    solotery.reset_winner_pubkey();
    Ok(())
}

#[derive(Accounts)]
pub struct Create<'info> {
    // It must be initialized with the SOLotery::SIZE + 8 bytes of space (anchor).
    #[account(init, seeds = [SOLOTERY], bump, payer = user, space = SoLotery::SIZE + 8)]
    pub solotery: Account<'info, SoLotery>,
    // The user account that will pay for the SOLotery account's initialization.
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

```

The function is called "create_stake" and it takes a "ctx" argument which is the execution context. The function uses the Solana Rust SDK library and defines a structure called "Create" with three fields labeled with attributes. The fields are "solotery" which is a Solana account containing the lottery data, "user" which is a Solana account representing the user creating the lottery, and "system_program" which is a Solana object representing the system program.

The function sets the value of different fields of the "solotery" account with default values, such as the original "bump" number of the account, the list of players for the lottery winners, the status of the players, the time when that the draw is verified, the number of tickets sold, and the public key of the winner.


## 🎫Buy a ticket/summary>

```rust
pub fn ticket(ctx: Context<Ticket>) -> Result<()> {
    // useful variables
    let stake_account: Pubkey = ctx.accounts.stake.key();
    let winner_pass_as_arg: Pubkey = ctx.accounts.winner_publickey.key();
    let winner: &mut AccountInfo = &mut ctx.accounts.winner_publickey.to_account_info();
    let real_winner: Pubkey = ctx.accounts.solotery.winner_pubkey.key();
    let lamport_from_account: u64 = ctx.accounts.from.to_account_info().lamports();
    let (correct_pda, _bump) = Pubkey::find_program_address(
        &[SOLOTERY],
        &Pubkey::from_str(PROGRAM_ID).expect("PROGRAM ID ERROR"),
    );

    // validations
    require_keys_eq!(stake_account, correct_pda);
    require_keys_eq!(winner_pass_as_arg, real_winner);

    // &mut solotery
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
            select_winner(solotery).expect("Error selecting SOL to winner");
        }
    } else {
        transfer_to_winner(solotery, winner).expect("Error transfering SOL to winner");
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

```

There are two states of the lottery: state 1 and state 2. In each state, users can buy a ticket for x lamports which is sent to the lottery smart contract. If a user sends a transaction with x lamports and meets all the requirements, they will be added to the list of participants in the current state of the lottery.

Once there are 300 participants in the state, the lottery goes into the next state. If there are no winners in the first state, a winner is randomly chosen from the list of participants, and they receive all the lamports that were sent by the participants in both states.

The program uses the Pubkey::find_program_address() function to generate a program derived address (PDA) from the program ID and the seed b"SOLotery". The PDA is used as a stake account for the lottery.

There are several requirements that the user must meet to participate in the lottery, such as having enough lamports, the winner must match the winner_publickey in the solotery account, and the stake account must match the PDA. If these requirements are not met, the program will return an error code.


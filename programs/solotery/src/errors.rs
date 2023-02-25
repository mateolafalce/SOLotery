use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("This is not the winner")]ThisIsNotTheWinner, 
    #[msg("This is not the stake account")]WrongStake, 
    #[msg("No winner has been chosen")]NoWinner, 
    #[msg("The player limit is 300")]Limit,
    #[msg("Your proposal is not superior to the existing one")]AmountError,
    #[msg("The winner has already been chosen")]WinnerChosen,
}
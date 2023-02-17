// Declare the Solana SDK crate as a dependency
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Define the program ID
solana_program::declare_id!("sol_account");

// Define the entry point for the program
#[entrypoint]
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // Ensure that the program ID matches
    if program_id != &id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Parse the accounts
    let accounts_iter = &mut accounts.iter();
    let player_account = next_account_info(accounts_iter)?;
    let game_account = next_account_info(accounts_iter)?;

    // Verify that the game account belongs to this program
    if game_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Verify that the player has provided sufficient funds for the bet
    let bet_amount = 1; // Define the bet amount (in lamports)
    if player_account.lamports() < bet_amount {
        return Err(ProgramError::InsufficientFunds);
    }

    // Generate a random number to determine the outcome of the game
    let random_number = 42; // In a real game, you would use a more secure method to generate this number

    // Determine the outcome of the game based on the random number
    let is_win = random_number % 2 == 0;

    // If the player wins, transfer the winnings to their account
    if is_win {
        let winnings = 2 * bet_amount; // Double the bet amount
        **player_account.try_borrow_mut_lamports()? += winnings;
        **game_account.try_borrow_mut_lamports()? -= winnings;
        msg!("Congratulations, you won {} lamports!", winnings);
    } else {
        // Otherwise, transfer the bet amount to the game account
        **player_account.try_borrow_mut_lamports()? -= bet_amount;
        **game_account.try_borrow_mut_lamports()? += bet_amount;
        msg!("Sorry, you lost.");
    }

    Ok(())
}

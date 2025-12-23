use pinocchio::{ProgramResult, account_info::AccountInfo, entrypoint, msg, no_allocator, nostd_panic_handler, pubkey::Pubkey};
use p_amm_interface::error::AmmError;
use crate::processor::{process_deposit, process_initialize_pair, process_swap, process_withraw};

entrypoint!(process_instruction);
nostd_panic_handler!();

#[inline(always)]
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [descriminator, instruction_data @ ..] = instruction_data else {
        msg!("Instruction data too short");
        return Err(AmmError::InvalidInstruction.into());
    };

    match *descriminator {
        0 => {
            msg!("Process Initialize pair");
            process_initialize_pair(program_id, accounts, instruction_data)
        }
        1 => {
            msg!("Process deposit liquidity");
            process_deposit(program_id, accounts, instruction_data)
        }
        2 => {
            msg!("Process withdraw liquidity");
            process_withraw(program_id, accounts, instruction_data)
        }
        3 => {
            msg!("Process swap");
            process_swap(program_id, accounts, instruction_data)
        }
        _ => {
            return Err(AmmError::InvalidInstruction.into());
        }
    }
}

use pinocchio::{ProgramResult, account_info::AccountInfo, entrypoint, msg, pubkey::Pubkey};

entrypoint!(process_instruction);

#[inline(always)]
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [descriminator, instruction_data @ ..] = instruction_data else {
        msg!("Instruction data too short");
        return Err(p_amm_interface::error::AmmError::InvalidInstruction.into());
    };

    match descriminator {
        0 => {
            msg!("Process Initialize Pool instruction");
        }
        1 => {
            msg!("Process Swap instruction");
        }
        _ => {
            return Err(p_amm_interface::error::AmmError::InvalidInstruction.into());
        }
    }

    Ok(())
}

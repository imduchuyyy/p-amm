use crate::processor::U64_BYTES;
use p_amm_interface::state::Pair;
use pinocchio::program_error::ProgramError;
use pinocchio::sysvars::{rent::Rent, Sysvar};
use pinocchio::{account_info::AccountInfo, pubkey::Pubkey, ProgramResult};

#[inline(always)]
pub fn process_initialize_pair(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (trade_fee_numerator, trade_fee_denominator) = if instruction_data.len() == 16 {
        // SAFETY: instruction_data has been checked to be 16 bytes long
        let trade_fee_numerator = unsafe {
            u64::from_le_bytes(*(instruction_data[0..8].as_ptr() as *const [u8; U64_BYTES]))
        };
        let trade_fee_denominator = unsafe {
            u64::from_le_bytes(*(instruction_data[8..16].as_ptr() as *const [u8; U64_BYTES]))
        };
        (trade_fee_numerator, trade_fee_denominator)
    } else {
        // Default fee: 0.3%
        (3, 1000)
    };

    let [pair_info, token_a_info, token_b_info, pool_mint, _payer, _remainning @ ..] = accounts
    else {
        return Err(pinocchio::program_error::ProgramError::NotEnoughAccountKeys);
    };

    {   
        // Rent exemption check
        let pair_length = pair_info.data_len();

        if !Rent::get()?.is_exempt(pair_info.lamports(), pair_length) {
            return Err(ProgramError::AccountNotRentExempt);
        }
    }
    {
        // #TODO: check token_a_info and token_b_info mint addresses
    }
    {
        // #TODO: check pool_mint
    }
    let pair = unsafe { Pair::load_mut(pair_info.borrow_mut_data_unchecked())? };

    if pair.is_initialized() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    pair.initialize(
        token_a_info.key(),
        token_b_info.key(),
        pool_mint.key(),
        trade_fee_numerator,
        trade_fee_denominator,
    );

    Ok(())
}

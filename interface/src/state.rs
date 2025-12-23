use pinocchio::program_error::ProgramError;
use pinocchio::pubkey::Pubkey;

pub unsafe trait Transmutable {
    /// The length of the type.
    ///
    /// This must be equal to the size of each individual field in the type.
    const LEN: usize;
}

#[repr(C)]
pub struct Pair {
    pub is_initialized: bool,
    /// Token A
    pub token_a: Pubkey,
    /// Token B
    pub token_b: Pubkey,
    /// Pool tokens are issued when A or B tokens are deposited.
    /// Pool tokens can be withdrawn back to the original A or B token.
    pub pool_mint: Pubkey,
    /// Pool token account to receive trading and / or withdrawal fees
    pub pool_fee_account: Pubkey,

    /// All fee information
    pub trade_fee_numerator: u64,
    /// Trade fee denominator
    pub trade_fee_denominator: u64,
}

impl Pair {
    #[inline(always)]
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    #[inline(always)]
    pub unsafe fn load_mut(data: &mut [u8]) -> Result<&mut Self, ProgramError> {
        load_mut_unchecked::<Self>(data).and_then(|pair| {
            if !pair.is_initialized() {
                return Err(ProgramError::UninitializedAccount);
            }
            Ok(pair)
        })
    }

    #[inline(always)]
    pub unsafe fn load(data: &[u8]) -> Result<&Self, ProgramError> {
        load_unchecked::<Self>(data).and_then(|pair| {
            if !pair.is_initialized() {
                return Err(ProgramError::UninitializedAccount);
            }
            Ok(pair)
        })
    }
}

unsafe impl Transmutable for Pair {
    const LEN: usize = core::mem::size_of::<Pair>();
}

#[inline(always)]
pub unsafe fn load_mut_unchecked<T: Transmutable>(
    bytes: &mut [u8],
) -> Result<&mut T, ProgramError> {
    if bytes.len() != T::LEN {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(&mut *(bytes.as_mut_ptr() as *mut T))
}

#[inline(always)]
pub unsafe fn load_unchecked<T: Transmutable>(bytes: &[u8]) -> Result<&T, ProgramError> {
    if bytes.len() != T::LEN {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(&*(bytes.as_ptr() as *const T))
}

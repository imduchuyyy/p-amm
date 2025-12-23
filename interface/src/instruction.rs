use pinocchio::program_error::ProgramError;

#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(strum_macros::FromRepr, strum_macros::EnumIter))]
pub enum Instruction {
    /// Initializes a new AMM pool
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the pool
    /// 1. `[]` The token A mint account
    /// 2. `[]` The token B mint account
    /// 3. `[writable]` The pool state account
    /// 4. `[]` The rent sysvar
    InitializePool,

    /// Deposits liquidity into the AMM pool
    /// Accounts expected:
    /// 0. `[signer]` The account of the person depositing liquidity
    /// 1. `[writable]` The token A account
    /// 2. `[writable]` The token B account
    /// 3. `[writable]` The pool state account
    DepositLiquidity,

    /// Withdraws liquidity from the AMM pool
    /// Accounts expected:
    /// 0. `[signer]` The account of the person withdrawing liquidity
    /// 1. `[writable]` The token A account
    /// 2. `[writable]` The token B account
    /// 3. `[writable]` The pool state account
    WithdrawLiquidity,

    /// Swaps tokens in the AMM pool
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person performing the swap
    /// 1. `[writable]` The source token account
    /// 2. `[writable]` The destination token account
    /// 3. `[writable]` The pool state account
    SwapTokens,
}

impl TryFrom<u8> for Instruction {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..=3 => Ok(unsafe { core::mem::transmute::<u8, Instruction>(value) }),
            _ => panic!("Invalid instruction"),
        }
    }
}

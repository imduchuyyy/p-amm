use pinocchio::program_error::{ProgramError, ToStr};

/// Errors that may be returned by the Token program.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AmmError {
    /// Invalid instruction
    InvalidInstruction,
    /// Insufficient funds
    InsufficientFunds,
    /// Pool not initialized
    PoolNotInitialized,
    /// Invalid account
    InvalidAccount,
}

impl From<AmmError> for ProgramError {
    fn from(e: AmmError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl ToStr for AmmError {
    fn to_str<E>(&self) -> &'static str
    where
        E: 'static + ToStr + TryFrom<u32>,
    {
        match self {
            AmmError::InvalidInstruction => "Invalid instruction",
            AmmError::InsufficientFunds => "Insufficient funds",
            AmmError::PoolNotInitialized => "Pool not initialized",
            AmmError::InvalidAccount => "Invalid account",
        }
    }
}

impl TryFrom<u32> for AmmError {
    type Error = ProgramError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            x if x == AmmError::InvalidInstruction as u32 => Ok(AmmError::InvalidInstruction),
            x if x == AmmError::InsufficientFunds as u32 => Ok(AmmError::InsufficientFunds),
            x if x == AmmError::PoolNotInitialized as u32 => Ok(AmmError::PoolNotInitialized),
            x if x == AmmError::InvalidAccount as u32 => Ok(AmmError::InvalidAccount),
            _ => Err(ProgramError::InvalidArgument),
        }
    }
}
use std::error;
use std::fmt::{self, Display};

/// Enum to store possible errors when using this library.
#[derive(Debug)]
pub enum Error {
    /// When converting, the resulting value is too large to store in the target type.
    ConversionOverflow,
    /// The position is out of bounds for the value being accessed.
    PosOutOfBounds,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ConversionOverflow => write!(f, "converted value overflows"),
            Error::PosOutOfBounds => write!(f, "position out of bounds"),
        }
    }
}

impl error::Error for Error {}

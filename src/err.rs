//! Error handling module for oscirs

use std::fmt;
use std::error;

/// Custom error type for oscirs crate
#[derive(Debug)]
pub enum SciError {
    DimensionsError,
    RangeError,
    VectorLengthsError,
    OtherError(String)
}

impl fmt::Display for SciError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SciError::DimensionsError =>
                write!(f, "Given dimensions not valid for requested operation"),
            SciError::RangeError =>
                write!(f, "Given value is not in range of given vector"),
            SciError::VectorLengthsError =>
                write!(f, "Input vector lengths not valid for requested operation"),
            SciError::OtherError(error) =>
                write!(f, "{}", error)
        }
    }
}

impl error::Error for SciError {}
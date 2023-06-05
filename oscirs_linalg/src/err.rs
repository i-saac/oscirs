use std::fmt;
use std::error;

use opencl3::error_codes::ClError;

#[derive(Debug)]
pub enum LAError {
    ArgumentError,
    DataUpdateError,
    MatrixMismatchError,
    MemoryError,
    ResizeError,
    ReturnValueError,
    SizeError,
    ClError(ClError),
    OtherError(String)
}

impl fmt::Display for LAError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LAError::ArgumentError =>
                write!(f, "Too many or too few arguments provided"),
            LAError::DataUpdateError =>
                write!(f, "New data vector has invalid length for current matrix dimensions"),
            LAError::MatrixMismatchError =>
                write!(f, "Matrix dimensions are not consistent with data vector length"),
            LAError::MemoryError =>
                write!(f, "Memory Calculator and Handler have inconsistent memory"),
            LAError::ResizeError =>
                write!(f, "Invalid dimensions for matrix resize operation"),
            LAError::ReturnValueError =>
                write!(f, "No return value"),
            LAError::SizeError =>
                write!(f, "Matrix dimensions not valid for requested operation"),
            LAError::ClError(error) =>
                write!(f, "{}", error),
            LAError::OtherError(error) =>
                write!(f, "{}", error)
        }
    }
}

impl error::Error for LAError {}

impl From<ClError> for LAError {
    fn from(err: ClError) -> Self {
        LAError::ClError(err)
    }
}
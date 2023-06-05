use std::fmt;
use std::error;

#[derive(Debug)]
pub enum SciError {
    DimensionsError,
    VectorLengthsError,
    OtherError(String)
}

impl fmt::Display for SciError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SciError::DimensionsError =>
                write!(f, "Given dimensions not valid for requested operation"),
            SciError::VectorLengthsError =>
                write!(f, "Input vector lengths not valid for requested operation"),
            SciError::OtherError(error) =>
                write!(f, "{}", error)
        }
    }
}

impl error::Error for SciError {}
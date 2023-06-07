//! Error handling module for oscirs_plot

use std::fmt;
use std::error;

#[derive(Debug)]
pub enum PlotError {
    BoundsError,
    ChartTypeError,
    DataLengthError,
    OtherError(String)
}

impl fmt::Display for PlotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlotError::BoundsError =>
                write!(f, "Data out of bounds for chart type"),
            PlotError::ChartTypeError =>
                write!(f, "Incompatible chart type for requested operation"),
            PlotError::DataLengthError =>
                write!(f, "Input data lengths are not equal"),
            PlotError::OtherError(error) =>
                write!(f, "{}", error)
        }
    }
}

impl error::Error for PlotError {}

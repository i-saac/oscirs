//! [![crates.io](https://shields.io/crates/v/oscirs_plot)](https://crates.io/crates/oscirs_plot)
//! 
//! A plotting library for Rust

use std::fmt;

pub mod err;
pub mod svg;
pub mod style;

/// Simplified svgplot module to clean up imports and get you plotting quickly
pub mod svgplot_core {
    pub use crate::Color;
    pub use crate::svg::scatterline::Scatterline;
    pub use crate::svg::bar::Bar;
    pub use crate::svg::histogram::Histogram;
    pub use crate::style::PlotStyle;
}

/// Custom result type using PlotError
pub type PlotResult<T> = Result<T, err::PlotError>;

/// Enum of plotting colors
#[derive(Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
    Pink,
    Black,
    Gray,
    LightGray,
    None,
    Hex(String)
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Red =>
                write!(f, "#FF0000"),
            Color::Green =>
                write!(f, "#00FF00"),
            Color::Blue =>
                write!(f, "#0000FF"),
            Color::Pink =>
                write!(f, "#FF00FF"),
            Color::Black =>
                write!(f, "#000000"),
            Color::Gray =>
                write!(f, "#777777"),
            Color::LightGray =>
                write!(f, "#D3D3D3"),
            Color::None =>
                write!(f, "none"),
            Color::Hex(hex_code) =>
                write!(f, "{}", hex_code)
        }
    }
}
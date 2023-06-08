//! Definition and Implementations for PlotStyle struct

use crate::Color;

/// Struct containing plotting parameters for PlotStyle
#[derive(Clone)]
pub struct PlotStyle {
    pub stroke_color: Color,
    pub stroke_width: i32,
    pub fill_color: Color,
    pub has_markers: bool
}

// Default parameters for PlotStyle
impl Default for PlotStyle {
    fn default() -> PlotStyle {
        PlotStyle {
            stroke_color: Color::Black,
            stroke_width: 2,
            fill_color: Color::None,
            has_markers: false
        }
    }
}

pub fn scatter_style(color: Color) -> PlotStyle {
    PlotStyle {
        stroke_color: color,
        stroke_width: 0,
        has_markers: true,
        ..Default::default()
    }
}
//! Definition and Implementations for SVGStyle struct

use crate::Color;

/// Struct containing plotting parameters for SVGFigure
#[derive(Clone)]
pub struct SVGStyle {
    pub stroke_color: Color,
    pub stroke_width: i32,
    pub fill_color: Color,
    pub has_markers: bool
}

// Default parameters for SVGStyle
impl Default for SVGStyle {
    fn default() -> SVGStyle {
        SVGStyle {
            stroke_color: Color::Black,
            stroke_width: 2,
            fill_color: Color::None,
            has_markers: false
        }
    }
}

pub fn scatter_style(color: Color) -> SVGStyle {
    SVGStyle {
        stroke_color: color,
        stroke_width: 0,
        has_markers: true,
        ..Default::default()
    }
}
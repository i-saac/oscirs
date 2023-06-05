use crate::Color;

#[derive(Clone)]
pub struct SVGStyle {
    pub stroke_color: Color,
    pub stroke_width: i32,
    pub fill_color: Color,
    pub has_markers: bool
}

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
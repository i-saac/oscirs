//! Module organizing all plots that result in a .svg file

pub mod scatterline;
pub mod bar;

use crate::style::PlotStyle;

// Push line element to render_string (private function)
fn draw_line(
    render_string: &mut String,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    style: &PlotStyle
) {
    render_string.push_str(
        &format!(r#"<line x1="{}" y1="{}" x2="{}" y2="{}" fill="{}" stroke="{}" stroke-width="{}"/>"#,
            x1, y1, x2, y2, style.fill_color, style.stroke_color, style.stroke_width
        )
    );
}

// Push text element to render_string (private function)
fn draw_text(
    render_string: &mut String,
    x: usize,
    y: usize,
    angle: usize,
    text: &str,
    style: &PlotStyle,
    text_size: &str
) {
    render_string.push_str(
        &format!(r#"<text transform="translate({}, {}) rotate({})" color="{}" font-size="{}" dominant-baseline="middle" text-anchor="middle">{}</text>"#,
            x, y, angle, style.stroke_color, text_size, text
        )
    );
}
//! .svg-based bar figure generation

extern crate open;

use std::fs::File;
use std::io::Write;
use std::error::Error;

use crate::PlotResult;
use crate::err::PlotError;
use crate::style::PlotStyle;

use super::{
    draw_line,
    draw_text
};

/// Struct containing information required to generate a final figure
pub struct Bar {
    width: usize,
    height: usize,
    axis_pad: usize,
    bar_pad: usize,
    y_label: String,
    y_limit: Option<f32>,
    title: String,
    anno_style: PlotStyle,
    bar_labels: Vec<String>,
    y_dataset: Vec<f32>,
    plot_style: PlotStyle,
    n_ticks: usize
}

// Default parameters for Bar
impl Default for Bar {
    fn default() -> Bar {
        Bar {
            width: 1000,
            height: 750,
            axis_pad: 50,
            bar_pad: 10,
            y_label: "".to_string(),
            y_limit: None,
            title: "".to_string(),
            anno_style: PlotStyle::default(),
            bar_labels: Vec::default(),
            y_dataset: Vec::default(),
            plot_style: PlotStyle::default(),
            n_ticks: 11
        }
    }
}

impl Bar {
    /// Set y label text
    pub fn label_y(&mut self, new_label: &str) {
        self.y_label = new_label.to_string();
    }

    /// Set title text
    pub fn title(&mut self, new_title: &str) {
        self.title = new_title.to_string();
    }

    /// Add bar height data
    pub fn add_data(&mut self, bar_labels: &Vec<String>, new_data: &Vec<f32>, plot_style: &PlotStyle) -> PlotResult<()> {
        if bar_labels.len() != new_data.len() {
            return Err(PlotError::DataLengthError)
        }

        if new_data.into_iter()
            .any(|&value| value < 0.0)
        {
            return Err(PlotError::BoundsError)
        }

        self.bar_labels = bar_labels.clone();
        self.y_dataset = new_data.clone();
        self.plot_style = plot_style.clone();

        Ok(())
    }

    /// Set y axis limits
    pub fn set_ymax(&mut self, upper_lim: f32) {
        self.y_limit = Some(upper_lim);
    }

    /// Compile bar plot data into file_name.svg and open the image
    pub fn render(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
        // Header of svg file
        let mut render_string: String =  format!(r#"<!DOCTYPE svg><svg xmlns="http://www.w3.org/2000/svg" viewBox="-50 -50 {} {}" width="{}" height="{}">"#, self.width + 50, self.height + 50, self.width, self.height);

        // Draw title
        draw_text(&mut render_string, self.width / 2, self.axis_pad / 2, 0, &self.title, &self.anno_style, "xx-large");

        // Find minimum of y_dataset
        let y_data_max: f32 = (&self.y_dataset).into_iter()
            .fold(f32::NEG_INFINITY, |left, &right| left.max(right));

        // Get bounds of y axis
        let y_abs_min: f32 = 0.0;
        let y_abs_max: f32 = self.y_limit.unwrap_or(y_data_max);

        // Draw axes
        self.draw_axes(&mut render_string, y_abs_min, y_abs_max);

        // Create plot window sub-image (this is done to auto-clip out of bounds data points)
        render_string.push_str(&format!(r#"<svg width="{}" height="{}" x="{}" y="{}">"#, self.width - 2 * self.axis_pad, self.height - 2 * self.axis_pad, self.axis_pad, self.axis_pad));

        // Map y series from plot values to pixel values
        let mapped_y: Vec<usize> = (0..self.y_dataset.len()).into_iter()
            .map(|idx| (self.height - self.axis_pad) - (self.axis_pad as f32 + (self.height - 2 * self.axis_pad) as f32 * (self.y_dataset[idx] - y_abs_min) / (y_abs_max - y_abs_min)) as usize)
            .collect::<Vec<usize>>();

        // Get number of bars
        let n_bars: usize = self.bar_labels.len();

        let bar_width: usize = ((self.width - 2 * self.axis_pad) / (n_bars + 2)) - self.bar_pad;

        // Loop through bars
        for bar_idx in 0..n_bars {
            // Determine proportion travelled along axis
            let progression: f32 = (bar_idx + 1) as f32 / (n_bars + 1) as f32;

            // Calculate x location of the next tick mark
            let bar_center_loc: usize = (progression * (self.width - 2 * self.axis_pad) as f32) as usize;

            // Draw Bar
            render_string.push_str(&format!(r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}"><title>{}</title></rect>"#,
                bar_center_loc - (bar_width / 2),
                mapped_y[bar_idx],
                bar_width,
                (self.height - self.axis_pad) - mapped_y[bar_idx],
                self.plot_style.fill_color,
                self.plot_style.stroke_color,
                self.plot_style.stroke_width,
                self.y_dataset[bar_idx]
            ))
        }

        // Close out svg file
        render_string.push_str(&format!("</svg></svg>"));

        {
            // Create file_name.svg
            let mut output_svg: File = File::create(format!("{}.svg", file_name))?;

            // Write string data to file_name.svg
            output_svg.write_all(render_string.as_bytes())?;
        }

        // Automatically open svg in browser
        open::that(format!("./{}.svg", file_name))?;

        Ok(())
    }

    // Append drawn axis elements to render_string (private function)
    fn draw_axes(&self, mut render_string: &mut String, y_start: f32, y_end: f32) {
        // Half of axis tick mark length
        let tick_r: usize = 6;

        // Draw axis lines
        draw_line(&mut render_string, self.axis_pad, self.height - self.axis_pad, self.width - self.axis_pad, self.height - self.axis_pad, &self.anno_style);
        draw_line(&mut render_string, self.axis_pad, self.height - self.axis_pad, self.axis_pad, self.axis_pad, &self.anno_style);

        // Get label location
        let y_label_y: usize = self.axis_pad + (self.height - 2 * self.axis_pad) / 2;

        // Draw y axis label
        draw_text(&mut render_string, 10, y_label_y, 270, &self.y_label, &self.anno_style, "large");

        // Get number of bars
        let n_bars: usize = self.bar_labels.len();

        // Drawing x axis tick marks
        for bar_idx in 0..n_bars {
            // Determine proportion travelled along axis
            let progression: f32 = (bar_idx + 1) as f32 / (n_bars + 1) as f32;

            // Calculate x location of the next tick mark
            let x_tick_loc: usize = self.axis_pad + (progression * (self.width - 2 * self.axis_pad) as f32) as usize;

            // Draw axis tick mark
            draw_line(&mut render_string, x_tick_loc, self.height - self.axis_pad + tick_r, x_tick_loc, self.height - self.axis_pad - tick_r, &self.anno_style);

            // Draw tick mark label
            draw_text(&mut render_string, x_tick_loc, self.height - self.axis_pad + 3 * tick_r, 0, &self.bar_labels[bar_idx], &self.anno_style, "medium");
        }

        // Drawing y axis tick marks
        for y_tick in 0..self.n_ticks {
            // Determine proportion travelled along axis
            let progression: f32 = y_tick as f32 / (self.n_ticks - 1) as f32;

            // Calculate y location of the next tick mark
            let y_tick_loc: usize = self.height - (self.axis_pad + (progression * (self.height - 2 * self.axis_pad) as f32) as usize);

            // Calculate tick mark label value
            let y_tick_val: f32 = y_start + progression * (y_end - y_start);

            // Draw axis tick mark
            draw_line(&mut render_string, self.axis_pad - tick_r, y_tick_loc, self.axis_pad + tick_r, y_tick_loc, &self.anno_style);

            // Draw tick mark label
            draw_text(&mut render_string, self.axis_pad - 3 * tick_r, y_tick_loc, 270, &format!("{:.2}", y_tick_val), &self.anno_style, "medium");
        }
    }
}
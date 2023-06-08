//! .svg-based histogram figure generation

extern crate open;

use std::fs::File;
use std::io::Write;
use std::error::Error;

use crate::style::PlotStyle;

use super::{
    draw_line,
    draw_text
};

/// Struct containing information required to generate a final figure
pub struct Histogram {
    width: usize,
    height: usize,
    axis_pad: usize,
    x_label: String,
    x_limits: Option<[f32; 2]>,
    y_label: String,
    y_limit: Option<usize>,
    title: String,
    anno_style: PlotStyle,
    dataset: Vec<f32>,
    plot_style: PlotStyle,
    n_bars: usize,
    n_y_ticks: Option<usize>
}

// Default parameters for Histogram
impl Default for Histogram {
    fn default() -> Histogram {
        Histogram {
            width: 1000,
            height: 750,
            axis_pad: 50,
            x_label: "".to_string(),
            x_limits: None,
            y_label: "".to_string(),
            y_limit: None,
            title: "".to_string(),
            anno_style: PlotStyle::default(),
            dataset: Vec::default(),
            plot_style: PlotStyle::default(),
            n_bars: 10,
            n_y_ticks: None
        }
    }
}

impl Histogram {
    /// Set x label text
    pub fn label_x(&mut self, new_label: &str) {
        self.x_label = new_label.to_string();
    }

    /// Set y label text
    pub fn label_y(&mut self, new_label: &str) {
        self.y_label = new_label.to_string();
    }

    /// Set title text
    pub fn title(&mut self, new_title: &str) {
        self.title = new_title.to_string();
    }

    /// Add histogram data series
    pub fn add_data(&mut self, data: &Vec<f32>, plot_style: &PlotStyle) {
        self.dataset = data.clone();
        self.plot_style = plot_style.clone();
    }

    /// Set number of histogram bars (including outside of x limits regardless of whether there are any)
    pub fn set_n_bars(&mut self, n_bars: usize) {
        self.n_bars = n_bars;
    }

    /// Set range of each block (may cause unintended behavior if x limits are changed after calling this)
    pub fn set_block_range(&mut self, block_range: f32) {
        if self.x_limits.is_some() {
            let limits: [f32; 2] = self.x_limits.unwrap();

            self.n_bars = 2 + ((limits[1] - limits[0]) / block_range) as usize;
        }
        else {
            // Find max and min of data
            let data_min: f32 = (&self.dataset).into_iter()
                .fold(f32::INFINITY, |left, &right| left.min(right));
            let data_max: f32 = (&self.dataset).into_iter()
                .fold(f32::NEG_INFINITY, |left, &right| left.max(right));

            self.n_bars = 2 + ((data_max - data_min) / block_range) as usize;
        }
    }

    /// Set x axis limits
    pub fn set_xlims(&mut self, lower_lim: f32, upper_lim: f32) {
        self.x_limits = Some([lower_lim, upper_lim]);
    }

    /// Set y axis maximum
    pub fn set_ymax(&mut self, upper_lim: usize) {
        let mut n_ticks: usize = 1;
        for divisor in (1..=10).rev() {
            if upper_lim % divisor == 0 {
                n_ticks = (upper_lim / divisor) + 1;
            }
        }
        self.n_y_ticks = Some(n_ticks);

        self.y_limit = Some(upper_lim);
    }

    /// Compile histogram data into file_name.svg and open the image
    pub fn render(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
        // Header of svg file
        let mut render_string: String =  format!(r#"<!DOCTYPE svg><svg xmlns="http://www.w3.org/2000/svg" viewBox="-50 -50 {} {}" width="{}" height="{}">"#, self.width + 50, self.height + 50, self.width, self.height);

        // Draw title
        draw_text(&mut render_string, self.width / 2, self.axis_pad / 2, 0, &self.title, &self.anno_style, "xx-large");

        // Find number of segments in between x limits
        let inner_segments: usize = self.n_bars - 2;

        // Find max and min of data
        let data_min: f32 = (&self.dataset).into_iter()
            .fold(f32::INFINITY, |left, &right| left.min(right));
        let data_max: f32 = (&self.dataset).into_iter()
            .fold(f32::NEG_INFINITY, |left, &right| left.max(right));

        // Unwrap x limits and find delta between each tick mark
        let limits: [f32; 2] = self.x_limits.unwrap_or([data_min, data_max]);
        let delta: f32 = (limits[1] - limits[0]) / inner_segments as f32;


        let mut cumulative_frequencies: Vec<usize> = (0..=inner_segments)
            .map(|idx| 
                (&self.dataset).into_iter()
                    .filter(|&&val| val >= (limits[0] + delta * idx as f32))
                    .count()
            ).collect();

        cumulative_frequencies.push(0);

        let mut frequencies: Vec<usize> = Vec::with_capacity(self.n_bars);
        
        frequencies.push(self.dataset.len() - cumulative_frequencies[0]);

        for idx in 1..cumulative_frequencies.len() {
            frequencies.push(cumulative_frequencies[idx - 1] - cumulative_frequencies[idx])
        }

        let max_freq: usize = (&frequencies).into_iter()
            .fold(0, |left, &right| left.max(right));

        let mut n_ticks: usize = 1;
        for divisor in (1..=10).rev() {
            if max_freq % divisor == 0 {
                n_ticks = (max_freq / divisor) + 1;
            }
        }

        self.draw_axes(&mut render_string, limits[0], limits[1], self.y_limit.unwrap_or(max_freq), self.n_y_ticks.unwrap_or(n_ticks));

        // Map frequencies from counts to pixel values
        let mapped_freq: Vec<usize> = (0..frequencies.len()).into_iter()
            .map(|idx| (self.height - self.axis_pad) - (self.axis_pad as f32 + (self.height - 2 * self.axis_pad) as f32 * (frequencies[idx] as f32) / (self.y_limit.unwrap_or(max_freq) as f32)) as usize)
            .collect::<Vec<usize>>();

        // Create plot window sub-image (this is done to auto-clip out of bounds data points)
        render_string.push_str(&format!(r#"<svg width="{}" height="{}" x="{}" y="{}">"#, self.width - 2 * self.axis_pad, self.height - 2 * self.axis_pad, self.axis_pad, self.axis_pad));

        let bar_width: usize = (self.width - 2 * self.axis_pad) / self.n_bars;

        for bar_idx in 0..self.n_bars {
            let progression: f32 = bar_idx as f32 / self.n_bars as f32;

            render_string.push_str(&format!(r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}"><title>{}</title></rect>"#,
                (progression * (self.width - 2 * self.axis_pad) as f32) as usize,
                mapped_freq[bar_idx],
                bar_width,
                (self.height - self.axis_pad) - mapped_freq[bar_idx],
                self.plot_style.fill_color,
                self.plot_style.stroke_color,
                self.plot_style.stroke_width,
                frequencies[bar_idx]
            ));
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
    fn draw_axes(&self, mut render_string: &mut String, x_start: f32, x_end: f32, y_end: usize, n_y_ticks: usize) {
        // Half of axis tick mark length
        let tick_r: usize = 6;

        // Draw axis lines
        draw_line(&mut render_string, self.axis_pad, self.height - self.axis_pad, self.width - self.axis_pad, self.height - self.axis_pad, &self.anno_style);
        draw_line(&mut render_string, self.axis_pad, self.height - self.axis_pad, self.axis_pad, self.axis_pad, &self.anno_style);

        // Get label locations
        let x_label_x: usize = self.axis_pad + (self.width - 2 * self.axis_pad) / 2;
        let y_label_y: usize = self.axis_pad + (self.height - 2 * self.axis_pad) / 2;

        // Draw axis labels
        draw_text(&mut render_string, x_label_x, self.height - 10, 0, &self.x_label, &self.anno_style, "large");
        draw_text(&mut render_string, 10, y_label_y, 270, &self.y_label, &self.anno_style, "large");

        // Drawing x axis tick marks
        for x_tick in 0..(self.n_bars - 1) {
            // Determine proportion travelled along axis
            let progression: f32 = (x_tick + 1) as f32 / self.n_bars as f32;

            // Calculate x location of the next tick mark
            let x_tick_loc: usize = self.axis_pad + (progression * (self.width - 2 * self.axis_pad) as f32) as usize;

            // Calculate tick mark label value
            let x_tick_val: f32 = x_start + (x_tick as f32 / (self.n_bars - 2) as f32) * (x_end - x_start);

            // Draw axis tick mark
            draw_line(&mut render_string, x_tick_loc, self.height - self.axis_pad + tick_r, x_tick_loc, self.height - self.axis_pad - tick_r, &self.anno_style);

            // Draw tick mark label
            draw_text(&mut render_string, x_tick_loc, self.height - self.axis_pad + 3 * tick_r, 0, &format!("{:.2}", x_tick_val), &self.anno_style, "medium");
        }

        // Drawing y axis tick marks
        for y_tick in 0..n_y_ticks {
            // Determine proportion travelled along axis
            let progression: f32 = y_tick as f32 / (n_y_ticks - 1) as f32;

            // Calculate y location of the next tick mark
            let y_tick_loc: usize = self.height - (self.axis_pad + (progression * (self.height - 2 * self.axis_pad) as f32) as usize);

            // Calculate tick mark label value
            let y_tick_val: usize = (progression * y_end as f32) as usize;

            // Draw axis tick mark
            draw_line(&mut render_string, self.axis_pad - tick_r, y_tick_loc, self.axis_pad + tick_r, y_tick_loc, &self.anno_style);

            // Draw tick mark label
            draw_text(&mut render_string, self.axis_pad - 3 * tick_r, y_tick_loc, 270, &format!("{:.2}", y_tick_val), &self.anno_style, "medium");
        }
    }
}
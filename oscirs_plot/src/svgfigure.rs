extern crate open;

use std::fs::File;
use std::io::Write;
use std::error::Error;

use crate::Color;
use crate::svgstyle::SVGStyle;

pub enum Location {
    Northwest
}

pub struct SVGFigure {
    width: i32,
    height: i32,
    axis_pad: i32,
    x_label: String,
    x_limits: Option<[f32; 2]>,
    y_label: String,
    y_limits: Option<[f32; 2]>,
    title: String,
    anno_style: SVGStyle,
    x_dataset: Vec<Vec<f32>>,
    y_dataset: Vec<Vec<f32>>,
    legend_names: Option<Vec<String>>,
    plot_styles: Vec<SVGStyle>,
    max_ticks: i32,
    axis_equal: bool
}

// Default parameters for SVGFigure
impl Default for SVGFigure {
    fn default() -> SVGFigure {
        SVGFigure {
            width: 1000,
            height: 750,
            axis_pad: 50,
            x_label: "".to_string(),
            x_limits: None,
            y_label: "".to_string(),
            y_limits: None,
            title: "".to_string(),
            anno_style: SVGStyle::default(),
            x_dataset: Vec::default(),
            y_dataset: Vec::default(),
            legend_names: None,
            plot_styles: Vec::default(),
            max_ticks: 11,
            axis_equal: false
        }
    }
}

impl SVGFigure {
    // Set x label text
    pub fn label_x(&mut self, new_label: &str) {
        self.x_label = new_label.to_string();
    }

    // Set y label text
    pub fn label_y(&mut self, new_label: &str) {
        self.y_label = new_label.to_string();
    }
    
    // Set title text
    pub fn title(&mut self, new_title: &str) {
        self.title = new_title.to_string();
    }

    // Add data series
    pub fn add_data(&mut self, x_data: &Vec<f32>, y_data: &Vec<f32>, plot_style: &SVGStyle) {
        self.x_dataset.push(x_data.clone());
        self.y_dataset.push(y_data.clone());
        self.plot_styles.push(plot_style.clone());
    }

    // Set x axis limits
    pub fn set_xlims(&mut self, lower_lim: f32, upper_lim: f32) {
        self.x_limits = Some([lower_lim, upper_lim]);
    }

    // Set y axis limits
    pub fn set_ylims(&mut self, lower_lim: f32, upper_lim: f32) {
        self.y_limits = Some([lower_lim, upper_lim]);
    }

    // Set axis scales to equal
    pub fn axis_equal(&mut self) {
        self.axis_equal = true
    }

    // Set axis scales to auto
    pub fn axis_auto(&mut self) {
        self.axis_equal = false
    }

    // Assign legend labels
    pub fn assign_legend(&mut self, legend_names: &Vec<String>) {
        self.legend_names = Some(legend_names.clone());
    }
    
    // Append drawn axis elements to render_string (private function)
    fn draw_axes(&self, mut render_string: &mut String, x_start: f32, x_end: f32, y_start: f32, y_end: f32, n_x_ticks: i32, n_y_ticks: i32) {
        // Half of axis tick mark length
        let tick_r: i32 = 6;

        // Draw axis lines
        draw_line(&mut render_string, self.axis_pad, self.height - self.axis_pad, self.width - self.axis_pad, self.height - self.axis_pad, &self.anno_style);
        draw_line(&mut render_string, self.axis_pad, self.height - self.axis_pad, self.axis_pad, self.axis_pad, &self.anno_style);

        // Get label locations
        let x_label_x: i32 = self.axis_pad + (self.width - 2 * self.axis_pad) / 2;
        let y_label_y: i32 = self.axis_pad + (self.height - 2 * self.axis_pad) / 2;

        // Draw axis labels
        draw_text(&mut render_string, x_label_x, self.height - 10, 0, &self.x_label, &self.anno_style, "large");
        draw_text(&mut render_string, 10, y_label_y, -90, &self.y_label, &self.anno_style, "large");

        // Drawing x axis tick marks
        for x_tick in 0..n_x_ticks {
            // Determine proportion travelled along axis
            let progression: f32 = x_tick as f32 / (n_x_ticks - 1) as f32;

            // Calculate x location of the next tick mark
            let x_tick_loc: i32 = self.axis_pad + (progression * (self.width - 2 * self.axis_pad) as f32) as i32;

            // Calculate tick mark label value
            let x_tick_val: f32 = x_start + progression * (x_end - x_start);

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
            let y_tick_loc: i32 = self.height - (self.axis_pad + (progression * (self.height - 2 * self.axis_pad) as f32) as i32);

            // Calculate tick mark label value
            let y_tick_val: f32 = y_start + progression * (y_end - y_start);

            // Draw axis tick mark
            draw_line(&mut render_string, self.axis_pad - tick_r, y_tick_loc, self.axis_pad + tick_r, y_tick_loc, &self.anno_style);

            // Draw tick mark label
            draw_text(&mut render_string, self.axis_pad - 3 * tick_r, y_tick_loc, -90, &format!("{:.2}", y_tick_val), &self.anno_style, "medium");
        }
    }

    fn draw_legend(&self, render_string: &mut String, location: Location) {
        let entries: &Vec<String> = self.legend_names.as_ref().unwrap();

        let entry_height: i32 = 20;
        let char_width: i32 = 8;
        let max_entry_length: i32 = entries.into_iter()
            .fold(i32::MIN, |left, right| left.max(right.len() as i32));

        let (legend_x_loc, legend_y_loc) = match location {
            Location::Northwest =>
                (20, 20)
        };

        render_string.push_str(&format!(r#"<rect x="{}" y="{}" width="{}" height="{}" fill="none" stroke="{}" stroke-width="{}"/>"#, 
            legend_x_loc,
            legend_y_loc,
            entry_height + char_width * max_entry_length + entry_height / 2,
            entry_height * self.legend_names.as_ref().unwrap().len() as i32,
            self.anno_style.stroke_color,
            self.anno_style.stroke_width
        ));

        for entry_idx in 0..entries.len() {
            render_string.push_str(&format!(r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="none" />"#,
                legend_x_loc,
                legend_y_loc + entry_idx as i32 * entry_height,
                entry_height,
                entry_height,
                self.plot_styles[entry_idx].stroke_color
            ));

            let entry_string: &String = &entries[entry_idx];
            render_string.push_str(&format!(r#"<text x="{}" y="{}" color="{}" dominant-baseline="middle">{}</text>"#,
                legend_x_loc + entry_height + entry_height / 4,
                legend_y_loc + entry_idx as i32 * entry_height + entry_height / 2,
                self.anno_style.stroke_color,
                entry_string
            ));
        }
    }

    // Compile plot data into file_name.svg and open the image
    pub fn render(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
        // Define point marker size
        let point_r: i32 = 3;

        // Header of svg file
        let mut render_string: String = format!(r#"<!DOCTYPE svg><svg xmlns="http://www.w3.org/2000/svg" viewBox="-50 -50 {} {}" width="{}" height="{}">"#, self.width + 50, self.height + 50, self.width, self.height);

        // Draw title
        draw_text(&mut render_string, self.width / 2, self.axis_pad / 2, 0, &self.title, &self.anno_style, "xx-large");

        // Calculate number of x tick marks required by iterating over all x data vectors and finding the longest one
        let n_ticks: i32 = (&self.x_dataset).into_iter()
            .fold(i32::MIN, |left, right| left.max(right.len() as i32))
            .min(self.max_ticks); // Make sure n_ticks doesn't exceed max_ticks

        // Find global minimum of x_dataset (vector of x data vectors)
        let x_data_min: f32 =  (&self.x_dataset).into_iter()
            .map(|x_set| x_set.into_iter()
                    .fold(f32::INFINITY, |left, &right| left.min(right))
            ).fold(f32::INFINITY, |left, right| left.min(right));

        // Find global maximum of x_dataset (vector of x data vectors)
        let mut x_data_max: f32 = (&self.x_dataset).into_iter()
            .map(|x_set| x_set.into_iter()
                    .fold(f32::NEG_INFINITY, |left, &right| left.max(right))
            ).fold(f32::NEG_INFINITY, |left, right| left.max(right));

        // Find global minimum of y_dataset (vector of y data vectors)
        let y_data_min: f32 = (&self.y_dataset).into_iter()
            .map(|y_set| y_set.into_iter()
                    .fold(f32::INFINITY, |left, &right| left.min(right))
            ).fold(f32::INFINITY, |left, right| left.min(right));

        // Find global maximum of y_dataset (vector of y data vectors)
        let mut y_data_max: f32 = (&self.y_dataset).into_iter()
            .map(|y_set| y_set.into_iter()
                    .fold(f32::NEG_INFINITY, |left, &right| left.max(right))
            ).fold(f32::NEG_INFINITY, |left, right| left.max(right));
        
        // If axes are defined as equal
        if self.axis_equal {
            // Calculate aspect ratio of plot window
            let aspect_ratio: f32 = (self.width as f32 - 2.0 * self.axis_pad as f32) / (self.height as f32 - 2.0 * self.axis_pad as f32);

            // Calculate ranges of data
            let x_range: f32 = x_data_max - x_data_min;
            let y_range: f32 = y_data_max - y_data_min;

            // Extend lacking range to equalize axis scales
            if x_range < (y_range * aspect_ratio) {
                x_data_max = y_range * aspect_ratio + x_data_min;
            } else {
                y_data_max = x_range / aspect_ratio + y_data_min;
            }
        }

        // Get bounds of x and y axes 
        let x_bounds: [f32; 2] = self.x_limits.unwrap_or([x_data_min, x_data_max]);
        let y_bounds: [f32; 2] = self.y_limits.unwrap_or([y_data_min, y_data_max]);

        // Unpack x_bounds and y_bounds
        let x_abs_min: f32 = x_bounds[0];
        let x_abs_max: f32 = x_bounds[1];
        let y_abs_min: f32 = y_bounds[0];
        let y_abs_max: f32 = y_bounds[1];

        // Draw axes
        self.draw_axes(&mut render_string, x_abs_min, x_abs_max, y_abs_min, y_abs_max, n_ticks, n_ticks);

        // Create plot window sub-image (this is done to auto-clip out of bounds data points)
        render_string.push_str(&format!(r#"<svg width="{}" height="{}" x="{}" y="{}">"#, self.width - 2 * self.axis_pad, self.height - 2 * self.axis_pad, self.axis_pad, self.axis_pad));

        // Define style for origin markers (only if x or y axis is within plot window)
        let origin_line_style: SVGStyle = SVGStyle {
            stroke_color: Color::LightGray,
            ..Default::default()
        };

        // Draw y axis if within plot window
        if (x_bounds[0] < 0.0) && (0.0 < x_bounds[1]) {
            let origin_x: i32 = ((self.width - 2 * self.axis_pad) as f32 * (-x_bounds[0] / (x_bounds[1] - x_bounds[0]))) as i32;
            draw_line(&mut render_string, origin_x, 0, origin_x, self.height - self.axis_pad, &origin_line_style);
        }

        // Draw x axis if within plot window
        if (y_bounds[0] < 0.0) && (0.0 < y_bounds[1]) {
            println!("{:?}", y_bounds);
            let origin_y: i32 = ((self.height - 2 * self.axis_pad) as f32 - (self.height - 2 * self.axis_pad) as f32 * (-y_bounds[0] / (y_bounds[1] - y_bounds[0]))) as i32;
            draw_line(&mut render_string, 0, origin_y, self.width - self.axis_pad, origin_y, &origin_line_style);
        }
        
        // Loop through each data series
        for data_idx in 0..self.x_dataset.len() {
            // Extract relevant data series
            let x_data: &Vec<f32> = &self.x_dataset[data_idx];
            let y_data: &Vec<f32> = &self.y_dataset[data_idx];

            // Map x series from plot values to pixel values
            let mapped_x: Vec<i32> = (0..x_data.len()).into_iter()
                .map(|idx| ((self.width - 2 * self.axis_pad) as f32 * (x_data[idx] - x_abs_min) / (x_abs_max - x_abs_min)) as i32)
                .collect::<Vec<i32>>();
            
            // Map y series from plot values to pixel values
            let mapped_y: Vec<i32> = (0..x_data.len()).into_iter()
                .map(|idx| (self.height - self.axis_pad) - (self.axis_pad as f32 + (self.height - 2 * self.axis_pad) as f32 * (y_data[idx] - y_abs_min) / (y_abs_max - y_abs_min)) as i32)
                .collect::<Vec<i32>>();
            
            // Draw data markers (title allows for label when hovered over in browser)
            if self.plot_styles[data_idx].has_markers {
                let point_string: String = (0..x_data.len())
                    .map(|subidx| format!(r#"<circle r="{}" cx="{}" cy="{}" fill="{}" stroke="{}" stroke-width="1"><title>({}, {})</title></circle>"#,
                        point_r,
                        mapped_x[subidx],
                        mapped_y[subidx],
                        self.plot_styles[data_idx].stroke_color,
                        self.plot_styles[data_idx].stroke_color,
                        x_data[subidx],
                        y_data[subidx],
                    )).collect::<String>();
                render_string.push_str(&point_string);
            }

            // Start polyline element
            render_string.push_str(&format!(r#"<polyline fill="none" stroke="{}" stroke-width="{}" points=" "#,
                self.plot_styles[data_idx].stroke_color,
                self.plot_styles[data_idx].stroke_width
            ));

            // Compile mapped pixel values into single string series
            let polyline_points_string: String = (0..mapped_x.len())
                .map(|subidx| format!("{},{} ", mapped_x[subidx], mapped_y[subidx]))
                .collect::<String>();

            // Push pixel string to polyline element
            render_string.push_str(&polyline_points_string);

            // Close polyline element
            render_string.push_str("\"/>");
        }

        if self.legend_names.is_some() {
            self.draw_legend(&mut render_string, Location::Northwest);
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
}

// Push line element to render_string (private function)
fn draw_line(
    render_string: &mut String,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    style: &SVGStyle
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
    x: i32,
    y: i32,
    angle: i32,
    text: &str,
    style: &SVGStyle,
    text_size: &str
) {
    render_string.push_str(
        &format!(r#"<text transform="translate({}, {}) rotate({})" color="{}" font-size="{}" dominant-baseline="middle" text-anchor="middle">{}</text>"#,
            x, y, angle, style.stroke_color, text_size, text
        )
    );
}
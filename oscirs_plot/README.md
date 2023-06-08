# oscirs_plot

[![crates.io](https://shields.io/crates/v/oscirs_plot)](https://crates.io/crates/oscirs_plot)

A plotting crate for Rust

## Description

This crate focuses on plotting data based on float vectors. It currently only supports line plots, but on those line plots you can equalize axes, set custom axis limits, add a legend, title the plot, and set axis labels.

## Use

For a quick start, import everything from svgplot_core and initialize a new figure. Use the default constructor and create a `Scatterline` object. Make the figure variable mutable, as it will be dynamically storing data series internally. 

```rust
use oscirs_plot::svgplot_core::*;

let mut figure: Scatterline = Scatterline::default();
```

Once we have a figure, we can set the axis labels and the figure title.

```rust
figure.label_x("X axis (unit)");
figure.label_y("Y axis (unit)");
figure.title("This is a plot");
```

Now we can create some data vectors and specify a style for our series to be plotted with. We'll make this mutable so we can reuse the same style object for another data series. For this example I chose to plot a square root function in blue.

```rust
let x: Vec<f32> = (0..=6)
    .map(|x| x as f32)
    .collect();
let y: Vec<f32> = x.clone()
    .into_iter()
    .map(|x| x.sqrt())
    .collect();

let mut style: PlotStyle = PlotStyle {
    stroke_color: Color::Blue,
    ..Default::default()
};

figure.add_data(&x, &y, &style)
    .expect("Failed to add data series");
```

We can also create scatter plots by specifying a stroke width of 0 and turning markers on. Lets add a scatter series of the line y=x.

```rust
let y2: Vec<f32> = x.clone();

style.stroke_color = Color::Red;
style.stroke_width = 0;
style.has_markers = true;

figure.add_data(&x, &y2, &style)
    .expect("Failed to add data series");
```

To display our plot, we just need to call `render()` on our figure and specify a file name. The .svg file will be auto-generated and will open at the end of the writing process.

```rust
figure.render("Example_Figure").expect("Failed to generate figure");
```
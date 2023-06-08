#![cfg(feature="test_plot")]

use oscirs_plot::svgplot_core::*;

#[test]
pub fn scatterline_test() {
    let mut figure: Scatterline = Scatterline::default();

    figure.label_x("X axis (unit)");
    figure.label_y("Y axis (unit)");
    figure.title("This is a plot");

    let x: Vec<f32> = (0..=8).map(|x| 2.0*x as f32).collect::<Vec<f32>>();
    let y: Vec<f32> = x.clone();
    
    let style: PlotStyle = PlotStyle {
        stroke_color: Color::Blue,
        ..Default::default()
    };

    figure.add_data(&x, &y, &style)
        .expect("Failed to add data series");

    figure.axis_equal();

    figure.render("scatterline_test").expect("Failed to generate figure");
}

#[test]
pub fn bar_test() {
    let mut figure: Bar = Bar::default();

    figure.label_y("Y axis (unit)");
    figure.title("Bar Chart");

    let labels: Vec<String> = (["Group A", "Group B", "Group C"])
        .into_iter()
        .map(|label| label.to_string())
        .collect::<Vec<String>>();
    let y: Vec<f32> = vec![2.0, 4.0, 5.0];

    let style: PlotStyle = PlotStyle {
        fill_color: Color::Red,
        ..Default::default()
    };

    figure.add_data(&labels, &y, &style)
        .expect("Failed to add data series");

    figure.set_ymax(6.0);

    figure.render("bar_test").expect("Failed to generate figure");
}

#[test]
pub fn histogram_test() {
    let mut figure: Histogram = Histogram::default();

    figure.label_x("X axis (unit)");
    figure.label_y("Count");
    figure.title("Histogram");

    let dataset: Vec<f32> = vec![1.0, 7.0, 3.0, 5.0, 7.0, 3.0, 7.0, 9.0];

    let style: PlotStyle = PlotStyle { 
        fill_color: Color::Green,
        ..Default::default()
    };

    figure.add_data(&dataset, &style);

    figure.set_xlims(2.0, 8.0);
    figure.set_block_range(2.0);
    figure.set_ymax(5);

    figure.render("hist_test").expect("Failed to generate figure");
}
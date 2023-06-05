use oscirs_plot::svgplot_core::*;

#[test]
pub fn figure_test() {
    let mut figure: SVGFigure = SVGFigure::default();

    figure.label_x("X axis (unit)");
    figure.label_y("Y axis (unit)");
    figure.title("This is a plot");

    let x: Vec<f32> = (0..=8).map(|x| 2.0*x as f32).collect::<Vec<f32>>();
    let y: Vec<f32> = x.clone();
    
    let style = SVGStyle {
        stroke_color: Color::Blue,
        ..Default::default()
    };

    figure.add_data(&x, &y, &style);

    figure.axis_equal();

    figure.render("figure_test").expect("Failed to generate figure");
}
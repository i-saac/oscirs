#![cfg(feature = "test_integration")]

use oscirs_plot::svgplot_core::*;
use oscirs_stats::summaries::Normal;
use oscirs::vector::integrate;

#[test]
fn normal_dist_test() {
    let mut figure: Scatterline = Scatterline::default();

    figure.label_x("X");
    figure.label_y("P");
    figure.title("Normal Distribution");

    let mut style: PlotStyle = PlotStyle {
        stroke_color: Color::Red,
        ..Default::default()
    };

    let normal_vals: Normal = Normal {
        mean: 0.0,
        std_dev: 1.0
    };

    let (x_vec, p_vec) = normal_vals.to_distribution();

    figure.add_data(&x_vec, &p_vec, &style)
        .expect("Failed to add Normal 1 data series");

    let integral: Vec<f32> = integrate(&x_vec, &p_vec)
        .expect("Failed to integrate p_vec with respect to x_vec");
    
    style.stroke_color = Color::Blue;

    figure.add_data(&x_vec, &integral, &style)
        .expect("Failed to add CDF 1 data series");

    let normal_vals_2 = Normal {
        mean: 2.0, 
        std_dev: 2.0
    };

    let (x_vec_2, p_vec_2) = normal_vals_2.to_distribution();

    style.stroke_color = Color::Green;

    figure.add_data(&x_vec_2, &p_vec_2, &style)
        .expect("Failed to add Normal 2 data series");

    let integral_2: Vec<f32> = integrate(&x_vec_2, &p_vec_2)
        .expect("Failed to integrate p_vec_2 with respect to x_vec_2");

    style.stroke_color = Color::Pink;

    figure.add_data(&x_vec_2, &integral_2, &style)
        .expect("Failed to add CDF 2 data series");

    figure.set_xlims(-4.5, 4.5);
    figure.set_ylims(0.0, 1.0);

    let legend_names = (["Normal 1", "CDF 1", "Normal 2", "CDF 2"])
        .map(|entry| entry.to_string())
        .to_vec();
    figure.assign_legend(&legend_names).expect("Failed to assign legend names");

    figure.render("normal_dist_test").expect("Failed to generate figure");
}
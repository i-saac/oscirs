use oscirs_stats::StatFuncs;
use oscirs_stats::summaries::{
    FiveNumber,
    Normal
};
use oscirs_plot::svgplot_core::*;
use oscirs::vector::integrate;

#[test]
fn five_number_test() {
    let input_vec: Vec<f32> = ([6, 7, 15, 36, 39, 40, 41, 42, 43, 47, 49])
        .map(|x| x as f32)
        .to_vec();

    let fns: FiveNumber = input_vec.five_number();

    assert_eq!(fns.minimum, 6.0);
    assert_eq!(fns.lower_quart, 25.5);
    assert_eq!(fns.median, 40.0);
    assert_eq!(fns.upper_quart, 42.5);
    assert_eq!(fns.maximum, 49.0);

    let input_vec_2: Vec<f32> = ([7, 15, 36, 39, 40, 41])
        .map(|x| x as f32)
        .to_vec();

    let fns_2: FiveNumber = input_vec_2.five_number();

    assert_eq!(fns_2.minimum, 7.0);
    assert_eq!(fns_2.lower_quart, 15.0);
    assert_eq!(fns_2.median, 37.5);
    assert_eq!(fns_2.upper_quart, 40.0);
    assert_eq!(fns_2.maximum, 41.0);
}

#[test]
fn normal_test() {
    let input_vec: Vec<f32> = ([6, 7, 15, 36, 39, 40, 41, 42, 43, 47, 49])
        .map(|x| x as f32)
        .to_vec();

    let normal: Normal = input_vec.normal();

    assert_eq!(normal.mean.floor(), 33.0);
    assert_eq!(normal.mean.ceil(), 34.0);
    assert_eq!(normal.std_dev.floor(), 15.0);
    assert_eq!(normal.std_dev.ceil(), 16.0);
}

#[test]
fn normal_dist_test() {
    let mut figure: SVGFigure = SVGFigure::default();

    figure.label_x("X");
    figure.label_y("P");
    figure.title("Normal Distribution");

    let mut style: SVGStyle = SVGStyle {
        stroke_color: Color::Red,
        ..Default::default()
    };

    let normal_vals: Normal = Normal {
        mean: 0.0,
        std_dev: 1.0
    };

    let (x_vec, p_vec) = normal_vals.to_distribution();

    figure.add_data(&x_vec, &p_vec, &style);

    let integral: Vec<f32> = integrate(&x_vec, &p_vec)
        .expect("Failed to integrate p_vec with respect to x_vec");
    
    style.stroke_color = Color::Blue;

    figure.add_data(&x_vec, &integral, &style);

    let normal_vals_2 = Normal {
        mean: 2.0, 
        std_dev: 2.0
    };

    let (x_vec_2, p_vec_2) = normal_vals_2.to_distribution();

    style.stroke_color = Color::Green;

    figure.add_data(&x_vec_2, &p_vec_2, &style);

    let integral_2: Vec<f32> = integrate(&x_vec_2, &p_vec_2)
        .expect("Failed to integrate p_vec_2 with respect to x_vec_2");

    style.stroke_color = Color::Pink;

    figure.add_data(&x_vec_2, &integral_2, &style);

    figure.set_xlims(-4.5, 4.5);
    figure.set_ylims(0.0, 1.0);

    let legend_names = (["Normal 1", "CDF 1", "Normal 2", "CDF 2"])
        .map(|entry| entry.to_string())
        .to_vec();
    figure.assign_legend(&legend_names);

    figure.render("normal_dist_test").expect("Failed to generate figure");
}
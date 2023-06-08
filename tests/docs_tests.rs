#[test]
#[cfg(feature = "linalg")]
fn linalg_readme() {
    use oscirs_linalg::matrix::{
        new_matrix,
        Matrix
    };
    
    let data: Vec<f32> = vec![2.0; 6];
    let n_rows: usize = 2;
    let n_cols: usize = 3;
    
    let mat: Matrix = new_matrix(data, n_rows, n_cols)
        .expect("Failed to create mat");

    let data_2: Vec<f32> = vec![3.0; 6];
    let n_rows_2: usize = 2;
    let n_cols_2: usize = 3;

    let mat_2: Matrix = new_matrix(data_2, n_rows_2, n_cols_2)
        .expect("Failed to create mat_2");

    let result: Matrix = (mat + mat_2)
        .expect("Failed to add mat and mat_2");

    assert_eq!(result.get_data(), vec![5.0; 6]);

    assert_eq!(result[[1, 1]], 5.0);

    use oscirs_linalg::calculator::{
        self,
        Calculator
    };
    
    let mut calc: Calculator = calculator::init()
        .expect("Failed to initialize Calculator");

    let a_vec: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let b_vec: Vec<f32> = vec![2.0, 1.0, 2.0, 3.0, 2.0, 1.0];

    let a_mat: Matrix = new_matrix(a_vec, 2, 3)
        .expect("Failed to create Matrix A");
    let b_mat: Matrix = new_matrix(b_vec, 3, 2)
        .expect("Failed to create Matrix B");

    let a_idx: usize = calc.store_matrix(a_mat)
        .expect("Failed to store Matrix A in calculator memory");
    let b_idx: usize = calc.store_matrix(b_mat)
        .expect("Failed to store Matrix B in calculator memory");

    let (c_mat, _c_idx) = calc.mat_mul(a_idx, b_idx)
        .expect("Failed to mulitply Matrix A and Matrix B");
    
    assert_eq!(c_mat.get_data(), vec![12.0, 10.0, 30.0, 25.0], "Matrix C data not as expected");
    assert_eq!(c_mat.get_rows(), 2, "Matrix C row dimension not as expected");
    assert_eq!(c_mat.get_cols(), 2, "Matrix C col dimension not as expected");
}

#[test]
#[cfg(feature = "plot")]
fn plot_readme() {
    use oscirs_plot::svgplot_core::*;

    let mut figure: Scatterline = Scatterline::default();

    figure.label_x("X axis (unit)");
    figure.label_y("Y axis (unit)");
    figure.title("This is a plot");

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

    let y2: Vec<f32> = x.clone();

    style.stroke_color = Color::Red;
    style.stroke_width = 0;
    style.has_markers = true;
        
    figure.add_data(&x, &y2, &style)
        .expect("Failed to add data series");

    figure.render("Example_Figure").expect("Failed to generate figure");
}

#[test]
#[cfg(feature = "stats")]
fn stats_readme() {
    use oscirs_stats::summaries_core::*;

    let input_vec: Vec<f32> = vec![6.0, 7.0, 15.0, 36.0, 39.0, 40.0, 41.0, 42.0, 43.0, 47.0, 49.0];

    println!("{:?}", input_vec.normal());
    println!("{:?}", input_vec.sample());
    println!("{:?}", input_vec.five_number());
}
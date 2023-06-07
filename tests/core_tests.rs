use oscirs::vector::{
    integrate,
    linterp
};

#[test]
fn integral_test() {
    let x_vec: Vec<f32> = (0..=10).map(|x| x as f32).collect();

    let integral: Vec<f32> = integrate(&x_vec, &x_vec).expect("Failed to integrate");
    
    assert_eq!(integral[integral.len() - 1], 50.0, "Integral value not as expected");
}

#[test]
fn linterp_test() {
    let x_vec: Vec<f32> = (0..=10).map(|x| x as f32).collect();
    let y_vec: Vec<f32> = (0..=10).map(|x| x as f32 * 2.0).collect();

    let guess: f32 = linterp(&x_vec, &y_vec, 8.5).expect("Failed to linearly interpolate (1)");

    assert_eq!(guess, 17.0, "Linear interpolation 1 not as expected");

    let inv_guess: f32 = linterp(&y_vec, &x_vec, guess).expect("Failed to linearly interpolate (2)");

    assert_eq!(inv_guess, 8.5, "Linear interpolation 2 not as expected");
}
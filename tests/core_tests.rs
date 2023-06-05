use oscirs::vector;

#[test]
fn integral_test() {
    let x_vec: Vec<f32> = (0..=10).map(|x| x as f32).collect();

    let integral: Vec<f32> = vector::integrate(&x_vec, &x_vec).expect("Failed to integrate");
    
    assert_eq!(integral[integral.len() - 1], 50.0, "Integral value not as expected");
}
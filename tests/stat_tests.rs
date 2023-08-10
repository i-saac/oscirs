use oscirs_stats::StatFuncs;
use oscirs_stats::summaries::{
    FiveNumber,
    Normal,
    Sample
};

use oscirs::stats::t_test::*;

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
fn t_test_test() {
    let input_vec: Vec<f32> = ([6, 7, 15, 36, 39, 40, 41, 42, 43, 47, 49])
        .map(|x| x as f32)
        .to_vec();

    let sample_summary: Sample = input_vec.sample();

    let prob: f32 = single_t_test(42.0, sample_summary, TTestType::TestNotEqual)
        .expect("Failed to perform single sample t test");

    assert!(0.095 < prob && prob < 0.098, "Probability not as expected");
}
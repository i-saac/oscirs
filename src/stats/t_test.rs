//! Module containing functions required to perform t-tests

extern crate oscirs_stats;

use oscirs_stats::summaries::Sample;

use crate::Result;
use crate::math::beta;
use crate::vector::{
    integrate,
    linterp
};

/// Enum for type of t test to perform
pub enum TTestType {
    TestLess,
    TestGreater,
    TestNotEqual
}

fn t_dist(dof: f32) -> (Vec<f32>, Vec<f32>) {
    let ind: Vec<f32> = (-6000..=6000)
        .map(|x| x as f32 / 1000.0)
        .collect();

    let coeff: f32 = 1.0 / (dof.sqrt() * beta(0.5, 0.5 * dof));

    let pdf: Vec<f32> = (&ind).into_iter()
        .map(|x| coeff * (1.0 + x * x / dof).powf(-0.5 * (dof + 1.0)))
        .collect();

    return (ind, pdf)
}

/// Performs single-sample t-test (ACCURACY IS STILL BEING IMPROVED)
pub fn single_t_test(test_mean: f32, sample_params: Sample, test_type: TTestType) -> Result<f32> {
    let dof = sample_params.sample_size - 1;

    let (ind, pdf) = t_dist(dof as f32);

    let cdf: Vec<f32> = integrate(&ind, &pdf)?;

    let test_stat: f32 = (sample_params.sample_mean - test_mean) / (sample_params.sample_std_dev / (sample_params.sample_size as f32).sqrt());

    let prob: f32 = match test_type {
        TTestType::TestLess =>
            linterp(&ind, &cdf, test_stat)?,
        TTestType::TestGreater =>
            1.0 - linterp(&ind, &cdf, test_stat)?,
        TTestType::TestNotEqual =>
            2.0 * (1.0 - linterp(&ind, &cdf, test_stat.abs())?)
    };

    Ok(prob)
}
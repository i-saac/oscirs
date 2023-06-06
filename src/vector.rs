//! Useful functions and implementations for standard vectors

use crate::{
    ApplyFunction,
    SciResult
};
use crate::err::SciError;

impl<T> ApplyFunction<T> for Vec<T> {
    fn apply_fn(&self, fn_to_apply: Box<dyn Fn(&T) -> T>) -> Self {
        self.into_iter()
            .map(|x| fn_to_apply(x))
            .collect::<Vec<T>>()
    }
}

/// Discrete trapezoidal integration of dep with respect to ind
/// 
/// Assumes sorted inputs
pub fn integrate(ind: &Vec<f32>, dep: &Vec<f32>) -> SciResult<Vec<f32>> {
    if ind.len() != dep.len() {
        return Err(SciError::VectorLengthsError)
    }

    let mut output_vec: Vec<f32> = Vec::with_capacity(ind.len());
    output_vec.push(0.0);

    for idx in 1..ind.len() {
        output_vec.push(output_vec[idx - 1] + 0.5 * (dep[idx] + dep[idx - 1]) * (ind[idx] - ind[idx -1]));
    }

    Ok(output_vec)
}

/// Estimate value of dependent at input extrapolated from ind and dep
/// 
/// Assumes sorted inputs
pub fn linterp(ind: &Vec<f32>, dep: &Vec<f32>, input: f32) -> SciResult<f32> {
    if ind.len() != dep.len() {
        return Err(SciError::VectorLengthsError)
    }

    if ind[0] > input || input > ind[ind.len() - 1] {
        return Err(SciError::RangeError)
    }

    let mut idx: usize = 0;
    while ind[idx] < input { idx += 1 }

    let less_idx: usize = idx - 1;

    let d_ind: f32 = ind[idx] - ind[less_idx];
    let d_dep: f32 = dep[idx] - dep[less_idx];

    let guess_dep: f32 = dep[less_idx] + (d_dep / d_ind) * (input - ind[less_idx]);

    Ok(guess_dep)
}
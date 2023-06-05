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
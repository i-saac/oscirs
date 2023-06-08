//! Statistical summary structs and implementations

use std::f32::consts::PI;

/// Five Number Summary struct
#[derive(Debug)]
pub struct FiveNumber {
    pub minimum: f32,
    pub lower_quart: f32,
    pub median: f32,
    pub upper_quart: f32,
    pub maximum: f32
}

/// Normal Distribution description struct
#[derive(Debug)]
pub struct Normal {
    pub mean: f32,
    pub std_dev: f32
}

/// Sample property description struct
#[derive(Debug)]
pub struct Sample {
    pub sample_mean: f32,
    pub sample_std_dev: f32,
    pub sample_size: usize
}

impl Normal {
    /// Convert normal distribution into discrete vectors
    pub fn to_distribution(&self) -> (Vec<f32>, Vec<f32>) {
        let n_samples: i32 = 100;

        let mn: f32 = self.mean;
        let sd: f32 = self.std_dev;

        let min_ind: f32 = self.mean - 4.0 * self.std_dev;
        let max_ind: f32 = self.mean + 4.0 * self.std_dev;

        let map_ind: Vec<f32> = (0..n_samples).map(|ind| min_ind + (max_ind - min_ind) * (ind as f32) / (n_samples as f32 - 1.0)).collect();

        let dist_prop: Vec<f32> = (&map_ind).into_iter()
            .map(
                |x| {
                    (1.0 / (sd * (2.0 * PI).sqrt())) * (-0.5 * ((x - mn) / sd).powi(2)).exp()
                }
            ).collect();

        return (map_ind, dist_prop);
    }
}
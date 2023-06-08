//! Statistical analysis implmentations for standard vectors

use crate::StatFuncs;
use crate::summaries::{
    FiveNumber,
    Normal,
    Sample
};

impl StatFuncs for Vec<f32> {
    fn mean(&self) -> f32 {
        let sum: f32 = self.into_iter().sum();

        sum / (self.len() as f32)
    }

    fn normal(&self) -> Normal {
        let mean: f32 = self.mean();

        let std_dev: f32 = self.into_iter()
            .map(|x| (x - mean).powi(2) / self.len() as f32)
            .sum::<f32>()
            .sqrt();

        Normal { mean: mean, std_dev: std_dev }
    }

    fn sample(&self) -> Sample {
        let mean: f32 = self.mean();

        let sample_size: usize = self.len();

        let std_dev: f32 = self.into_iter()
            .map(|x| (x - mean).powi(2) / (sample_size - 1) as f32)
            .sum::<f32>()
            .sqrt();

        Sample { sample_mean: mean, sample_std_dev: std_dev, sample_size: sample_size }
    }
    
    fn five_number(&self) -> FiveNumber {
        let mut sorted_vec: Vec<f32> = self.clone();
        sorted_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let last_idx: usize = sorted_vec.len() - 1;

        let minimum: f32 = sorted_vec[0];
        let maximum: f32 = sorted_vec[last_idx];
        
        let median_idx: f32 = (last_idx as f32) / 2.0;
        let median: f32 = (sorted_vec[median_idx.floor() as usize] + sorted_vec[median_idx.ceil() as usize]) / 2.0;

        let lq_idx: f32 = median_idx.floor() / 2.0;
        let lower_quart: f32 = (sorted_vec[lq_idx.floor() as usize] + sorted_vec[lq_idx.ceil() as usize]) / 2.0;

        let uq_idx: f32 = last_idx as f32 - lq_idx;
        let upper_quart: f32 = (sorted_vec[uq_idx.floor() as usize] + sorted_vec[uq_idx.ceil() as usize]) / 2.0;

        FiveNumber { minimum: minimum, lower_quart: lower_quart, median: median, upper_quart: upper_quart, maximum: maximum }
    }
}
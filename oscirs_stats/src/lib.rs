//! [![crates.io](https://shields.io/crates/v/oscirs_stats)](https://crates.io/crates/oscirs_stats)
//! 
//! A statistical analysis crate for Rust

pub mod stats_vec;
pub mod summaries;

/// Trait containing some generic statistical analysis functions
pub trait StatFuncs {
    /// Get arithmetic mean of an object
    fn mean(&self) -> f32;
    /// Get arithmetic mean and standard deviation of an object
    fn normal(&self) -> summaries::Normal;
    /// Get five-number summary of an object (inclusive of median in quartile calculations)
    fn five_number(&self) -> summaries::FiveNumber;
}
//! [![crates.io](https://shields.io/crates/v/oscirs)](https://crates.io/crates/oscirs)
//! 
//! A scientific computing environment for Rust

use std::error::Error;
use std::result;

#[cfg(feature = "linalg")]
extern crate oscirs_linalg;

#[cfg(feature = "plot")]
extern crate oscirs_plot;

#[cfg(feature = "stats")]
extern crate oscirs_stats;

pub mod err;
pub mod math;
pub mod vector;

#[cfg(feature = "stats")]
pub mod stats;

/// Custom result type
pub type Result<T> = result::Result<T, Box<dyn Error>>;

/// Macro for executing a closure on an object more easily
pub trait ApplyFunction<T> {
    fn apply_fn(&self, fn_to_apply: Box<dyn Fn(&T) -> T>) -> Self;
}
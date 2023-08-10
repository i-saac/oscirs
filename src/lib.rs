//! [![crates.io](https://shields.io/crates/v/oscirs)](https://crates.io/crates/oscirs)
//! 
//! A scientific computing environment for Rust

use std::error::Error;
use std::result;

extern crate oscirs_linalg;
extern crate oscirs_plot;
extern crate oscirs_stats;

pub mod err;
pub mod math;
pub mod vector;

pub mod stats;

/// Custom result type
pub type Result<T> = result::Result<T, Box<dyn Error>>;

/// Macro for executing a closure on an object more easily
pub trait ApplyFunction<T> {
    fn apply_fn(&self, fn_to_apply: Box<dyn Fn(&T) -> T>) -> Self;
}
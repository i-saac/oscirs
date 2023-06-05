//! [![crates.io](https://shields.io/crates/v/oscirs)](https://crates.io/crates/oscirs)
//! 
//! A scientific computing environment for Rust

extern crate oscirs_linalg;
extern crate oscirs_plot;
extern crate oscirs_stats;

pub mod err;
pub mod vector;

/// Custom result type using SciError
pub type SciResult<T> = Result<T, err::SciError>;

/// Macro for executing a closure on an object more easily
pub trait ApplyFunction<T> {
    fn apply_fn(&self, fn_to_apply: Box<dyn Fn(&T) -> T>) -> Self;
}
extern crate oscirs_linalg;
extern crate oscirs_plot;
extern crate oscirs_stats;

pub mod err;
pub mod vector;

pub type SciResult<T> = Result<T, err::SciError>;

pub trait ApplyFunction<T> {
    fn apply_fn(&self, fn_to_apply: Box<dyn Fn(&T) -> T>) -> Self;
}
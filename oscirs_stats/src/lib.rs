pub mod stats_vec;
pub mod summaries;

pub trait StatFuncs {
    fn mean(&self) -> f32;
    fn normal(&self) -> summaries::Normal;
    fn five_number(&self) -> summaries::FiveNumber;
}
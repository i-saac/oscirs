//! [![crates.io](https://shields.io/crates/v/oscirs_linalg)](https://crates.io/crates/oscirs_linalg)
//! 
//! A linear algebra crate for Rust

use std::result;

use err::LAError;

pub mod err;
pub mod calculator;
pub mod matrix;
mod memory;

/// Custom result type
pub type Result<T> = result::Result<T, LAError>;

/// Default amount of memory slots for matrices in MemoryHandler and Calculator
const INIT_MEMORY_CAPACITY: usize = 3;

/// List of default kernel names
const PROGRAM_LIST: [&str; 1] = [
    "mat_mul"
];

/// Source code for default kernels
const PROGRAM_SOURCE: &str = r#"
kernel void mat_mul (
    global float* c,
    const int N,
    const int K,
    const global float* a,
    const global float* b
) {
    const int globalRow = get_global_id(0);
    const int globalCol = get_global_id(1);

    float interm = 0.0f;
    for (int k = 0; k < K; k++) {
        interm += a[globalRow * K + k] * b[k * N + globalCol];
    }

    c[globalRow * N + globalCol] = interm;
}
"#;
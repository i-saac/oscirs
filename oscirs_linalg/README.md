# oscirs-linalg

## Description

This crate allows for simple and easy linear algebra-related calculations using GPU parallelization through OpenCL 3.0.

This crate relies upon the [opencl3](https://crates.io/crates/opencl3) crate which provides a rust implementation of OpenCL 3.0 macros.

Currently, this crate does require a GPU to run linear algebra-related code, although this issue will be resolved in the future.

## Use

rustolla primarily relies upon the `Matrix` struct. You can create one using a vector of 32-bit floats in addition to a number of rows and columns whose product adds up to the length of the vector. The constructor `new_matrix` checks to make sure this is the case before successfully returning a Matrix. Data vectors are row-major. Example code can be seen below.

```rust
use rustolla::matrix::{
    self,
    Matrix
};

let data: Vec<f32> = vec![2.0; 6];
let n_rows: usize = 2;
let n_cols: usize = 3;

let mat: Matrix = matrix::new_matrix(data, n_rows, n_cols)
    .expect("Failed to create Matrix");
```

Further clarification on use will be provided in the future.
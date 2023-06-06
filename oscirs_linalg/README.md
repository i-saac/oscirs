# oscirs-linalg

[![crates.io](https://shields.io/crates/v/oscirs_linalg)](https://crates.io/crates/oscirs_linalg)

A linear algebra crate for Rust

## Description

This crate allows for simple and easy linear algebra-related calculations and supports GPU parallelization through OpenCL 3.0.

This crate relies upon the [opencl3](https://crates.io/crates/opencl3) crate which provides a rust implementation of OpenCL 3.0 macros.

## Use

oscirs_linalg primarily relies upon the `Matrix` struct. You can create one using a vector of 32-bit floats in addition to a number of rows and columns whose product adds up to the length of the vector. The constructor `new_matrix` checks to make sure this is the case before successfully returning a Matrix. Data vectors are row-major. Example code can be seen below.

```rust
use oscirs_linalg::matrix::{
    new_matrix,
    Matrix
};

let data: Vec<f32> = vec![2.0; 6];
let n_rows: usize = 2;
let n_cols: usize = 3;

let mat: Matrix = new_matrix(data, n_rows, n_cols)
    .expect("Failed to create mat");
```

Matrices can be added together using typical arithmetic operators just like ints or floats. Matrices can also be negated or added/subtracted with floats. When an operation requires dimension checks, you should use the `?` operator or `expect()` to handle the error.

```rust
let data_2: Vec<f32> = vec![3.0; 6];
let n_rows_2: usize = 2;
let n_cols_2: usize = 3;

let mat_2: Matrix = new_matrix(data_2, n_rows_2, n_cols_2)
    .expect("Failed to create mat_2");

let result: Matrix = (mat + mat_2)
    .expect("Failed to add mat and mat_2");

assert_eq!(result.get_data(), vec![5.0; 6]);
```

Matrices can be indexed using nested square brackets, and individual rows/cols can be indexed using the `row()` and `col()` methods respectively.

```rust
assert_eq!(result[[1, 1]], 5.0);
```

## GPU Acceleration

While matrices can be multiplied through `A * B` syntax, this is a single-threaded CPU-based operation. For large matrices, this quickly becomes inefficient. oscirs_linalg supports GPU-based parallelized matrix multiplication through OpenCL.

To start with parallization, first initialize the `Calculator` struct through the `init()` function. Make sure to declare it as mutable, as it will be dynamically storing matrices internally. 

```rust
use oscirs_linalg::calculator::{
    self,
    Calculator
};

let mut calc: Calculator = calculator::init()
    .expect("Failed to initialize Calculator");
```

Matrices must be stored in the calculator's memory before any operations can be performed. The `store_matrix()` method returns a memory index that will be used to reference that matrix for the multiplication operation.

```rust
let a_vec: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
let b_vec: Vec<f32> = vec![2.0, 1.0, 2.0, 3.0, 2.0, 1.0];

let a_mat: Matrix = new_matrix(a_vec, 2, 3)
    .expect("Failed to create Matrix A");
let b_mat: Matrix = new_matrix(b_vec, 3, 2)
    .expect("Failed to create Matrix B");

let a_idx: usize = calc.store_matrix(a_mat)
    .expect("Failed to store Matrix A in calculator memory");
let b_idx: usize = calc.store_matrix(b_mat)
    .expect("Failed to store Matrix B in calculator memory");
```

Once both matrices are stored in the calculator's memory, you can call the `mat_mul()` method on `Calculator` to multiply the two matrices. The arguments to `mat_mul()` are the memory indices of the matrices to mulitply.

```rust
let (c_mat, _c_idx) = calc.mat_mul(a_idx, b_idx)
    .expect("Failed to mulitply Matrix A and Matrix B");

assert_eq!(c_mat.get_data(), vec![12.0, 10.0, 30.0, 25.0], "Matrix C data not as expected");
assert_eq!(c_mat.get_rows(), 2, "Matrix C row dimension not as expected");
assert_eq!(c_mat.get_cols(), 2, "Matrix C col dimension not as expected");
```

`mat_mul()` returns a tuple of the resultant matrix and its index in memory. The resultant matrix is always stored in the calculator's memory so that subsequent calculations can be performed faster and with less memory shuffling.

### Custom OpenCL Kernels
oscirs_linalg also supports using your own OpenCL kernels with the memory management tools provided by `Calculator`. This gets a bit complicated and involves some unsafe functions, but an example is given in the tests folder under linalg_tests.rs. It requires the creation of a custom closure that calculates the output matrix dimensions and work sizes from the input matrices, but once you do that it is easy to execute your custom kernel as many times as you want.
# oscirs_stats

[![crates.io](https://shields.io/crates/v/oscirs_stats)](https://crates.io/crates/oscirs_stats)

A statistical analysis crate for Rust

## Description

This crate allows for some basic statistical analysis such as finding the mean, standard deviation or five number summary of a data vector.

## Use

To get some quick data summaries, start by importing the following quick-start module and declaring a data vector.

```rust
use oscirs_stats::summaries_core::*;

let input_vec: Vec<f32> = vec![6.0, 7.0, 15.0, 36.0, 39.0, 40.0, 41.0, 42.0, 43.0, 47.0, 49.0];
```

To get the mean and standard deviation of the data, call the `normal()` method.

```rust
println!("{:?}", input_vec.normal());
```

To get the sample mean, sample standard deviation, and sample size, call the `sample()` method.

```rust
println!("{:?}", input_vec.sample());
```

To get the five-number summary of the data, call the `five_number()` method.

```rust
println!("{:?}", input_vec.five_number());
```

Each of these methods returns a struct that wraps all the data into a clean type, with public fields that can be read at any time.
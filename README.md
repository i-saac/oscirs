# oscirs

[![crates.io](https://shields.io/crates/v/oscirs)](https://crates.io/crates/oscirs)

A scientific computing environment for Rust

## Description and Goals

oscirs (pronounced like the Egyptian deity Osiris) is my attempt at a scientific computing environment for Rust. The goal of this project is to keep syntax intuitive and usable, not to offer the best performance possible and deepest featureset for each subcrate. If you need more power for a specific problem, I highly recommend using another crate specifically designed for that problem. Below are some overviews of the functionality offered by various subcrates. Please refer to their respective README files for example syntax.

## oscirs_linalg

This crate focuses on Linear Algebra via the `Matrix` struct.

## oscirs_plot

This crate focuses on plotting data.

## oscirs_stats

This crate focuses on statistical analysis.

## Use

The parent oscirs crate focuses on wrapping some utility functions with interactions between the three crates listed above. For example, you can perform a t-test on a sample using oscirs in combination with oscirs_stats.

### Features

oscirs as a crate is still being worked on, so some features are subject to change. For example, the beta and gamma functions are not as accurate as I would like them to be, which influences the output of the t-test probabilities. Please bear with me as I work through these growing pains.
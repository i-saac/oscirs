//! Module containing some mathematical functions

/// Euler's constant (gamma)
const EULER_CONSTANT: f32 = 0.5772156649;

/// Returns the natural log of the gamma function (ACCURACY IS STILL BEING IMPROVED)
pub fn lngamma(z: f32) -> f32 {
    let sum: f32 = (1..=10000)
        .map(|k| (z / k as f32) - (1.0 + z / k as f32).ln())
        .sum();

    return sum - EULER_CONSTANT * z - z.ln()
}

/// Returns the beta function of given parameters (ACCURACY IS STILL BEING IMPROVED)
pub fn beta(z1: f32, z2: f32) -> f32 {
    return (lngamma(z1) + lngamma(z2) - lngamma(z1 + z2)).exp()
}
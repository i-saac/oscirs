//! Matrix object and implementations

use std::ops;

use crate::LAResult;
use crate::err::LAError;

/// Matrix object definition
#[derive(Debug, Clone)]
pub struct Matrix {
    data: Vec<f32>, // Data vector
    rows: usize, // Number of rows
    cols: usize // Number of columns
}

/// Create new matrix (includes checking for matching number of elements)
pub fn new_matrix(input_data: Vec<f32>, rows: usize, cols: usize) -> LAResult<Matrix> {
    let vec_len: usize = input_data.len(); // Get input_data length
    let comp_len: usize = rows * cols; // Get projected matrix size
    
    if vec_len == comp_len { // If provided row and col values are reasonable
        // Create and return new matrix struct
        let output: Matrix = Matrix { data: input_data, rows: rows, cols: cols };
        Ok(output)
    }
    else { // If provided row and col values are not reasonable
        // Return SizeError
        return Err(LAError::SizeError);
    }
}

impl Matrix {
    /// Get number of rows in matrix
    pub fn get_rows(&self) -> usize {
        return self.rows
    }

    /// Get number of columns in matrix
    pub fn get_cols(&self) -> usize {
        return self.cols
    }

    /// Resize matrix dimensions (must have same total number of elements)
    pub fn resize(&mut self, new_rows: usize, new_cols: usize) -> LAResult<()> {
        if (new_rows * new_cols) != self.data.len() {
            return Err(LAError::ResizeError)
        }

        self.rows = new_rows;
        self.cols = new_cols;

        Ok(())
    }

    /// Get data of matrix in row-major vector form
    pub fn get_data(&self) -> Vec<f32> {
        return self.data.clone()
    }

    /// Update matrix data (must have same total number of elements)
    pub fn update_data(&mut self, new_data: Vec<f32>) -> LAResult<()> {
        if (self.rows * self.cols) != new_data.len() {
            return Err(LAError::DataUpdateError)
        }

        self.data = new_data;

        Ok(())
    }
}

// Get matrix value at row, col index
impl ops::Index<[usize; 2]> for Matrix {
    type Output = f32;

    fn index(&self, idx: [usize; 2]) -> &f32 {
        let linear_index: usize = self.cols * idx[0] + idx[1];
        
        &self.data[linear_index]
    }
}

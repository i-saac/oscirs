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

    // Return transpose of self
    pub fn transpose(&self) -> Matrix {
        let mut transpose_data: Vec<f32> = Vec::with_capacity(self.rows * self.cols); // Initialize vector for transpose data

        for transpose_row in 0..self.cols { // Loop through original cols (transpose rows)
            for transpose_col in 0..self.rows { // Loop through original rows (transpose cols)
                transpose_data.push(self[[transpose_col, transpose_row]]); // Store mat[i, j] at transpose[j, i]
            }
        }

        Matrix { data: transpose_data, rows: self.cols, cols: self.rows } // Create and return transpose matrix
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

// Add f32 to Matrix
impl ops::Add<f32> for Matrix {
    type Output = Matrix; // Declare that we are returning a matrix

    fn add(self, rhs: f32) -> Matrix {
        let mut output_data: Vec<f32> = self.data; // Copy matrix data to mutable output vector
        
        for item in &mut output_data { // Iterate over mutable output vector
            *item += rhs; // Add float to each value in output vector
        }

        Matrix { data: output_data, rows: self.rows, cols: self.cols } // Create and return new matrix struct
    }
}

// Add Matrix to Matrix
impl ops::Add<Matrix> for Matrix {
    type Output = Matrix; // Declare that we are returning a matrix

    fn add(self, rhs: Matrix) -> Matrix {
        if (self.rows == rhs.rows) && (self.cols == rhs.cols) { // If matrix dimensions are identical
            let n_elements: usize = self.rows * self.cols; // Calculate number of elements to allocate for / loop over
            let mut output_data: Vec<f32> = Vec::with_capacity(n_elements); // Allocate empty vector of length n_elements

            for element_index in 0..n_elements { // Iterate from index 0 to index n_elements-1
                output_data.push(self.data[element_index] + rhs.data[element_index]); // Append sum of left and right values at each index to output_data
            }

            Matrix { data: output_data, rows: self.rows, cols: self.cols } // Create and return new matrix struct
        }
        else { // If matrix dimensions are not identical
            panic!("Non-Identical dimensions for matrix addition") // Crash
        }
    }
}

// Negate Matrix
impl ops::Neg for Matrix {
    type Output = Matrix;

    fn neg(self) -> Matrix {
        let n_elements: usize = self.rows * self.cols; // Calculate number of elements to allocate for / loop over
        let mut output_data: Vec<f32> = Vec::with_capacity(n_elements); // Allocate empty vector of length n_elements
        
        for element_index in 0..n_elements { // Iterate from index 0 to index n_elements-1
            output_data.push(-self.data[element_index]); // Append negated values at each index to output_data
        }

        Matrix { data: output_data, rows: self.rows, cols: self.cols } // Create and return new Matrixrix struct
    }
}

// Subtract f32 from Matrix
impl ops::Sub<f32> for Matrix {
    type Output = Matrix; // Declare that we are returning a Matrixrix

    fn sub(self, rhs: f32) -> Matrix {
        self + -rhs // Utilize previously declared addition overloads
    }
}

// Subtract Matrix from Matrix
impl ops::Sub<Matrix> for Matrix {
    type Output = Matrix; // Declare that we are returning a Matrixrix

    fn sub(self, rhs: Matrix) -> Matrix {
        self + -rhs // Utilize previously declared addition overloads
    }
}

// Multiply f32 by Matrix
impl ops::Mul<Matrix> for f32 {
    type Output = Matrix; // Declare that we are returning a Matrixrix

    fn mul(self, rhs: Matrix) -> Matrix {
        let mut output_data: Vec<f32> = rhs.data; // Copy Matrixrix data to mutable output vector
        
        for item in &mut output_data { // Iterate over mutable output vector
            *item *= self; // Multiply float to each value in output vector
        }

        Matrix { data: output_data, rows: rhs.rows, cols: rhs.cols } // Create and return new Matrixrix struct
    }
}

// Multiply Matrix by Matrix
impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix; // Declare that we are returning a Matrixrix

    fn mul(self, rhs: Matrix) -> Matrix {
        if self.cols == rhs.rows { // If dimensions are compatible for Matrixrix multiplication
            let new_n_elements: usize = self.rows * rhs.cols; // Calculate number of elements to allocate for / loop over
            let mut output_data: Vec<f32> = Vec::with_capacity(new_n_elements); // Allocate empty vector of length n_elements

            for lhs_row in 0..self.rows { // Loop through all rows of left Matrixrix
                for rhs_col in 0..rhs.cols { // Loop through all columns of right Matrixrix
                    let mut dot_prod: f32 = 0.0; // Allocate f32 for the dot product result

                    for dot_index in 0..self.cols { // Loop through each element of selected subvectors
                        dot_prod += self[[lhs_row, dot_index]] * rhs[[dot_index, rhs_col]]; // Calculate dot product
                    }

                    output_data.push(dot_prod); // Append dot product to output_data
                }
            }

            Matrix { data: output_data, rows: self.rows, cols: rhs.cols } // Create and return new Matrixrix struct
        }
        else { // If not compatible
            panic!("Incompatible dimensions for Matrixrix multiplication") // Crash
        }
    }
}
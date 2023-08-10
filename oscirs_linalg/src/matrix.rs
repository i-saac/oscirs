//! Matrix object and implementations

use std::ops;

use crate::Result;
use crate::err::LAError;

/// Matrix object definition
#[derive(Debug, Clone)]
pub struct Matrix {
    data: Vec<f32>, // Data vector
    rows: usize, // Number of rows
    cols: usize // Number of columns
}

impl Matrix {
    /// Create new matrix (includes checking for matching number of elements)
    pub fn new(input_data: Vec<f32>, rows: usize, cols: usize) -> Result<Matrix> {
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

    /// Get number of rows in matrix
    pub fn get_rows(&self) -> usize {
        return self.rows
    }

    /// Get number of columns in matrix
    pub fn get_cols(&self) -> usize {
        return self.cols
    }

    /// Resize matrix dimensions (must have same total number of elements)
    pub fn resize(&mut self, new_rows: usize, new_cols: usize) -> Result<()> {
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
    pub fn update_data(&mut self, new_data: Vec<f32>) -> Result<()> {
        if (self.rows * self.cols) != new_data.len() {
            return Err(LAError::DataUpdateError)
        }

        self.data = new_data;

        Ok(())
    }

    /// Return selected row of self
    pub fn row(&self, row_idx: usize) -> Result<Vec<f32>> {
        if row_idx >= self.rows {
            return Err(LAError::IndexError)
        }

        let lower_data_idx: usize = row_idx * self.cols;
        let upper_data_idx: usize = (row_idx + 1) * self.cols;

        let output_row: Vec<f32> = (lower_data_idx..upper_data_idx)
            .map(|idx| self.data[idx])
            .collect();

        Ok(output_row)
    }

    /// Return selected col of self
    pub fn col(&self, col_idx: usize) -> Result<Vec<f32>> {
        if col_idx >= self.cols {
            return Err(LAError::IndexError)
        }

        let output_col: Vec<f32> = (0..self.rows)
            .map(|row| self.data[row * self.cols + col_idx])
            .collect();

        Ok(output_col)
    }

    /// Return transpose of self
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
    type Output = Matrix;

    fn add(self, rhs: f32) -> Matrix {
        let mut output_data: Vec<f32> = self.data.clone();
        
        for item in &mut output_data {
            *item += rhs;
        }

        Matrix { data: output_data, rows: self.rows, cols: self.cols }
    }
}

// Add Matrix to Matrix
impl ops::Add<Matrix> for Matrix {
    type Output = Result<Matrix>;

    fn add(self, rhs: Matrix) -> Result<Matrix> {
        if (self.rows != rhs.rows) || (self.cols != rhs.cols) {
            return Err(LAError::SizeError)
        }

        let n_elements: usize = self.rows * self.cols;
        let mut output_data: Vec<f32> = Vec::with_capacity(n_elements);

        for element_index in 0..n_elements {
            output_data.push(self.data[element_index] + rhs.data[element_index]);
        }

        Ok(Matrix { data: output_data, rows: self.rows, cols: self.cols })
    }
}

// Add Matrix to f32
impl ops::Add<Matrix> for f32 {
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Matrix {
        rhs + self
    }
}

// Negate Matrix
impl ops::Neg for Matrix {
    type Output = Matrix;

    fn neg(self) -> Matrix {
        let n_elements: usize = self.rows * self.cols;
        let mut output_data: Vec<f32> = Vec::with_capacity(n_elements);
        
        for element_index in 0..n_elements {
            output_data.push(-self.data[element_index]);
        }

        Matrix { data: output_data, rows: self.rows, cols: self.cols }
    }
}

// Subtract f32 from Matrix
impl ops::Sub<f32> for Matrix {
    type Output = Matrix;

    fn sub(self, rhs: f32) -> Matrix {
        self + -rhs
    }
}

// Subtract Matrix from Matrix
impl ops::Sub<Matrix> for Matrix {
    type Output = Result<Matrix>;

    fn sub(self, rhs: Matrix) -> Result<Matrix> {
        self + -rhs
    }
}

// Subtract Matrix from f32
impl ops::Sub<Matrix> for f32 {
    type Output = Matrix;

    fn sub(self, rhs: Matrix) -> Matrix {
        self + -rhs
    }
}

// Multiply f32 by Matrix
impl ops::Mul<Matrix> for f32 {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        let mut output_data: Vec<f32> = rhs.data.clone();
        
        for item in &mut output_data {
            *item *= self;
        }

        Matrix { data: output_data, rows: rhs.rows, cols: rhs.cols }
    }
}

// Multiply Matrix by Matrix
impl ops::Mul<Matrix> for Matrix {
    type Output = Result<Matrix>;

    fn mul(self, rhs: Matrix) -> Result<Matrix> {
        if self.cols != rhs.rows {
            return Err(LAError::SizeError)
        }
        let new_n_elements: usize = self.rows * rhs.cols;
        let mut output_data: Vec<f32> = Vec::with_capacity(new_n_elements);

        for lhs_row in 0..self.rows {
            for rhs_col in 0..rhs.cols {
                let mut dot_prod: f32 = 0.0;

                for dot_index in 0..self.cols {
                    dot_prod += self[[lhs_row, dot_index]] * rhs[[dot_index, rhs_col]];
                }

                output_data.push(dot_prod);
            }
         }

        Ok(Matrix { data: output_data, rows: self.rows, cols: rhs.cols })
    }
}
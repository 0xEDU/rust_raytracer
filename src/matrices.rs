use std::fmt;
use std::ops;

/* Matrix declaration and implementation ==================================== */
#[derive(Clone, Copy)]
pub struct Matrix {
    pub data: [[f64; 4]; 4],
}

impl Matrix {
    pub fn new() -> Self {
        Matrix {
            data: [[0.0; 4]; 4],
        }
    }
}
/* ========================================================================== */

/* Operator overloads for Matrix ============================================ */
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for &value in row {
                write!(f, "{:10.4} ", value)?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::cmp::PartialEq<Matrix> for Matrix {
    fn eq(&self, rhs: &Self) -> bool {
        self.data == rhs.data
    }
}

impl std::cmp::Eq for Matrix {}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut product: Matrix = Matrix::new();
        for row in 0..3 {
            for col in 0..3 {
                product.data[row][col] = self.data[row][0] * rhs.data[0][col] +
                                         self.data[row][1] * rhs.data[1][col] +
                                         self.data[row][2] * rhs.data[2][col] +
                                         self.data[row][3] * rhs.data[3][col]
            }
        }
        product
    }
}
/* ========================================================================== */

/* Operations with Matrices ================================================= */
/* ========================================================================== */

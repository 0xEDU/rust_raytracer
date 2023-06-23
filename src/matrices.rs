use std::fmt;
use std::ops;

use crate::tuple::Tuple;

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
    pub fn identity() -> Self {
        let mut id: Matrix = Matrix::new();
        id.data[0][0] = 1.0;
        id.data[1][1] = 1.0;
        id.data[2][2] = 1.0;
        id.data[3][3] = 1.0;
        id
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
        for row in 0..4 {
            for col in 0..4 {
                product.data[row][col] = self.data[row][0] * rhs.data[0][col] +
                                         self.data[row][1] * rhs.data[1][col] +
                                         self.data[row][2] * rhs.data[2][col] +
                                         self.data[row][3] * rhs.data[3][col]
            }
        }
        product
    }
}

impl ops::Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut product: Tuple = Tuple::new();
        product.x = self.data[0][0] * rhs.x + self.data[0][1] * rhs.y + self.data[0][2] * rhs.z + self.data[0][3] * rhs.w;
        product.y = self.data[1][0] * rhs.x + self.data[1][1] * rhs.y + self.data[1][2] * rhs.z + self.data[1][3] * rhs.w;
        product.z = self.data[2][0] * rhs.x + self.data[2][1] * rhs.y + self.data[2][2] * rhs.z + self.data[2][3] * rhs.w;
        product.w = self.data[3][0] * rhs.x + self.data[3][1] * rhs.y + self.data[3][2] * rhs.z + self.data[3][3] * rhs.w;
        product
    }
}
/* ========================================================================== */

/* Operations with Matrices ================================================= */
/* ========================================================================== */

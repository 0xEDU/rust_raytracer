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

    pub fn transpose(&self) -> Matrix {
        let mut t = Matrix::new();
        for i in 0..4 {
            for j in 0..4 {
                t.data[i][j] = self.data[j][i];
            }
        }
        t
    }

    pub fn determinant(&self) -> f64 {
        let sub_det_0 = self.data[2][2] * self.data[3][3] - self.data[2][3] * self.data[3][2];
        let sub_det_1 = self.data[2][1] * self.data[3][3] - self.data[2][3] * self.data[3][1];
        let sub_det_2 = self.data[2][1] * self.data[3][2] - self.data[2][2] * self.data[3][1];
        let sub_det_3 = self.data[2][0] * self.data[3][3] - self.data[2][3] * self.data[3][0];
        let sub_det_4 = self.data[2][0] * self.data[3][2] - self.data[2][2] * self.data[3][0];
        let sub_det_5 = self.data[2][0] * self.data[3][1] - self.data[2][1] * self.data[3][0];

        self.data[0][0] * (self.data[1][1] * sub_det_0 - self.data[1][2] * sub_det_1 + self.data[1][3] * sub_det_2) -
        self.data[0][1] * (self.data[1][0] * sub_det_0 - self.data[1][2] * sub_det_3 + self.data[1][3] * sub_det_4) +
        self.data[0][2] * (self.data[1][0] * sub_det_1 - self.data[1][1] * sub_det_3 + self.data[1][3] * sub_det_5) -
        self.data[0][3] * (self.data[1][0] * sub_det_2 - self.data[1][1] * sub_det_4 + self.data[1][2] * sub_det_5)
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let sub_matrix = self.submatrix(row, col);
        let sign = if (row + col) % 2 == 0 { 1.0 } else { -1.0 };
        // println!("{}", sub_matrix);
        sign * sub_matrix.determinant()
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut sub_matrix = Matrix::new();

        let mut sub_row = 0;
        for i in 0..4 {
            if i == row {
                continue;
            }

            let mut sub_col = 0;
            for j in 0..4 {
                if j == col {
                    continue;
                }
                sub_matrix.data[sub_row][sub_col] = self.data[i][j];
                println!("{} {} {}", self.data[i][j], i, j);
                sub_col += 1;
            }
            sub_row += 1;
        }
        sub_matrix
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

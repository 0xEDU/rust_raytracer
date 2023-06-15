/* Matrix declaration and implementation ==================================== */
pub struct Matrix {
    pub rows: usize,
    pub columns: usize,
    pub this: Vec<f64>,
}

impl Matrix {
    pub fn new(r: usize, c: usize, v: Vec<f64>) -> Matrix {
        Matrix {
            rows: r,
            columns: c,
            this: v,
        }
    }

    pub fn at(&self, i: usize, j: usize) -> f64 {
        self.this[i * self.columns + j]
    }
}
/* ========================================================================== */

/* Operator overloads for Matrix ============================================ */
/* ========================================================================== */

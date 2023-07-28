use crate::matrices::Matrix;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut m: Matrix = Matrix::identity();
    m.data[0][3] = x;
    m.data[1][3] = y;
    m.data[2][3] = z;
    m
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    let mut m: Matrix = Matrix::identity();
    m.data[0][0] = x;
    m.data[1][1] = y;
    m.data[2][2] = z;
    m
}

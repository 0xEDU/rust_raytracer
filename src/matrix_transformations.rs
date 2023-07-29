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

pub fn
shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    let mut m: Matrix = Matrix::identity();
    m.data[0][1] = xy;
    m.data[0][2] = xz;
    m.data[1][0] = yx;
    m.data[1][2] = yz;
    m.data[2][0] = zx;
    m.data[2][1] = zy;
    m
}

/* Rotations ---------------------------------------------------------------- */
pub fn rotation_x(r: f64) -> Matrix {
    let mut m: Matrix = Matrix::identity();
    m.data[1][1] = r.cos();
    m.data[1][2] = -(r.sin());
    m.data[2][1] = r.sin();
    m.data[2][2] = -(r.cos());
    m
}

pub fn rotation_y(r: f64) -> Matrix {
    let mut m: Matrix = Matrix::identity();
    m.data[0][0] = r.cos();
    m.data[0][2] = r.sin();
    m.data[2][0] = -(r.sin());
    m.data[2][2] = r.cos();
    m
}

pub fn rotation_z(r: f64) -> Matrix {
    let mut m: Matrix = Matrix::identity();
    m.data[0][0] = r.cos();
    m.data[0][1] = -(r.sin());
    m.data[1][0] = r.sin();
    m.data[1][1] = r.cos();
    m
}
/* -------------------------------------------------------------------------- */

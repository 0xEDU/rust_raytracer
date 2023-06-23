use std::println;

use rt_challenge::{matrices::*, tuple::{Tuple, point}};

#[test]
fn matrix_instatiation_test() {
    let mut m: Matrix = Matrix::new();
    for i in 0..3 {
        for j in 0..3 {
            m.data[i][j] = (i + j) as f64;
            assert!(m.data[i][j] == (i + j) as f64);
        }
    }
}

#[test]
fn matrix_operations_test() {
    let m: Matrix = Matrix::new();
    let n: Matrix = Matrix::new();
    assert!(m == n);

    let mut m: Matrix = Matrix::new();
    let mut n: Matrix = Matrix::new();
    for i in 0..3 {
        for j in 0..3 {
            m.data[i][j] = 2.0;
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            n.data[i][j] = 2.0;
        }
    }
    let product = m * n;
    for row in 0..3 {
        for col in 0..3 {
            assert!(product.data[row][col] == m.data[row][0] * n.data[0][col] +
                                            m.data[row][1] * n.data[1][col] +
                                            m.data[row][2] * n.data[2][col] +
                                            m.data[row][3] * n.data[3][col]
            )
        }
    }

    let mut m: Matrix = Matrix::new();
    let t: Tuple = point(1.0, 2.0, 3.0);
    let result: Tuple = point(18.0, 24.0, 33.0);
    m.data[0][0] = 1.0;
    m.data[0][1] = 2.0;
    m.data[0][2] = 3.0;
    m.data[0][3] = 4.0;
    m.data[1][0] = 2.0;
    m.data[1][1] = 4.0;
    m.data[1][2] = 4.0;
    m.data[1][3] = 2.0;
    m.data[2][0] = 8.0;
    m.data[2][1] = 6.0;
    m.data[2][2] = 4.0;
    m.data[2][3] = 1.0;
    m.data[3][0] = 0.0;
    m.data[3][1] = 0.0;
    m.data[3][2] = 0.0;
    m.data[3][3] = 1.0;
    assert!(m * t == result);
}

#[test]
fn test_identity_matrix() {
    let i = Matrix::identity();
    let mut m: Matrix = Matrix::new();
    m.data[0][0] = 1.0;
    m.data[0][1] = 2.0;
    m.data[0][2] = 3.0;
    m.data[0][3] = 4.0;
    m.data[1][0] = 2.0;
    m.data[1][1] = 4.0;
    m.data[1][2] = 4.0;
    m.data[1][3] = 2.0;
    m.data[2][0] = 8.0;
    m.data[2][1] = 6.0;
    m.data[2][2] = 4.0;
    m.data[2][3] = 1.0;
    m.data[3][0] = 0.0;
    m.data[3][1] = 0.0;
    m.data[3][2] = 0.0;
    m.data[3][3] = 1.0;
    assert!((m * i) == m);
}

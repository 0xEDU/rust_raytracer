use float_cmp::approx_eq;
use rt_challenge::{matrices::*, tuple::{Tuple, point}};

#[test]
fn matrix_instatiation_test() {
    let mut m: Matrix = Matrix::new();
    for i in 0..4 {
        for j in 0..4 {
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
    for i in 0..4 {
        for j in 0..4 {
            m.data[i][j] = 2.0;
        }
    }
    for i in 0..4 {
        for j in 0..4 {
            n.data[i][j] = 2.0;
        }
    }
    let product = m * n;
    for row in 0..4 {
        for col in 0..4 {
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
fn identity_matrix_test() {
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

#[test]
fn matrix_determinant_2_test() {
    let mut m = Matrix::matrix2();
    m.data[0][0] = 1.0;
    m.data[0][1] = 5.0;
    m.data[1][0] = -3.0;
    m.data[1][1] = 2.0;
    assert!(m.determinant() == 17.0);
}

#[test]
fn sub_matrix_test() {
    let mut m = Matrix::new();
    m.data[0][0] = 1.0;
    m.data[0][1] = 5.0;
    m.data[0][2] = 0.0;
    m.data[1][0] = -3.0;
    m.data[1][1] = 2.0;
    m.data[1][2] = 7.0;
    m.data[2][0] = 0.0;
    m.data[2][1] = 6.0;
    m.data[2][2] = 3.0;
    let sub_matrix = m.submatrix(0, 2);
    assert!(sub_matrix.data[0][0] == -3.0);
    assert!(sub_matrix.data[0][1] == 2.0);
    assert!(sub_matrix.data[1][0] == 0.0);
    assert!(sub_matrix.data[1][1] == 6.0);
}

#[test]
fn matrix_minor_test() {
    let mut m = Matrix::matrix3();
    m.data[0][0] = 3.0;
    m.data[0][1] = 5.0;
    m.data[0][2] = 0.0;
    m.data[1][0] = 2.0;
    m.data[1][1] = -1.0;
    m.data[1][2] = -7.0;
    m.data[2][0] = 6.0;
    m.data[2][1] = -1.0;
    m.data[2][2] = 5.0;
    assert!(m.minor(1, 0) == 25.0);
}

#[test]
fn matrix_cofactor_test() {
    let mut m = Matrix::matrix3();
    m.data[0][0] = 3.0;
    m.data[0][1] = 5.0;
    m.data[0][2] = 0.0;
    m.data[1][0] = 2.0;
    m.data[1][1] = -1.0;
    m.data[1][2] = -7.0;
    m.data[2][0] = 6.0;
    m.data[2][1] = -1.0;
    m.data[2][2] = 5.0;
    assert!(m.minor(0, 0) == -12.0);
    assert!(m.cofactor(0, 0) == -12.0);
    assert!(m.minor(1, 0) == 25.0);
    assert!(m.cofactor(1, 0) == -25.0);
}

#[test]
fn matrix_determinant_3_test() {
    let mut m = Matrix::matrix3();
    m.data[0][0] = 1.0;
    m.data[0][1] = 2.0;
    m.data[0][2] = 6.0;
    m.data[1][0] = -5.0;
    m.data[1][1] = 8.0;
    m.data[1][2] = -4.0;
    m.data[2][0] = 2.0;
    m.data[2][1] = 6.0;
    m.data[2][2] = 4.0;
    assert!(m.cofactor(0, 0) == 56.0);
    assert!(m.cofactor(0, 1) == 12.0);
    assert!(m.cofactor(0, 2) == -46.0);
    assert!(m.determinant() == -196.0);
}

#[test]
fn matrix_determinant_4_test() {
    let mut m = Matrix::new();
    m.data[0][0] = -2.0;
    m.data[0][1] = -8.0;
    m.data[0][2] = 3.0;
    m.data[0][3] = 5.0;
    m.data[1][0] = -3.0;
    m.data[1][1] = 1.0;
    m.data[1][2] = 7.0;
    m.data[1][3] = 3.0;
    m.data[2][0] = 1.0;
    m.data[2][1] = 2.0;
    m.data[2][2] = -9.0;
    m.data[2][3] = 6.0;
    m.data[3][0] = -6.0;
    m.data[3][1] = 7.0;
    m.data[3][2] = 7.0;
    m.data[3][3] = -9.0;
    assert!(m.cofactor(0, 0) == 690.0);
    assert!(m.cofactor(0, 1) == 447.0);
    assert!(m.cofactor(0, 2) == 210.0);
    assert!(m.cofactor(0, 3) == 51.0);
    assert!(m.determinant() == -4071.0);
}

#[test]
fn matrix_inversibility_test() {
    let mut m = Matrix::new();
    m.data[0][0] = -4.0;
    m.data[0][1] = -2.0;
    m.data[0][2] = -2.0;
    m.data[0][3] = -3.0;
    m.data[1][0] = 9.0;
    m.data[1][1] = 6.0;
    m.data[1][2] = 2.0;
    m.data[1][3] = 6.0;
    m.data[2][0] = 0.0;
    m.data[2][1] = -5.0;
    m.data[2][2] = 1.0;
    m.data[2][3] = -5.0;
    m.data[3][0] = 0.0;
    m.data[3][1] = 0.0;
    m.data[3][2] = 0.0;
    m.data[3][3] = 0.0;
    assert!(m.is_invertible() == false);
}

#[test]
fn matrix_inversion_test() {
    let mut m = Matrix::new();
    m.data[0][0] = -5.0;
    m.data[0][1] = 2.0;
    m.data[0][2] = 6.0;
    m.data[0][3] = -8.0;
    m.data[1][0] = 1.0;
    m.data[1][1] = -5.0;
    m.data[1][2] = 1.0;
    m.data[1][3] = 8.0;
    m.data[2][0] = 7.0;
    m.data[2][1] = 7.0;
    m.data[2][2] = -6.0;
    m.data[2][3] = -7.0;
    m.data[3][0] = 1.0;
    m.data[3][1] = -3.0;
    m.data[3][2] = 7.0;
    m.data[3][3] = 4.0;
    let inverted = m.inverse().unwrap();
    assert!((format!("{:.5}", inverted.data[0][0]) == "0.21805"));
    assert!(format!("{:.5}", inverted.data[0][1]) == "0.45113");
    assert!(format!("{:.5}", inverted.data[0][2]) == "0.24060");
    assert!(format!("{:.5}", inverted.data[0][3]) == "-0.04511");
    assert!(format!("{:.5}", inverted.data[1][0]) == "-0.80827");
    assert!(format!("{:.5}", inverted.data[1][1]) == "-1.45677");
    assert!(format!("{:.5}", inverted.data[1][2]) == "-0.44361");
    assert!(format!("{:.5}", inverted.data[1][3]) == "0.52068");
    assert!(format!("{:.5}", inverted.data[2][0]) == "-0.07895");
    assert!(format!("{:.5}", inverted.data[2][1]) == "-0.22368");
    assert!(format!("{:.5}", inverted.data[2][2]) == "-0.05263");
    assert!(format!("{:.5}", inverted.data[2][3]) == "0.19737");
    assert!(format!("{:.5}", inverted.data[3][0]) == "-0.52256");
    assert!(format!("{:.5}", inverted.data[3][1]) == "-0.81391");
    assert!(format!("{:.5}", inverted.data[3][2]) == "-0.30075");
    assert!(format!("{:.5}", inverted.data[3][3]) == "0.30639");
}

#[test]
fn matrix_multiplication_by_inverse_test() {
    let mut a = Matrix::new();
    a.data[0][0] = 3.0;
    a.data[0][1] = -9.0;
    a.data[0][2] = 7.0;
    a.data[0][3] = 3.0;
    a.data[1][0] = 3.0;
    a.data[1][1] = -8.0;
    a.data[1][2] = 2.0;
    a.data[1][3] = -9.0;
    a.data[2][0] = -4.0;
    a.data[2][1] = 4.0;
    a.data[2][2] = 4.0;
    a.data[2][3] = 1.0;
    a.data[3][0] = -6.0;
    a.data[3][1] = 5.0;
    a.data[3][2] = -1.0;
    a.data[3][3] = 1.0;
    
    let mut b = Matrix::new();
    b.data[0][0] = 8.0;
    b.data[0][1] = 2.0;
    b.data[0][2] = 2.0;
    b.data[0][3] = 2.0;
    b.data[1][0] = 3.0;
    b.data[1][1] = -1.0;
    b.data[1][2] = 7.0;
    b.data[1][3] = 0.0;
    b.data[2][0] = 7.0;
    b.data[2][1] = 0.0;
    b.data[2][2] = 5.0;
    b.data[2][3] = 4.0;
    b.data[3][0] = 6.0;
    b.data[3][1] = -2.0;
    b.data[3][2] = 0.0;
    b.data[3][3] = 5.0;
    
    let c = a * b;
    let result = c * b.inverse().unwrap();
    for row in 0..4 {
        for col in 0..4 {
            assert!(approx_eq!(f64, a.data[row][col], result.data[row][col], ulps = 15));
        }
    }
}

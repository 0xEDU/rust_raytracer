use rt_challenge::matrices::*;

#[test]
fn matrix_instatiation_test() {
    let mut m: Matrix = Matrix::new();
    for i in 0..3 {
        for j in 0..3 {
            m.data[i][j] = (i + j) as f64;
            assert!(m.data[i][j] == (i + j) as f64);
        }
    }
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
}

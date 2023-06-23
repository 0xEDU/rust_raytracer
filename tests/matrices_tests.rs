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
}

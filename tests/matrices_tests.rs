use rt_challenge::matrices::*;

#[test]
fn matrix_instatiation_test() {
    let v = vec![
        1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,
    ];
    let m = Matrix::new(4, 4, v);
    assert!(m.at(0, 0) == 1.0);
}

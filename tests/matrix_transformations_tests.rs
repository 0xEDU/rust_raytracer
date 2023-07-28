use rt_challenge::{matrix_transformations::*, tuple::{Tuple, point, vector}};

#[test]
fn simple_translation_test() {
    let transform = translation(5.0, -3.0, 2.0);
    let p: Tuple = point(-3.0, 4.0, 5.0);
    assert!(transform * p == point(2.0, 1.0, 7.0));
}

#[test]
fn point_translation_test() {
    let transform = translation(5.0, -3.0, 2.0);
    let inv = transform.inverse().unwrap();
    let p: Tuple = point(-3.0, 4.0, 5.0);
    assert!(inv * p == point(-8.0, 7.0, 3.0));
}

#[test]
fn vector_translation_test() {
    let transform = translation(5.0, -3.0, 2.0);
    let v: Tuple = vector(-3.0, 4.0, 5.0);
    assert!(transform * v == v);
}

#[test]
fn point_scaling_test() {
    let transform = scaling(2.0, 3.0, 4.0);
    let p = point(-4.0, 6.0, 8.0);
    assert!(transform * p == point(-8.0, 18.0, 32.0));
}

#[test]
fn vector_scaling_test() {
    let transform = scaling(2.0, 3.0, 4.0);
    let v = vector(-4.0, 6.0, 8.0);
    assert!(transform * v == vector(-8.0, 18.0, 32.0));
}

#[test]
fn inverse_scaling_test() {
    let transform = scaling(2.0, 3.0, 4.0);
    let inv = transform.inverse().unwrap();
    let v = vector(-4.0, 6.0, 8.0);
    assert!(inv * v == vector(-2.0, 2.0, 2.0));
}

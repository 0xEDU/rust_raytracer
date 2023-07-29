use std::f64::consts::PI;
use std::f64::consts::SQRT_2;

use float_cmp::approx_eq;
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

#[test]
fn x_axis_rotation_test() {
    let p = point(0.0, 1.0, 0.0);
    let half_quarter = rotation_x(PI / 4.0);
    let full_quarter = rotation_x(PI / 2.0);

    let half_result = half_quarter * p;
    let half_expected = point(0.0, SQRT_2/2.0, SQRT_2/2.0);

    let full_result = full_quarter * p;
    let full_expected = point(0.0, 0.0, 0.0);

    approx_eq!(f64, half_result.x, half_expected.x, ulps = 15);
    approx_eq!(f64, half_result.y, half_expected.x, ulps = 15);
    approx_eq!(f64, half_result.z, half_expected.x, ulps = 15);

    approx_eq!(f64, full_result.x, full_expected.x, ulps = 15);
    approx_eq!(f64, full_result.y, full_expected.x, ulps = 15);
    approx_eq!(f64, full_result.z, full_expected.x, ulps = 15);
}

#[test]
fn inverse_x_axis_rotation_test() {
    let p = point(0.0, 1.0, 0.0);
    let half_quarter = rotation_x(PI / 4.0);
    let inv = half_quarter.inverse().unwrap();

    let result = inv * p;
    let expected = point(0.0, SQRT_2/2.0, -(SQRT_2/2.0));

    approx_eq!(f64, result.x, expected.x, ulps = 15);
    approx_eq!(f64, result.y, expected.x, ulps = 15);
    approx_eq!(f64, result.z, expected.x, ulps = 15);
}

#[test]
fn y_axis_rotation_test() {
    let p = point(0.0, 0.0, 1.0);
    let half_quarter = rotation_y(PI / 4.0);
    let full_quarter = rotation_y(PI / 2.0);

    let half_result = half_quarter * p;
    let half_expected = point(SQRT_2/2.0, 0.0, SQRT_2/2.0);

    let full_result = full_quarter * p;
    let full_expected = point(1.0, 0.0, 0.0);

    approx_eq!(f64, half_result.x, half_expected.x, ulps = 15);
    approx_eq!(f64, half_result.y, half_expected.x, ulps = 15);
    approx_eq!(f64, half_result.z, half_expected.x, ulps = 15);

    approx_eq!(f64, full_result.x, full_expected.x, ulps = 15);
    approx_eq!(f64, full_result.y, full_expected.x, ulps = 15);
    approx_eq!(f64, full_result.z, full_expected.x, ulps = 15);
}

#[test]
fn z_axis_rotation_test() {
    let p = point(0.0, 1.0, 0.0);
    let half_quarter = rotation_z(PI / 4.0);
    let full_quarter = rotation_z(PI / 2.0);

    let half_result = half_quarter * p;
    let half_expected = point(-(SQRT_2/2.0), SQRT_2/2.0, 0.0);

    let full_result = full_quarter * p;
    let full_expected = point(-1.0, 0.0, 0.0);

    approx_eq!(f64, half_result.x, half_expected.x, ulps = 15);
    approx_eq!(f64, half_result.y, half_expected.x, ulps = 15);
    approx_eq!(f64, half_result.z, half_expected.x, ulps = 15);

    approx_eq!(f64, full_result.x, full_expected.x, ulps = 15);
    approx_eq!(f64, full_result.y, full_expected.x, ulps = 15);
    approx_eq!(f64, full_result.z, full_expected.x, ulps = 15);
}

#[test]
fn shearing_transformation_test() {
    let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let p = point(2.0, 3.0, 4.0);
    assert!(transform * p == point(5.0, 3.0, 4.0));
}

#[test]
fn x_to_z_shearing_transformation_test() {
    let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    let p = point(2.0, 3.0, 4.0);
    assert!(transform * p == point(6.0, 3.0, 4.0));
}

#[test]
fn y_to_x_shearing_transformation_test() {
    let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    let p = point(2.0, 3.0, 4.0);
    assert!(transform * p == point(2.0, 5.0, 4.0));
}

#[test]
fn y_to_z_shearing_transformation_test() {
    let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let p = point(2.0, 3.0, 4.0);
    assert!(transform * p == point(2.0, 7.0, 4.0));
}

#[test]
fn z_to_x_shearing_transformation_test() {
    let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let p = point(2.0, 3.0, 4.0);
    assert!(transform * p == point(2.0, 3.0, 6.0));
}
#[test]
fn z_to_y_shearing_transformation_test() {
    let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    let p = point(2.0, 3.0, 4.0);
    assert!(transform * p == point(2.0, 3.0, 7.0));
}

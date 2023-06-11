use rt_challenge::tuple::*;

#[test]
fn tuples_instatiation_test() {
    let v: Tuple = Tuple {x: 1.0, y: 1.0, z: 1.0, w: 0.0 };
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 1.0);
    assert_eq!(v.z, 1.0);
    assert_eq!(v.is_vector(), true);

    let d: Tuple = Tuple {x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
    assert_eq!(d.x, 1.0);
    assert_eq!(d.y, 1.0);
    assert_eq!(d.z, 1.0);
    assert_eq!(d.is_point(), true);
}

#[test]
fn point_creation_test() {
    let a: Point = point(4.0, -4.0, 3.0);
    assert_eq!(a.is_point(), true);
    assert_eq!(a.x, 4.0);
    assert_eq!(a.y, -4.0);
    assert_eq!(a.z, 3.0);
}

#[test]
fn vector_creation_test() {
    let a: Vector = vector(4.0, -4.0, 3.0);
    assert_eq!(a.is_vector(), true);
    assert_eq!(a.x, 4.0);
    assert_eq!(a.y, -4.0);
    assert_eq!(a.z, 3.0);
}

#[test]
fn tuple_comparison_test() {
    let a: Point = point(4.0, -4.0, 3.0);
    let b: Point = point(4.0, -4.0, 3.0);
    assert!(a == b);
}

#[test]
fn tuple_operator_overload_test() {
    let a: Vector = vector(1.0, 1.0, 1.0);
    let b: Vector = vector(1.0, 1.0, 1.0);
    let result: Vector = vector(2.0, 2.0, 2.0);
    assert!((a + b) == result);

    let a: Vector = vector(1.0, 1.0, 1.0);
    let b: Vector = vector(1.0, 1.0, 1.0);
    let result: Vector = vector(0.0, 0.0, 0.0);
    assert!((a - b) == result);

    let a: Tuple = Tuple {x: 1.0, y: 1.0, z: 1.0, w: 0.0};
    let result: Tuple = Tuple {x: -1.0, y: -1.0, z: -1.0, w: 0.0};
    assert!(-a == result);

    let a: Tuple = Tuple {x: 1.0, y: 1.0, z: 1.0, w: 0.0};
    let result: Tuple = Tuple {x: 2.0, y: 2.0, z: 2.0, w: 0.0};
    assert!(a * 2.0 == result);

    let a: Tuple = Tuple {x: 2.0, y: 2.0, z: 2.0, w: 0.0};
    let result: Tuple = Tuple {x: 1.0, y: 1.0, z: 1.0, w: 0.0};
    assert!(a / 2.0 == result);
}

#[test]
fn tuple_operations_test() {
    let a: Vector = vector(1.0, 2.0, 3.0);
    let result: f64 = (14.0_f64).sqrt();
    assert!(magnitude(a) == result);
}

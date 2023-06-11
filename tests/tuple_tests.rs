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

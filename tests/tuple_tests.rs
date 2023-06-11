use rt_challenge::tuple::Tuple;

#[test]
fn simple_tuples_test() {
    let a: Tuple = Tuple {x: 1.0, y: 1.0, z: 1.0, w: 0.0 };
    assert_eq!(a.x, 1.0);
    assert_eq!(a.y, 1.0);
    assert_eq!(a.z, 1.0);
    assert_eq!(a.is_vector(), true);

    let a: Tuple = Tuple {x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
    assert_eq!(a.x, 1.0);
    assert_eq!(a.y, 1.0);
    assert_eq!(a.z, 1.0);
    assert_eq!(a.is_direction(), true);
}

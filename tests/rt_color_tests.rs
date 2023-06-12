use rt_challenge::rt_color::*;

#[test]
fn color_creation_test() {
    let c: RTColor = color(-0.5, 0.4, 1.7);
    let result: RTColor = RTColor {r: -0.5, g: 0.4, b: 1.7};
    assert!(c == result);
}

#[test]
fn tuple_operator_overload_test() {
    let a: RTColor = color(1.0, 1.0, 1.0);
    let b: RTColor = color(1.0, 1.0, 1.0);
    let result: RTColor = color(2.0, 2.0, 2.0);
    assert!((a + b) == result);

    let a: RTColor = color(1.0, 1.0, 1.0);
    let b: RTColor = color(1.0, 1.0, 1.0);
    let result: RTColor = color(0.0, 0.0, 0.0);
    assert!((a - b) == result);

    let a: RTColor = color(2.0, 2.0, 2.0);
    let b: RTColor = color(2.0, 2.0, 2.0);
    let result: RTColor = RTColor {r: 4.0, g: 4.0, b: 4.0};
    assert!(a * b == result);

}

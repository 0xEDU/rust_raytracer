use rt_challenge::rt_color::*;

#[test]
fn color_creation_test() {
    let c: RTColor = color(-0.5, 0.4, 1.7);
    let result: RTColor = RTColor {r: -0.5, g: 0.4, b: 1.7};
    assert!(c == result);
}

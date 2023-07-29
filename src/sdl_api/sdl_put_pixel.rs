use crate::rt_color::RTColor;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::video::Window;
use sdl2::render::Canvas;

pub fn sdl_put_pixel(canvas: &mut Canvas<Window>, point: Point, color: RTColor) {
    canvas.set_draw_color(Color::RGB((color.r * 255.0) as u8,
                                     (color.g * 255.0) as u8,
                                     (color.b * 255.0) as u8));
    canvas.draw_point(point).unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
}

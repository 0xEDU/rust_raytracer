extern crate sdl2;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

struct SDLWrapper {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
}

// These functions will be stored in a sdl2_extension
// (or something like that) submodule.
fn sdl_put_pixel(canvas: &mut Canvas<Window>, point: Point, color: Color) {
    canvas.set_draw_color(color);
    canvas.draw_point(point).unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
}

fn sdl_start(window_name: &str,
    width: u32,
    height: u32) -> SDLWrapper {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window(window_name, width, height)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    SDLWrapper {
        sdl_context,
        canvas,
    }
}

fn sdl_event_handler(event: Event) -> Result<(), bool> {
    match event {
        Event::Quit { .. } |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => Err({
                true
            }),
        _ => Ok({})
    }
}
 
pub fn main() {
    let mut sdl_wrapper: SDLWrapper = sdl_start("RT", 800, 600);

    let mut event_pump = sdl_wrapper.sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            if let Err(_) = sdl_event_handler(event) {
                break 'running
            }
        }
        sdl_wrapper.canvas.clear();
        sdl_put_pixel(&mut sdl_wrapper.canvas,
            Point::new(40, 40),
            Color::RGB(149, 173, 32));
        sdl_wrapper.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

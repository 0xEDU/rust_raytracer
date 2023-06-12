mod sdl_api;

use sdl_api::*;
use std::time::Duration;

pub fn main() {
    let mut sdl_wrapper: SDLWrapper = sdl_api::sdl_start("RT", 800, 600);

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

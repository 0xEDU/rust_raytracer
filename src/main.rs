mod sdl_api;

use rt_challenge::rt_color::*;
use sdl_api::*;

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
            RTColor {r: 1.0, g: 0.0, b: 0.0});
        sdl_wrapper.canvas.present();
    }
}

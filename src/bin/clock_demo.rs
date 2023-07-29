use rt_challenge::sdl_api::*;

use rt_challenge::rt_color::RTColor;

pub fn main() {
    let mut sdl_wrapper: SDLWrapper = sdl_start("RT", 800, 600);

    sdl_put_pixel(&mut sdl_wrapper.canvas, Point::new(100, 100), RTColor::new(255.0, 0.0, 0.0));
    sdl_wrapper.canvas.present();
    let mut event_pump = sdl_wrapper.sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            if let Err(_) = sdl_event_handler(event) {
                break 'running
            }
        }
    }
}

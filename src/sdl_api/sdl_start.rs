use crate::SDLWrapper;
use sdl2::pixels::Color;

pub fn sdl_start(window_name: &str,
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


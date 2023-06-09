pub mod sdl_start;
pub mod sdl_put_pixel;
pub mod sdl_event_handler;

pub use sdl_start::*;
pub use sdl_put_pixel::*;
pub use sdl_event_handler::*;

use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::render::Canvas;
pub use sdl2::pixels::Color;
pub use sdl2::rect::Point;

pub struct SDLWrapper {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
}

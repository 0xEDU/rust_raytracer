use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn sdl_event_handler(event: Event) -> Result<(), bool> {
    match event {
        Event::Quit { .. } |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => Err({
                true
            }),
        _ => Ok({})
    }
}

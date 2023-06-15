mod sdl_api;

use rt_challenge::{rt_color::*, tuple::{normalize, vector, point}};
use sdl_api::*;

use rt_challenge::tuple::{
    RTPoint,
    Vector
};
// use std::time::Duration;

#[derive(Clone, Copy)]
struct Projectile {
    pub position: RTPoint,
    pub velocity: Vector,
}

#[derive(Clone, Copy)]
struct Environment {
    pub gravity: Vector,
    pub wind: Vector
}

fn tick(env: Environment, proj: Projectile) -> Projectile {
    Projectile {
        position: proj.position + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind
    }
}

pub fn main() {
    let mut sdl_wrapper: SDLWrapper = sdl_api::sdl_start("RT", 800, 600);

    let mut p = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: normalize(vector(0.01, 0.08, 0.0)),
    };
    let e = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(0.8, 0.0, 0.0)
    };
    while p.position.y >= 0.0 {
        p = tick(e, p);
        println!("x = {}, y = {}, z = {}", p.position.x, p.position.y, p.position.y);
        sdl_put_pixel(&mut sdl_wrapper.canvas,
            Point::new((p.position.x * 100.25) as i32, (p.position.y * 100.25) as i32),
            RTColor {r: 1.0, g: 0.0, b: 0.0});
    }
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

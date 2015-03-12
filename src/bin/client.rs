extern crate sdl2;
extern crate game;

use game::gfx::*;
use game::physics::particle::*;

use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::pixels::Color;
use sdl2::keycode::KeyCode;

pub fn main() {
    let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();

    let window = match Window::new("game",
                                   WindowPos::PosUndefined,
                                   WindowPos::PosUndefined,
                                   800, 600,
                                   OPENGL) {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    let renderer = Renderer::from_window(window, RenderDriverIndex::Auto, ACCELERATED).unwrap();
    let mut drawer = renderer.drawer();
    drawer.set_draw_color(Color::RGB(43, 53, 56));

    let mut running = true;
    let mut event_pump = sdl_context.event_pump();

    let particle = ParticleTexture::new(&renderer, 32, Color::RGBA(173, 200, 206, 128));

    while running {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Escape, .. } => {
                    running = false
                },
                _ => {}
            }
        }

        drawer.clear();
        particle.draw(&mut drawer, (50, 50));
        particle.draw(&mut drawer, (70, 60));
        drawer.present();
        // The rest of the game loop goes here...
    }
}

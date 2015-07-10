extern crate sdl2;
extern crate shared;

mod gfx_particle_type;
mod particle_drawing;
mod fps_limiter;
mod states;
mod font;
mod font_data;
mod colors;
mod texture_generator;
mod raytrace;

use fps_limiter::*;
use sdl2::pixels::Color;

pub fn main() {
    let mut sdl_context = sdl2::init().video().unwrap();
    let window = sdl_context.window("Planet Jump", 800, 600)
        .resizable()
        .build()
        .unwrap();
    let mut renderer = window.renderer()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();

    let mut state: Box<states::State> = states::inmenu::in_main_menu();
    state.init(None);

    for (elapsed, fps) in FpsLimiter::new(60) {
        for event in sdl_context.event_pump().poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => return,
                _ => state.handle(event),
            }
        }

        match state.update(&sdl_context, &mut renderer, elapsed, fps) {
            states::UpdateResult::Stay => {},
            states::UpdateResult::Change(mut new_state) => {
                new_state.init(Some(state));
                state = new_state;
            },
            states::UpdateResult::Back(Some(mut new_state)) => {
                state = new_state;
                // We're not setting previous state because this already is the previous
            },
            states::UpdateResult::Back(None) => {
                return
            }
            states::UpdateResult::Reset(mut new_state) => {
                new_state.init(None);
                state = new_state;
            }
        }
    }
}

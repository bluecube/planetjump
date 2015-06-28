extern crate sdl2;

pub mod inmenu;
pub mod ingame;

use gfx_particle_type::*;
use particle_drawing::*;
use font;
use colors;

pub trait State {
    fn handle(&mut self, event: sdl2::event::Event);
    fn update(&mut self, renderer: &mut sdl2::render::Renderer,
              elapsed: u32, fps: f32) -> UpdateResult;
    fn init(&mut self, previous_state: Option<Box<State>>);
}

pub enum UpdateResult {
    /// Keep the same state
    Stay,

    /// Regular forward step. Call init on the new state
    Change(Box<State>),

    /// Backward step. The state is supposed to be already inited.
    /// If state is None, ends the program.
    Back(Option<Box<State>>),

    /// Like change, but passes None as the previous_step of init method.
    Reset(Box<State>)
}

extern crate sdl2;

pub mod inmenu;
pub mod ingame;

use gfx_particle_type::*;
use particle_drawing::*;
use font;
use colors;

pub trait State<'a> {
    fn handle(&'a mut self, event: sdl2::event::Event);
    fn draw(&'a mut self, drawer: &mut sdl2::render::RenderDrawer);
    fn update(&'a mut self) -> UpdateResult;
    fn init(&'a mut self, previous_state: Option<Box<State>>,
            renderer: &sdl2::render::Renderer);
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

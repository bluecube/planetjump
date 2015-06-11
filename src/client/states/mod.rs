extern crate sdl2;

pub mod inmenu;

use gfx_particle_type::*;
use particle_drawing::*;
use shared::physics::tree::*;
use font;
use colors;

pub trait State {
    fn handle(&mut self, event: sdl2::event::Event);
    fn draw(&mut self, drawer: &mut sdl2::render::RenderDrawer);
    fn update(&mut self) -> UpdateResult;
    fn set_previous(&mut self, previous_state: Box<State>);
}

pub enum UpdateResult {
    Stay,
    Change(Box<State>),
    Back(Option<Box<State>>),
}

/*
pub struct InGame {
    tree: Tree<GfxParticleType>,
    step: u8,
}

impl State for Box<InGame> {
    fn handle(&mut self, event: Event) {}

    fn draw(&mut self, drawer: &mut sdl2::render::Drawer) {
        drawer.clear();
        draw_particles(&self.tree, self.step, &mut drawer);
        drawer.present();
    }

    fn update(self) {
        self.step = 1 - self.step;
        self.tree.update(self.step);

        return self;
    }
}*/

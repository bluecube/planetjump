extern crate sdl2;

use gfx_particle_type::*;
use particle_drawing::*;
use shared::physics::tree::*;

use sdl2::event::Event;
use sdl2::keycode::KeyCode;

pub trait State {
    fn handle(&mut self, event: Event);
    fn draw(&mut self, drawer: &mut sdl2::render::RenderDrawer);
    fn update(self: Box<Self>) -> Option<Box<State>>;
}

pub enum StateUpdateResult {
    Quit,
    KeepState,
    ChangeState(Box<State>),
}

pub struct InMainMenu {
    selected: u32,
}

impl State for InMainMenu {
    fn handle(&mut self, event: Event) {
        match event {
            Event::KeyDown {keycode: KeyCode::Up, .. } => {
                self.selected -= 10;
            },
            Event::KeyDown {keycode: KeyCode::Down, .. } => {
                self.selected += 10;
            }
            _ => {}
        }
    }

    fn draw(&mut self, drawer: &mut sdl2::render::RenderDrawer) {
        println!("drawing");
        if self.selected > 255 {
            self.selected = 0;
        }
        drawer.set_draw_color(sdl2::pixels::Color::RGB(self.selected as u8, 0, 0));
        drawer.clear();
    }

    fn update(self: Box<Self>) -> Option<Box<State>> {
        println!("updating");
        Some(self)
    }
}

impl InMainMenu {
    pub fn new() -> InMainMenu {
        InMainMenu { selected: 0 }
    }
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

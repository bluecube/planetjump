extern crate sdl2;

use gfx_particle_type::*;
use particle_drawing::*;
use shared::physics::tree::*;
use font;

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
                self.selected -= 1;
            },
            Event::KeyDown {keycode: KeyCode::Down, .. } => {
                self.selected += 1;
            }
            _ => {}
        }
    }

    fn draw(&mut self, drawer: &mut sdl2::render::RenderDrawer) {
        if self.selected > 255 {
            self.selected = 0;
        }
        drawer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        drawer.clear();
        drawer.set_draw_color(sdl2::pixels::Color::RGB(0, 126, 0));
        font::draw_text("Hello world! E = mc^2 1234567890", drawer, 10, 10, 5);
        font::draw_text("abcdefghijklmnopqrstuvwxyz", drawer, 10, 50, 5);
        font::draw_text("ABCDEFGHIJKLMNOPQRSTUVWXYZ", drawer, 10, 100, 5);
        font::draw_text("~!@#$%^&*()_+-=[]{}:;\"'\\|<,>.?/", drawer, 10, 150, 5);
        font::draw_text("This is unknown char: \"á\".", drawer, 10, 200, 5);
    }

    fn update(self: Box<Self>) -> Option<Box<State>> {
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

extern crate sdl2;

use states::{State, UpdateResult};
use sdl2::event::Event;
use sdl2::keycode::KeyCode;
use std;

use font;
use colors;

pub struct InMenu {
    options: Vec<(&'static str, Box<Fn() -> UpdateResult>)>,
    selected: usize,
    enter: bool,
    exit: bool,
    previous: Option<Box<State>>,
}

impl State for InMenu {
    fn handle(&mut self, event: Event) {
        if self.enter || self.exit {
            return;
        }
        match event {
            Event::KeyDown {keycode: KeyCode::Up, .. } => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            },
            Event::KeyDown {keycode: KeyCode::Down, .. } => {
                if self.selected < self.options.len() - 1 {
                    self.selected += 1;
                }
            }
            Event::KeyDown {keycode: KeyCode::Return, .. } |
            Event::KeyDown {keycode: KeyCode::Space, .. } => {
                self.enter = true;
            }
            Event::KeyDown {keycode: KeyCode::Escape, .. } => {
                self.exit = true;
            }
            _ => {}
        }
    }

    fn draw(&mut self, drawer: &mut sdl2::render::RenderDrawer) {
        drawer.set_draw_color(colors::bg);
        drawer.clear();
        let mut i = 0;
        for (i, tuple) in self.options.iter().enumerate() {
            let text = tuple.0;
            drawer.set_draw_color(if i == self.selected { colors::highlight } else { colors::fg });
            //let rect = font::measure_text(text, 5);
            font::draw_text(text, drawer, 10, 10 + 50 * i as i32, 5);
        }
    }

    fn update(&mut self) -> UpdateResult {
        if self.enter {
            self.enter = false;
            self.options[self.selected].1()
        }
        else if self.exit {
            let mut ret = None;
            std::mem::swap(&mut self.previous, &mut ret);
            // Here we must swap to avoid moving from reference.
            // This operation resets self.previous to None, but
            // we don't mind, because this state will get deleted anyway.

            UpdateResult::Back(ret)
        }
        else {
            UpdateResult::Stay
        }
    }

    fn set_previous(&mut self, previous_state: Box<State>) {
        self.previous = Some(previous_state);
    }
}

pub fn in_main_menu() -> InMenu {
    InMenu {
        options: vec!(("Join Game", Box::new(|| UpdateResult::Stay)),
                      ("Player Settings", Box::new(|| UpdateResult::Stay)),
                      ("Controls Settings", Box::new(|| UpdateResult::Stay)),
                      ("Exit", Box::new(|| UpdateResult::Back(None)))),
        selected: 0,
        enter: false,
        exit: false,
        previous: None
    }
}

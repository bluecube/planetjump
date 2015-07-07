extern crate sdl2;
extern crate shared;
extern crate rand;

use states;
use states::{State, UpdateResult};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std;

use font;
use colors;


pub struct InMenu {
    options: Vec<(&'static str, Box<Fn(&mut sdl2::render::Renderer) -> UpdateResult>)>,
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
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            },
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                if self.selected < self.options.len() - 1 {
                    self.selected += 1;
                }
            }
            Event::KeyDown {keycode: Some(Keycode::Return), .. } |
            Event::KeyDown {keycode: Some(Keycode::Space), .. } => {
                self.enter = true;
            }
            Event::KeyDown {keycode: Some(Keycode::Escape), .. } => {
                self.exit = true;
            }
            _ => {}
        }
    }

    fn update(&mut self,
              sdl: &sdl2::Sdl,
              renderer: &mut sdl2::render::Renderer,
              elapsed: u32, fps: f32) -> UpdateResult {
        renderer.set_draw_color(colors::bg);
        renderer.clear();
        let mut i = 0;
        for (i, tuple) in self.options.iter().enumerate() {
            let text = tuple.0;
            renderer.set_draw_color(if i == self.selected { colors::highlight } else { colors::fg });
            //let rect = font::measure_text(text, 5);
            let scale = 5;
            font::draw_text(text, renderer, 10, 10 + font::line_spacing(scale) * i as i32, scale);
        }
        renderer.present();

        if self.enter {
            self.enter = false;
            self.options[self.selected].1(renderer)
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

    fn init(&mut self, previous_state: Option<Box<State>>) {
        self.previous = previous_state;
    }
}

pub fn in_main_menu() -> Box<State> {
    Box::new(InMenu {
        options: vec!(("Join Game", Box::new(|renderer| UpdateResult::Change(states::ingame::new_game(renderer)))),
                      ("Player Settings", Box::new(|renderer| UpdateResult::Stay)),
                      ("Controls Settings", Box::new(|renderer| UpdateResult::Stay)),
                      ("Exit", Box::new(|renderer| UpdateResult::Back(None)))),
        selected: 0,
        enter: false,
        exit: false,
        previous: None
    })
}

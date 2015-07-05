extern crate sdl2;

use states;
use states::{State, UpdateResult};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use gfx_particle_type::*;
use particle_drawing::*;

use shared::game_state::*;
use shared::particle_definitions::*;

use font;
use colors;

pub struct InGame {
    game: GameState<GfxParticleType>,
    action: Action,
}

enum Action {
    None,
    Exit
}

impl State for InGame {
    fn handle(&mut self, event: Event) {
        match event {
            Event::KeyDown {keycode: Some(Keycode::Escape), .. } => {
                self.action = Action::Exit;
            }
            _ => {}
        }
    }

    fn update(&mut self,
              renderer: &mut sdl2::render::Renderer,
              elapsed: u32, fps: f32) -> UpdateResult {
        if let Action::Exit = self.action {
            return UpdateResult::Reset(states::inmenu::in_main_menu());
        }

        self.game.update();

        renderer.set_draw_color(colors::bg);
        renderer.clear();
        draw_fps(fps, renderer);
        draw_particles(&self.game.particles, self.game.step, renderer);
        renderer.present();

        UpdateResult::Stay
    }

    fn init(&mut self, previous_state: Option<Box<State>>) {
        // No-op
    }
}

fn draw_fps(fps: f32, renderer: &mut sdl2::render::Renderer) {
    let text = &format!("{:.1}", fps);
    let scale = 20;
    let (screen_w, _) = renderer.get_output_size().unwrap();
    let (font_w, _) = font::measure_text(text, scale);

    renderer.set_draw_color(colors::faded);
    font::draw_text(text, renderer, (screen_w - font_w) as i32, 0, scale);
}


pub fn new_game(renderer: &sdl2::render::Renderer) -> Box<InGame> {
    let particle_types = load_particle_types(&renderer,
                                             particle_types());
    Box::new(InGame {
        game: GameState::<GfxParticleType>::new(particle_types),
        action: Action::None
    })
}

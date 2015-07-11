use sdl2;

use states;
use states::{State, UpdateResult};
use sdl2::event::Event;
use sdl2::keyboard::Scancode;

use gfx_particle_type::*;
use particle_drawing::*;

use shared::game_state::*;
use shared::input_state::*;
use shared::particle_definitions::*;
use shared::geometry::*;

use font;
use colors;

pub struct InGame {
    game: GameState<GfxParticleType>,
    input_state: InputState,
    action: Action,
}

enum Action {
    None,
    Exit
}

impl State for InGame {
    fn handle(&mut self, event: Event) {
        match event {
            Event::KeyDown {scancode: Some(Scancode::Escape), .. } => {
                self.action = Action::Exit;
            }
            _ => {}
        }
    }

    fn update(&mut self,
              sdl: &sdl2::Sdl,
              renderer: &mut sdl2::render::Renderer,
              elapsed: u32, fps: f32) -> UpdateResult {

        // TODO: mappable controls?
        let keyboard_state = sdl.keyboard_state();
        if (keyboard_state.is_scancode_pressed(Scancode::Space))
        {
            self.input_state.jump = true;
        }

        let (mouse_state, x, y) = sdl2::mouse::get_mouse_state();
        self.input_state.aim = Vector2D::new(x as f32, y as f32);
        self.input_state.primary_fire = mouse_state.left();
        self.input_state.secondary_fire = mouse_state.right();

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
        input_state: InputState::new(),
        action: Action::None

    })
}

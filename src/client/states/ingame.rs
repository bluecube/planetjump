extern crate sdl2;
extern crate shared;
extern crate rand;

use states;
use states::{State, UpdateResult};
use sdl2::event::Event;
use sdl2::keycode::KeyCode;
use std;
use std::rc::Rc;

use gfx_particle_type::*;
use particle_drawing::*;
use self::shared::geometry::*;
use self::shared::physics::particle::*;
use self::shared::physics::tree::*;

use font;
use colors;

pub struct InGame {
    particle_types: Vec<Rc<GfxParticleType>>,
    tree: Tree<GfxParticleType>,
    step: u8,
    action: Action,
}

enum Action {
    None,
    Exit
}

impl State for InGame {
    fn handle(&mut self, event: Event) {
        match event {
            Event::KeyDown {keycode: KeyCode::Escape, .. } => {
                self.action = Action::Exit;
            }
            _ => {}
        }
    }

    fn update(&mut self, renderer: &mut sdl2::render::Renderer) -> UpdateResult {
        if let Action::Exit = self.action {
            return UpdateResult::Reset(states::inmenu::in_main_menu());
        }
        self.step = 1 - self.step;
        self.tree.update(self.step);

        let mut drawer = renderer.drawer();
        drawer.set_draw_color(colors::bg);
        drawer.clear();
        draw_particles(&self.tree, self.step, &mut drawer);
        drawer.present();

        UpdateResult::Stay
    }

    fn init(&mut self, previous_state: Option<Box<State>>) {
        // No-op
    }
}

fn add_particles(count: u32,
                 position: Vector2D, pos_range: f32,
                 velocity: Vector2D, vel_range: f32,
                 particle_type: &Rc<GfxParticleType>,
                 out: &mut Vec<Particle<GfxParticleType>>) {
    let mut rng = rand::weak_rng();

    for _ in 0..count {
        let pos = Vector2D::random_radius(&mut rng, position, pos_range);
        let vel = Vector2D::random_radius(&mut rng, velocity, vel_range);

        let p = Particle::<GfxParticleType>::new(pos, vel, 0, particle_type.clone());

        out.push(p);
    }
}


pub fn new_game(renderer: &sdl2::render::Renderer) -> Box<InGame> {
    let particle_types = load_particle_types(&renderer,
                                             shared::particle_definitions::particle_types());
    let mut particles = Vec::<Particle<GfxParticleType>>::new();
    add_particles(100,
                  Vector2D::new(400.0, 400.0), 40.0,
                  Vector2D::new(-2.0, -1.0), 0.0,
                  &particle_types[2],
                  &mut particles);
    add_particles(70,
                  Vector2D::new(600.0, 400.0), 20.0,
                  Vector2D::new(-5.0, 10.0), 0.0,
                  &particle_types[0],
                  &mut particles);
    add_particles(50,
                  Vector2D::new(700.0, 300.0), 20.0,
                  Vector2D::new(10.0, 5.0), 0.0,
                  &particle_types[1],
                  &mut particles);

    Box::new(InGame {
        particle_types: particle_types,
        tree: Tree::new(particles, 0),
        step: 0,
        action: Action::None
    })
}

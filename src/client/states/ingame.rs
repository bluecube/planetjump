extern crate sdl2;
extern crate shared;
extern crate rand;

use states;
use states::{State, UpdateResult};
use sdl2::event::Event;
use sdl2::keycode::KeyCode;
use std;

use gfx_particle_type::*;
use particle_drawing::*;
use self::shared::geometry::*;
use self::shared::physics::particle::*;
use self::shared::physics::tree::*;

use font;
use colors;

pub struct InGame<'a> {
    particle_types: Vec<GfxParticleType>,
    tree: Tree<'a, GfxParticleType>,
    step: u8,
    action: Action,
}

enum Action {
    None,
    Exit
}

impl<'a> State<'a> for InGame<'a> {
    fn handle(&mut self, event: Event) {
        match event {
            Event::KeyDown {keycode: KeyCode::Escape, .. } => {
                self.action = Action::Exit;
            }
            _ => {}
        }
    }

    fn draw(&mut self, drawer: &mut sdl2::render::RenderDrawer) {
        drawer.clear();
        draw_particles(&self.tree, self.step, &mut drawer);
        drawer.present();
    }

    fn update(&mut self) -> UpdateResult {
        if let Action::Exit = self.action {
            return UpdateResult::Reset(states::inmenu::in_main_menu());
        }
        self.step = 1 - self.step;
        self.tree.update(self.step);

        UpdateResult::Stay
    }

    fn init(&'a mut self, previous_state: Option<Box<State>>,
            renderer: &sdl2::render::Renderer) {
        self.particle_types = load_particle_types(&renderer,
                                                  shared::particle_definitions::particle_types());

        let mut particles = Vec::<Particle<GfxParticleType>>::new();
        add_particles(100,
                      Vector2D::new(400.0, 400.0), 40.0,
                      Vector2D::new(0.0, -1.0), 0.1,
                      &self.particle_types[0],
                      &mut particles);
        add_particles(70,
                      Vector2D::new(600.0, 400.0), 20.0,
                      Vector2D::new(0.0, 20.0), 0.1,
                      &self.particle_types[0],
                      &mut particles);
        add_particles(50,
                      Vector2D::new(700.0, 300.0), 20.0,
                      Vector2D::new(5.0, 10.0), 0.1,
                      &self.particle_types[1],
                      &mut particles);

        self.tree = Tree::new(particles, self.step)
    }
}

fn add_particles<'a>(count: u32,
                     position: Vector2D, pos_range: f32,
                     velocity: Vector2D, vel_range: f32,
                     particle_type: &'a GfxParticleType,
                     out: &mut Vec<Particle<'a, GfxParticleType>>) {
    let mut rng = rand::weak_rng();

    for _ in 0..count {
        let pos = Vector2D::random_radius(&mut rng, position, pos_range);
        let vel = Vector2D::random_radius(&mut rng, velocity, vel_range);

        let p = Particle::<'a, GfxParticleType>::new(pos, vel, 0, particle_type);

        out.push(p);
    }
}


pub fn new_game<'a>() -> Box<InGame<'a>> {
    Box::new(InGame {
        particle_types: vec!(),
        tree: Tree::new(vec!(), 0),
        step: 0
    })
}

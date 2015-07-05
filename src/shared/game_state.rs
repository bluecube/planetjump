use physics::traits::*;
use physics::particle::*;
use physics::tree::*;
use geometry::*;

use std::rc::Rc;
use rand;

pub struct GameState<T: HasParticleProperties> {
    pub particles: Tree<T>,
    pub step: u8,

    particle_types: Vec<Rc<T>>,
}

impl<T: HasParticleProperties> GameState<T> {
    pub fn new(particle_types: Vec<T>) -> GameState<T> {
        let particle_types_rc : Vec<Rc<T>> = particle_types.into_iter().map(|x| Rc::new(x)).collect();

        let mut particles = Vec::<Particle<T>>::new();
        add_particles(100,
                      Vector2D::new(400.0, 400.0), 40.0,
                      Vector2D::new(-2.0, -1.0), 0.0,
                      &particle_types_rc[2],
                      &mut particles);
        add_particles(70,
                      Vector2D::new(600.0, 400.0), 20.0,
                      Vector2D::new(-5.0, 5.0), 0.0,
                      &particle_types_rc[0],
                      &mut particles);
        add_particles(50,
                      Vector2D::new(700.0, 300.0), 20.0,
                      Vector2D::new(10.0, 5.0), 0.0,
                      &particle_types_rc[1],
                      &mut particles);

        GameState::<T> {
            particles: Tree::new(particles, 0),
            step: 0,
            particle_types: particle_types_rc,
        }
    }

    // TODO: add parameters: User input and optional reference state from server
    pub fn update(&mut self) {
        self.step = 1 - self.step;
        self.particles.update(self.step);
    }
}

fn add_particles<T: HasParticleProperties>(count: u32,
                                           position: Vector2D, pos_range: f32,
                                           velocity: Vector2D, vel_range: f32,
                                           particle_type: &Rc<T>,
                                           out: &mut Vec<Particle<T>>) {
    let mut rng = rand::weak_rng();

    for _ in 0..count {
        let pos = Vector2D::random_radius(&mut rng, position, pos_range);
        let vel = Vector2D::random_radius(&mut rng, velocity, vel_range);

        let p = Particle::<T>::new(pos, vel, 0, particle_type.clone());

        out.push(p);
    }
}


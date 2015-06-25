use geometry::*;
use physics::traits::*;
use physics::forces::FRICTION;
use std::rc::Rc;

// Time step.
static DT: f32 = 0.01;

#[derive(PartialEq,Clone,Debug)]
pub struct BasicParticleType {
    pub inertia_mass: f32,
    pub gravity_mass: f32,
    pub d0: f32,
    pub hardness: f32,
}

#[derive(Debug)]
pub struct Particle<ParticleType: HasParticleProperties> {
    // Current and previous position switch places.
    // Velocity doesn't need to be stored explicitly for verlet integration
    position: [Vector2D; 2],
    particle_type: Rc<ParticleType>,
}

impl<T: HasParticleProperties> Clone for Particle<T> {
    fn clone(&self) -> Self {
        Particle::<T> {
            position: self.position,
            particle_type: self.particle_type.clone()
        }
    }
}

impl<T:HasParticleProperties> HasGravityMass for Particle<T> {
    fn get_gravity_mass(&self) -> f32 {
        self.particle_type.get_gravity_mass()
    }
}

impl<T: HasParticleProperties> HasParticleProperties for Particle<T> {
    fn get_inertia_mass(&self) -> f32 {
        self.particle_type.get_inertia_mass()
    }
    fn get_d0(&self) -> f32 {
        self.particle_type.get_d0()
    }
    fn get_hardness(&self) -> f32{
        self.particle_type.get_hardness()
    }
}

impl<T: HasParticleProperties> HasPosition for Particle<T> {
    fn get_position(&self, step: u8) -> Vector2D {
        self.position[step as usize]
    }
}

impl HasGravityMass for BasicParticleType {
    fn get_gravity_mass(&self) -> f32 {
        self.gravity_mass
    }
}

impl HasParticleProperties for BasicParticleType {
    fn get_inertia_mass(&self) -> f32 {
        self.inertia_mass
    }
    fn get_d0(&self) -> f32 {
        self.d0
    }
    fn get_hardness(&self) -> f32{
        self.hardness
    }
}

impl<T: HasParticleProperties> Particle<T> {
    pub fn new(position: Vector2D, velocity: Vector2D, step: u8, particle_type: Rc<T>) -> Particle<T> {
        Particle {
            position:  if step == 0 {
                    [position, position - velocity * DT]
                }
                else {
                    [position - velocity * DT, position]
                },
            particle_type: particle_type
        }
    }

    /// One step of Verlet integration on the particle based on the forces.
    /// Changes the previous position into the next position.
    pub fn update(&mut self, forces: Vector2D, step: u8) {
        let next_step = (step + 1) & 1;
        let pos1 = self.position[step as usize];
        let pos2 = self.position[next_step as usize];

        self.position[next_step as usize] = pos1 * (2.0 - FRICTION) -
                                            pos2 * (1.0 - FRICTION) +
                                            forces * (DT * DT  / self.get_inertia_mass());
    }

    pub fn get_particle_type(&self) -> &T {
        &self.particle_type
    }
}

use geometry::*;
use physics::traits::*;
use physics::forces::FRICTION;

// Time step.
static DT: f32 = 0.01;

#[derive(PartialEq,Clone,Copy,Debug)]
pub struct BasicParticleType {
    pub inertia_mass: f32,
    pub gravity_mass: f32,
    pub d0: f32,
    pub hardness: f32,
}

#[derive(Debug,Clone,Copy)]
pub struct Particle<'a, ParticleType: HasParticleProperties + 'a> {
    // Current and previous position switch places.
    // Velocity doesn't need to be stored explicitly for verlet integration
    position: [Vector2D; 2],
    particle_type: &'a ParticleType,
}

impl<'a, T:HasParticleProperties> HasGravityMass for Particle<'a, T> {
    fn get_gravity_mass(&self) -> f32 {
        self.particle_type.get_gravity_mass()
    }
}

impl<'a, T: HasParticleProperties> HasParticleProperties for Particle<'a, T> {
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

impl<'a, T: HasParticleProperties> HasPosition for Particle<'a, T> {
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

impl<'a, T: HasParticleProperties> Particle<'a, T> {
    pub fn new(position: Vector2D, velocity: Vector2D, step: u8, particle_type: &T) -> Particle<T> {
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
        self.particle_type
    }
}

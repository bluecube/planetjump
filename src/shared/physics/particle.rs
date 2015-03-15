use geometry::*;
use physics::traits::*;

pub struct BasicParticleType {
    inertia_mass: f32,
    gravity_mass: f32,
    d0: f32,
    hardness: f32,
}

//pub struct GfxParticleType {
//    base: BasicParticleType,
//    pub texture: ParticleTexture
//}

#[derive(Debug)]
pub struct Particle<'a, ParticleType: HasParticleProperties + 'a> {
    // Current and previous position switch places.
    // Velocity doesn't need to be stored explicitly for verlet integration
    position: [Vector2D; 2],
    particle_type: &'a ParticleType,
}

impl<'a, T:HasParticleProperties> Copy for Particle<'a, T> { }

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
    /// One step of Verlet integration on the particle based on the forces.
    /// Changes the previous position into the next position.
    pub fn update(&mut self, forces: Vector2D, step: u8) {
        let next_step = (step + 1) & 1;
        let acceleration = forces / self.get_inertia_mass();
        let pos1 = self.position[step as usize];
        let pos2 = self.position[next_step as usize];

        self.position[next_step as usize] = pos1 * 2.0 - pos2 + acceleration;
    }
}

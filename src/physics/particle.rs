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

pub struct Particle<'a, ParticleType: HasParticleProperties + 'a> {
    position: Vector2D,
    velocity: Vector2D,
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
    fn get_position(&self) -> Vector2D {
        self.position
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


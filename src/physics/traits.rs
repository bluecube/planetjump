use geometry::*;

pub trait HasPosition {
    fn get_position(&self, step: u8) -> Vector2D;
}

pub trait HasGravityMass {
    fn get_gravity_mass(&self) -> f32;
}

/// Represents object that can be influenced by gravity.
pub trait HasGravity: HasGravityMass + HasPosition {}

/// Represents properties of a class of particles
/// (everything but position and velocity)
pub trait HasParticleProperties: HasGravityMass {
    fn get_inertia_mass(&self) -> f32;
    fn get_d0(&self) -> f32;
    fn get_hardness(&self) -> f32;
}

// pub trait IsParticleType: HasGravityMass + HasInertiaMass {}

use physics::traits::*;
use physics::particle::*;
use geometry::*;

// Everywhere in this module positive force is separating the particles,
// negative force is pulling them together.

static GRAVITATIONAL_CONSTANT: f32 = 1.0;

fn get_gravity_scalar<T: HasGravityMass, U:HasGravityMass>(x1: &T, x2: &U, distance: f32) -> f32 {
    let m1 = x1.get_gravity_mass();
    let m2 = x2.get_gravity_mass();
    -GRAVITATIONAL_CONSTANT * m1 * m2 / (distance * distance)
}

fn normalized<T: HasPosition, U: HasPosition>(x1: &T, x2: &U) -> (Vector2D, f32) {
    (x2.get_position() - x1.get_position()).normalized2()
}

fn max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

pub fn get_gravity_vector<T: HasGravity, U: HasGravity>(x1: &T, x2: &U) -> Vector2D {
    let (direction, distance) = normalized(x1, x2);
    direction * get_gravity_scalar(x1, x2, distance)
}

pub fn get_force_vector<'a, ParticleType: HasParticleProperties>(x1: &Particle<'a, ParticleType>,
                                                                 x2: &Particle<'a, ParticleType>) -> Vector2D {
    let (direction, distance) = normalized(x1, x2);

    let gravity = -get_gravity_scalar(x1, x2, distance);

    let hardness = x1.get_hardness() * x2.get_hardness();
    let d0 = x1.get_d0() + x2.get_d0();
    let close_forces = hardness * (d0 - distance);

    direction * max(gravity, close_forces)
}

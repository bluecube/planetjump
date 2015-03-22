use physics::traits::*;
use physics::particle::Particle;
use geometry::*;

// Everywhere in this module negative force is separating the particles,
// positive force is pulling them together.

static GRAVITATIONAL_CONSTANT: f32 = 0.5;

/// This fraction of velocity is substracted from particle speed every tick.
pub static FRICTION: f32 = 1e-3;

pub fn get_gravity_scalar(m1: f32, m2: f32, distance: f32) -> f32 {
    GRAVITATIONAL_CONSTANT * m1 * m2 / (distance * distance)
}

fn min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

/// Force that x2 exterts on x1
pub fn get_force_vector<'a, ParticleType: HasParticleProperties>(x1: &Particle<'a, ParticleType>,
                                                                 x2: &Particle<'a, ParticleType>,
                                                                 step: u8) -> Vector2D {

    // TODO: Convert this to polynomial

    let pos1 = x1.get_position(step);
    let pos2 = x2.get_position(step);

    if pos1 == pos2 {
        return Vector2D::zero();
    }

    let (direction, distance) = (pos2 - pos1).normalized2();

    let m1 = x1.get_gravity_mass();
    let m2 = x2.get_gravity_mass();
    let gravity = get_gravity_scalar(m1, m2, distance);

    let hardness = x1.get_hardness() * x2.get_hardness();
    let d0 = x1.get_d0() + x2.get_d0();
    let close_forces = hardness * (distance - d0);

    direction * min(gravity, close_forces)
}

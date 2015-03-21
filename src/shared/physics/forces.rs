use physics::traits::*;
use physics::particle::*;
use geometry::*;

static EPSILON: f32 = 1e-3;

// Everywhere in this module positive force is separating the particles,
// negative force is pulling them together.

static GRAVITATIONAL_CONSTANT: f32 = 1.0;

pub fn get_gravity_scalar(m1: f32, m2: f32, distance: f32) -> f32 {
    -GRAVITATIONAL_CONSTANT * m1 * m2 / (distance * distance)
}

fn max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

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
    let gravity = -get_gravity_scalar(m1, m2, distance);

    let hardness = x1.get_hardness() * x2.get_hardness();
    let d0 = x1.get_d0() + x2.get_d0();
    let close_forces = hardness * (d0 - distance);

    direction * max(gravity, close_forces)
}

use physics::particle::*;
use physics::traits::*;
use geometry::*;
use physics::forces::*;

use std::cell::UnsafeCell;

pub enum Tree<'a, ParticleType: HasParticleProperties + 'a> {
    InnerNode (InnerNode<'a, ParticleType>),
    LeafNode (Particle<'a, ParticleType>)
}

struct InnerNodeState {
    com: Vector2D,
    radius: f32,
}

pub struct InnerNode<'a, ParticleType: HasParticleProperties + 'a> {
    state: [InnerNodeState; 2],
    gravity_mass: f32,
    children: [Box<Tree<'a, ParticleType> >; 2]
}

impl<'a, T: HasParticleProperties> Tree<'a, T> {
    /// Calculates forces that every particle exterts on every other particle
    /// and updates particles accordingly.
    pub fn update(&mut self, step: u8) {
        unsafe {
            let cell = UnsafeCell::new(self);
            let a: &mut Tree<T> = *cell.get();
            let b: &Tree<T> = *cell.get();

            a.update_internal(b, step);
        }
    }

    /// Calculates forces that `tree` exterts on every particle from `self`
    /// and updates particles in tree.
    /// TODO: Rewrite this to use explicit stack.
    ///
    /// *NOTE*
    /// `self` and `tree` passed to this function alias each other!
    fn update_internal(&mut self, tree: &Tree<T>, step: u8) -> (Vector2D, f32, f32) {
        let next_step = (step + 1) & 1;
        match self {
            &mut Tree::InnerNode(ref mut node) => {
                let (com1, m1, r1) = node.children[0].update_internal(tree, step);
                let (com2, m2, r2) = node.children[1].update_internal(tree, step);

                // TODO: Optimize this
                let com = (com1 * m1 + com2 * m2) / (m1 + m2);
                let new_r1 = r1 + (com - com1).len();
                let new_r2 = r2 + (com - com2).len();
                let radius = if new_r1 > new_r2 { new_r1 } else { new_r2 };

                let state = &mut node.state[next_step as usize];

                state.com = com;
                state.radius = radius;

                return (com, node.gravity_mass, radius)
            },
            &mut Tree::LeafNode(ref mut particle) => {
                let forces = tree.collect_forces(particle, step);
                particle.update(forces, step);

                return (particle.get_position(next_step),
                        particle.get_gravity_mass(),
                        2.0 * particle.get_d0());
            }
        }
    }

    /// Calculate force that all particles in `self` extert on `particle`.
    /// TODO: Rewrite this to use explicit stack.
    fn collect_forces(&self, particle: &Particle<T>, step: u8) -> Vector2D {
        match self {
            &Tree::InnerNode(ref node) => {
                let p1 = particle.get_position(step);
                let p2 = node.state[step as usize].com;
                let (direction, distance) = (p2 - p1).normalized2();

                if must_open(particle, &node, distance, step) {
                    return node.children[0].collect_forces(particle, step) +
                           node.children[1].collect_forces(particle, step);
                }
                else {
                    let m1 = particle.get_gravity_mass();
                    let m2 = node.gravity_mass;
                    return direction * get_gravity_scalar(m1, m2, distance);
                }
            },
            &Tree::LeafNode(ref other_particle) => {
                return get_force_vector(particle, other_particle, step);
            },
        }
    }
}

fn must_open<T>(particle: &Particle<T>, node: &InnerNode<T>, distance: f32, step: u8) -> bool
    where T: HasParticleProperties {

    let radius = node.state[step as usize].radius;
    let min_distance_radius = 2.0 * particle.get_d0() + radius;
    let min_distance_angle = radius / 0.05;

    distance < min_distance_radius || distance < min_distance_angle
}


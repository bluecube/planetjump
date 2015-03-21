use physics::particle::*;
use physics::traits::*;
use geometry::*;
use physics::forces::*;

use std::cell::UnsafeCell;

static K_MEANS_ITERATIONS: usize = 4;

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
    pub left_child: Box<Tree<'a, ParticleType>>,
    pub right_child: Box<Tree<'a, ParticleType>>,
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
                let (com1, m1, r1) = node.left_child.update_internal(tree, step);
                let (com2, m2, r2) = node.right_child.update_internal(tree, step);

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
                    return node.left_child.collect_forces(particle, step) +
                           node.right_child.collect_forces(particle, step);
                }
                else {
                    let m1 = particle.get_gravity_mass();
                    let m2 = node.gravity_mass;
                    return direction * get_gravity_scalar(m1, m2, distance);
                }
            },
            &Tree::LeafNode(ref other_particle) => {
                let force = get_force_vector(particle, other_particle, step);
                return force;
            }
        }
    }

    fn destroy(self, output: &mut Vec<Particle<'a, T>>) {
        match self {
            Tree::InnerNode(node) => {
                node.left_child.destroy(output);
                node.right_child.destroy(output);
            },
            Tree::LeafNode(particle) => output.push(particle)
        }
    }

    pub fn rebuild(self, step: u8) -> Tree<'a, T> {
        let mut particles = Vec::<Particle<T>>::new();
        self.destroy(&mut particles);
        Tree::<T>::new(particles, step)
    }

    pub fn new(mut particles: Vec<Particle<'a, T>>, step: u8) -> Tree<'a, T> {
        Tree::build(particles.as_mut_slice(), step)
    }

    /// This function attempts to split the particles into two groups
    /// using algorithm based on k-means clustering.
    ///
    /// Basically we have two candidate points, then in each step we assign every
    /// particle to the closest point and calculate center of weight of both groups.
    /// The centers of mass then serve as candidates for next step.
    /// Two inital points are taken as the first and last particle's coordinates.
    fn build(particles: &mut [Particle<'a, T>], step: u8) -> Tree<'a, T> {
        let len = particles.len();

        assert!(len > 0);
        if len == 1 {
            return Tree::LeafNode(particles[0]); // Particle is Copy
        }

        let mut pos1 = particles[0].get_position(step);
        let mut pos2 = particles[len - 1].get_position(step);
        let mut separation = 0;
        let mut m1 = 0.0;
        let mut m2 = 0.0;

        for i in 0..K_MEANS_ITERATIONS {
            let mut new_pos1 = Vector2D::zero();
            let mut new_pos2 = Vector2D::zero();
            separation = 0;
            m1 = 0.0;
            m2 = 0.0;
            for j in 0..particles.len() {
                let particle_position = particles[j].get_position(step);
                let particle_mass = particles[j].get_gravity_mass();

                let dist1 = (particle_position - pos1).len_squared();
                let dist2 = (particle_position - pos2).len_squared();

                if dist1 < dist2 {
                    new_pos1 = new_pos1 + particle_position * particle_mass;
                    m1 = m1 + particle_mass;

                    // TODO: We should use more stable partitioning algorithm, so
                    // that the array doesn't shuffle much when converging.
                    particles.swap(j, separation);
                    separation += 1;
                }
                else {
                    new_pos2 = new_pos2 + particle_position * particle_mass;
                    m2 = m2 + particle_mass;
                }
            }
            pos1 = new_pos1 / m1;
            pos2 = new_pos2 / m2;
        }

        if 10 * separation < len || 10 * separation > 9 * len {
            // avoid too imbalanced tree
            separation = len / 2;
        }

        let (left_particles, right_particles) = particles.split_at_mut(separation);
        let left = Tree::build(left_particles, step);
        let right = Tree::build(right_particles, step);

        let m = m1 + m2;
        let pos = (pos1 * m1 + pos2 * m2) / m;
        let node = InnerNode { state: [ InnerNodeState { com: pos, radius: 100.0 },
                                        InnerNodeState { com: pos, radius: 100.0 } ],
                               gravity_mass: m,
                               left_child: Box::new(left),
                               right_child: Box::new(right) };
        Tree::InnerNode(node)
    }
}

fn must_open<T>(particle: &Particle<T>, node: &InnerNode<T>, distance: f32, step: u8) -> bool
    where T: HasParticleProperties {

    let radius = node.state[step as usize].radius;
    let min_distance_radius = 2.0 * particle.get_d0() + radius;
    let min_distance_angle = radius / 0.05;

    distance < min_distance_radius || distance < min_distance_angle
}


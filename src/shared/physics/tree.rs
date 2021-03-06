use physics::particle::*;
use physics::traits::*;
use geometry::*;
use physics::forces::*;

use std::cell::UnsafeCell;
use std::cmp::Ordering;

#[derive(Debug)]
pub enum Tree<ParticleType: HasParticleProperties> {
    InnerNode (InnerNode<ParticleType>),
    LeafNode (Particle<ParticleType>)
}

#[derive(Debug)]
pub struct InnerNode<ParticleType: HasParticleProperties> {
    pub bounding_box: [BoundingBox; 2],
    gravity_mass: f32,
    pub left_child: Box<Tree<ParticleType>>,
    pub right_child: Box<Tree<ParticleType>>,
}

impl<T: HasParticleProperties> Tree<T> {
    /// Calculates forces that every particle exterts on every other particle
    /// and updates particles accordingly.
    pub fn update(&mut self, step: u8) -> u32 {
        unsafe {
            let cell = UnsafeCell::new(self);
            let a: &mut Tree<T> = *cell.get();
            let b: &Tree<T> = *cell.get();

            let (bbox, splits) = a.update_internal(b, step);
            splits
        }
    }

    /// Calculates forces that `tree` exterts on every particle from `self`
    /// and updates particles in tree.
    /// TODO: Rewrite this to use explicit stack.
    ///
    /// *NOTE*
    /// `self` and `tree` passed to this function alias each other!
    fn update_internal(&mut self, tree: &Tree<T>, step: u8) -> (BoundingBox, u32) {
        let next_step = (step + 1) & 1;
        match self {
            &mut Tree::InnerNode(ref mut node) => {
                let (bbox1, splits1) = node.left_child.update_internal(tree, step);
                let (bbox2, splits2) = node.right_child.update_internal(tree, step);

                // TODO: Optimize this
                let bbox = BoundingBox::combine(&bbox1, &bbox2);

                node.bounding_box[next_step as usize] = bbox;

                return (bbox, splits1 + splits2);
            },
            &mut Tree::LeafNode(ref mut particle) => {
                let (forces, splits) = tree.collect_forces(particle, step);
                particle.update(forces, step);

                let bbox = BoundingBox::from_radius(particle.get_position(next_step),
                                                    2.0 * particle.get_d0());

                return (bbox, splits);
            }
        }
    }

    /// Calculate force that all particles in `self` extert on `particle`.
    /// TODO: Rewrite this to use explicit stack.
    fn collect_forces(&self, particle: &Particle<T>, step: u8) -> (Vector2D, u32) {
        match self {
            &Tree::InnerNode(ref node) => {
                let p1 = particle.get_position(step);
                let p2 = node.bounding_box[step as usize].get_center();
                let (direction, distance) = (p2 - p1).normalized2();

                if must_open(particle, &node, distance, step) {
                    let (forces1, splits1) = node.left_child.collect_forces(particle, step);
                    let (forces2, splits2) = node.right_child.collect_forces(particle, step);

                    return (forces1 + forces2, splits1 + splits2);
                }
                else {
                    let m1 = particle.get_gravity_mass();
                    let m2 = node.gravity_mass;
                    let force = direction * get_gravity_scalar(m1, m2, distance);
                    return (force, 1);
                }
            },
            &Tree::LeafNode(ref other_particle) => {
                let force = get_force_vector(particle, other_particle, step);
                return (force, 1);
            }
        }
    }

    fn destroy(self, output: &mut Vec<Particle<T>>) {
        match self {
            Tree::InnerNode(node) => {
                node.left_child.destroy(output);
                node.right_child.destroy(output);
            },
            Tree::LeafNode(particle) => output.push(particle)
        }
    }

    pub fn rebuild(self, step: u8) -> Tree<T> {
        let mut particles = Vec::<Particle<T>>::new();
        self.destroy(&mut particles);
        Tree::<T>::new(particles, step)
    }

    pub fn new(mut particles: Vec<Particle<T>>, step: u8) -> Tree<T> {
        Tree::build(&mut particles[..], step)
    }

    fn build(particles: &mut [Particle<T>], step: u8) -> Tree<T> {
        let len = particles.len();

        assert!(len > 0);
        if len == 1 {
            return Tree::LeafNode(particles[0].clone());
        }

        let mut m = 0.0;
        let mut bbox = BoundingBox::empty();
        for particle in particles.iter() {
            m += particle.get_gravity_mass();
            bbox.expand(particle.get_position(step));
        }

        let bbox_size = bbox.get_size();
        let split_axis = (if bbox_size.x > bbox_size.y { 0 } else { 1 }) as usize;

        particles.sort_by(|a, b| float_cmp(a.get_position(step)[split_axis],
                                           b.get_position(step)[split_axis]));
        let (left_particles, right_particles) = particles.split_at_mut(len / 2);

        let left = Tree::build(left_particles, step);
        let right = Tree::build(right_particles, step);

        let node = InnerNode { bounding_box: [ bbox, bbox ],
                               gravity_mass: m,
                               left_child: Box::new(left),
                               right_child: Box::new(right) };
        Tree::InnerNode(node)
    }

}

fn float_cmp(a: f32, b: f32) -> Ordering {
    a.partial_cmp(&b).unwrap_or(Ordering::Equal)
}

fn must_open<T>(particle: &Particle<T>, node: &InnerNode<T>, distance: f32, step: u8) -> bool
    where T: HasParticleProperties {

    // TODO: Get rid of sqrt.
    let radius = node.bounding_box[step as usize].get_size().len();
    let min_distance_radius = 2.0 * particle.get_d0() + radius;
    let min_distance_angle = radius / 0.05;

    distance < min_distance_radius || distance < min_distance_angle
}

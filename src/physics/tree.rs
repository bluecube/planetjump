use physics::particle::*;
use physics::traits::*;
use geometry::*;

pub enum Tree<'a, ParticleType: HasParticleProperties + 'a> {
    InnerNode (InnerNode<'a, ParticleType>),
    LeafNode (Particle<'a, ParticleType>)
}

pub struct InnerNode<'a, ParticleType: HasParticleProperties + 'a> {
    bbox: BoundingBox,
    m: f32,
    children: [Box<Tree<'a, ParticleType> >; 2]
}

impl<'a, T:HasParticleProperties> HasPosition for InnerNode<'a, T> {
    fn get_position(&self) -> Vector2D {
        self.bbox.get_center()
    }
}

impl<'a, T:HasParticleProperties> HasGravityMass for InnerNode<'a, T> {
    fn get_gravity_mass(&self) -> f32 {
        self.m
    }
}

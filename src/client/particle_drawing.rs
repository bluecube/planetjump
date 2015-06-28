extern crate shared;
extern crate sdl2;

use gfx_particle_type::*;
use self::shared::physics::tree::*;
use self::shared::physics::traits::*;
use sdl2::render::Renderer;

pub fn draw_particles(tree: &Tree<GfxParticleType>, step: u8, renderer: &mut Renderer) {
    match tree {
        &Tree::InnerNode(ref node) => {
            draw_particles(&node.left_child, step, renderer);
            draw_particles(&node.right_child, step, renderer);
        }
        &Tree::LeafNode(ref particle) => {
            let position = particle.get_position(step);
            let particle_type = particle.get_particle_type();
            particle_type.draw(renderer, position.x as i32, position.y as i32)
        }
    }
}

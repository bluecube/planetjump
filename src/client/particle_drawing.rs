use gfx_particle_type::*;
use shared::physics::tree::*;
use shared::physics::traits::*;
use sdl2::render::RenderDrawer;

pub fn draw_particles(tree: &Tree<GfxParticleType>, step: u8, drawer: &mut RenderDrawer) {
    match tree {
        &Tree::InnerNode(ref node) => {
            draw_particles(&node.left_child, step, drawer);
            draw_particles(&node.right_child, step, drawer);
        }
        &Tree::LeafNode(ref particle) => {
            let position = particle.get_position(step);
            let particle_type = particle.get_particle_type();
            particle_type.draw(drawer, position.x as i32, position.y as i32)
        }
    }
}

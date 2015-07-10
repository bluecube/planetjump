use std::rc::Rc;

use sdl2;

use shared::physics::particle::*;
use shared::physics::traits::*;
use shared::particle_definitions::*;

use raytrace;

use texture_generator;

pub struct GfxParticleType {
    base: BasicParticleType,
    texture: sdl2::render::Texture,
    half_texture_size: u32
}

impl HasGravityMass for GfxParticleType {
    fn get_gravity_mass(&self) -> f32 {
        self.base.get_gravity_mass()
    }
}

impl HasParticleProperties for GfxParticleType {
    fn get_inertia_mass(&self) -> f32 {
        self.base.get_inertia_mass()
    }
    fn get_d0(&self) -> f32 {
        self.base.get_d0()
    }
    fn get_hardness(&self) -> f32{
        self.base.get_hardness()
    }
}

impl GfxParticleType {
    fn new<'a>(renderer: &'a sdl2::render::Renderer,
               definition: ParticleTypeDefinition) -> GfxParticleType {
        let (texture, half_size) = GfxParticleType::make_texture(renderer,
                                                                 definition.base.get_d0(),
                                                                 definition.color);

        GfxParticleType { base: definition.base,
                          texture: texture,
                          half_texture_size: half_size }
    }

    /// Renders texture for a particle.
    /// `particle_size` is the logical particle size, the texture will be larger.
    ///
    /// Returns tuple with the (square) texture and half of its size (offset for drawing).
    ///
    /// Panicks if texture creation fails.
    fn make_texture<'a>(renderer: &'a sdl2::render::Renderer,
                        particle_size: f32,
                        color: (u8, u8, u8, u8)) -> (sdl2::render::Texture, u32) {
        let (r, g, b, a) = color;
        let size = (2.0 * 4.0 * particle_size).round() as u32;
        let inner_threshold = 0.15 * 0.15;
        let outer_threshold = 0.5 * 0.5;

/*
        let texture = texture_generator::generate(renderer,
                                                  size, size,
                                                  |x, y| {
                                                      let r2 = x * x + y * y;
                                                      let alpha = if r2 < inner_threshold {
                                                           a
                                                      }
                                                      else if r2 < outer_threshold {
                                                          let a_float = a as f32;
                                                          let multiplier = (outer_threshold - r2) / (outer_threshold - inner_threshold);
                                                          (a_float * multiplier / 2.0) as u8
                                                      }
                                                      else {
                                                          0
                                                      };
                                                      (r, g, b, alpha)
                                                  }).unwrap();*/
        let scene = raytrace::objects::Sphere().transform(raytrace::geometry::Vector3D::new(0, 0, 10));
        let texture = texture_generator::generate(renderer, size, size,
            |x, y| raytrace::render(scene, 1.0, x, y));
                
        (texture, size)
    }

    pub fn draw(&self, renderer: &mut sdl2::render::Renderer, x: i32, y: i32) {
        let r = self.half_texture_size as i32;
        if x < 0 || x > 1000 || y < 0 || y > 1000 {
            return;
        }
        renderer.copy(&self.texture,
                      None,
                      Some(sdl2::rect::Rect::new_unwrap(x - r,
                                                        y - r,
                                                        2 * r as u32,
                                                        2 * r as u32)));
    }
}

pub fn load_particle_types<'a>(renderer: &'a sdl2::render::Renderer,
                               definitions: Vec<ParticleTypeDefinition>) -> Vec<GfxParticleType> {
    definitions.into_iter().map(|definition| GfxParticleType::new(renderer, definition)).collect()
}

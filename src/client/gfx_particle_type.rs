use std::num::Float;

use sdl2::render::{Renderer, RenderDrawer, Texture, BlendMode};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::*;

use shared::physics::particle::*;
use shared::physics::traits::*;

use shared::particle_definitions::*;

pub struct GfxParticleType<'a> {
    base: BasicParticleType,
    texture: Texture<'a>,
    half_texture_size: u32
}

impl<'a> HasGravityMass for GfxParticleType<'a> {
    fn get_gravity_mass(&self) -> f32 {
        self.base.get_gravity_mass()
    }
}

impl<'a> HasParticleProperties for GfxParticleType<'a> {
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

impl<'a> GfxParticleType<'a> {
    fn new(renderer: &'a Renderer, definition: ParticleTypeDefinition) -> GfxParticleType<'a> {
        let (texture, half_size) = GfxParticleType::make_texture(renderer,
                                                                 definition.base.get_d0(),
                                                                 definition.color);

        GfxParticleType { base: definition.base,
                          texture: texture,
                          half_texture_size: half_size }
    }

    /// Renders texture for a particle.
    /// `size` is the logical particle size, the texture will be larger.
    ///
    /// Returns tuple with the (square) texture and half of its size (offset for drawing).
    ///
    /// Panicks if texture creation fails.
    fn make_texture(renderer: &'a Renderer, size: f32, color: (u8, u8, u8, u8)) -> (Texture<'a>, u32) {
        let (r, g, b, a) = color;
        let inside_radius = size.round() as u32;
        let glow_radius = 4 * inside_radius;

        let size = 2 * glow_radius;
        let glow_threshold = glow_radius * glow_radius;
        let inside_threshold = inside_radius * inside_radius;
        let pitch = size * 4;
        let mut pixels = Vec::with_capacity((size * pitch) as usize);
        for y in 0..size {
            let dy = y as i32 - glow_radius as i32;
            for x in 0..size {
                let dx = x as i32 - glow_radius as i32;

                // This is a little more complicated, bexause the expression
                // is actually an expansion of (dx + 0.5)**2 + (dy + 0.5)**2 - 0.5
                let r2 = (dx * dx + dy * dy + dx + dy) as u32;

                let alpha = if r2 < inside_threshold {
                    a
                }
                else if r2 < glow_threshold {
                    let a32 = a as u32;
                    let tmp1 = glow_threshold - r2;
                    let tmp2 = glow_threshold - inside_threshold;
                    ((a32 * tmp1 * tmp1) / (2 * tmp2 * tmp2)) as u8
                }
                else {
                    0
                };

                // Why does this have to be reversed? Who knows.
                pixels.push(alpha);
                pixels.push(b);
                pixels.push(g);
                pixels.push(r);
            }
        }

        let mut texture = renderer.create_texture_static(PixelFormatEnum::RGBA8888,
                                                         (size as i32, size as i32)).unwrap();
        texture.set_blend_mode(BlendMode::Blend);
        texture.update(None, &pixels, pitch as i32).unwrap();

        (texture, glow_radius)
    }

    pub fn draw(&self, drawer: &mut RenderDrawer, x: i32, y: i32) {
        let r = self.half_texture_size as i32;
        if x < 0 || x > 1000 || y < 0 || y > 1000 {
            return;
        }
        drawer.copy(&self.texture, None, Some(Rect::new(x - r, y - r, 2 * r, 2 * r)));
    }
}

pub fn load_particle_types<'a>(renderer: &'a Renderer,
                               definitions: Vec<ParticleTypeDefinition>) -> Vec<GfxParticleType<'a>> {
    definitions.into_iter().map(|definition| GfxParticleType::new(renderer, definition)).collect()
}
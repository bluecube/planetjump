extern crate shared;

use std::rc::Rc;

use sdl2;

use self::shared::physics::particle::*;
use self::shared::physics::traits::*;

use self::shared::particle_definitions::*;

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
    /// `size` is the logical particle size, the texture will be larger.
    ///
    /// Returns tuple with the (square) texture and half of its size (offset for drawing).
    ///
    /// Panicks if texture creation fails.
    fn make_texture<'a>(renderer: &'a sdl2::render::Renderer,
                        size: f32,
                        color: (u8, u8, u8, u8)) -> (sdl2::render::Texture, u32) {
        let (r, g, b, a) = color;
        let inside_radius = size.round() as u32;
        let glow_radius = 4 * inside_radius;

        let size = 2 * glow_radius;
        let glow_threshold = glow_radius * glow_radius;
        let inside_threshold = inside_radius * inside_radius;
        let pitch = size as usize * 4;
        let mut pixels = Vec::with_capacity(size as usize * pitch);
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

        let mut texture = renderer.create_texture_static(sdl2::pixels::PixelFormatEnum::RGBA8888,
                                                         (size, size)).unwrap();
        texture.set_blend_mode(sdl2::render::BlendMode::Blend);
        texture.update(None, &pixels, pitch).unwrap();

        (texture, glow_radius)
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
                               definitions: Vec<ParticleTypeDefinition>) -> Vec<Rc<GfxParticleType>> {
    definitions.into_iter().map(|definition| Rc::new(GfxParticleType::new(renderer, definition))).collect()
}

extern crate sdl2;

use self::sdl2::render::{Renderer, RenderDrawer, Texture, BlendMode};
use self::sdl2::pixels::{Color, PixelFormatEnum};
use self::sdl2::rect::*;

pub struct ParticleTexture<'a> {
    texture: Texture<'a>,
    radius: u32,
}

impl<'a> ParticleTexture<'a> {
    pub fn new(renderer: &Renderer, radius: u32, base_color: Color) -> ParticleTexture {
        let (r, g, b, a) = match base_color {
            Color::RGB(r, g, b) => (r, g, b, 255),
            Color::RGBA(r, g, b, a) => (r, g, b, a),
        };

        let size = 2 * radius;
        let radius2 = radius * radius;
        let pitch = size * 4;
        let mut pixels = Vec::with_capacity((size * pitch) as usize);
        for y in 0..size {
            let dy = y - radius;
            for x in 0..size {
                let dx = x - radius;

                let r2 = dx * dx + dy * dy + dx + dy;// + 0.5

                if r2 < radius2 {
                    // Why does this have to be reversed? Who knows.
                    pixels.push(a);
                    pixels.push(b);
                    pixels.push(g);
                    pixels.push(r);
                }
                else {
                    pixels.push(0);
                    pixels.push(0);
                    pixels.push(0);
                    pixels.push(0);
                }
            }
        }


        let mut texture = renderer.create_texture_static(PixelFormatEnum::RGBA8888,
                                                         (size as i32, size as i32)).unwrap();
        texture.set_blend_mode(BlendMode::Blend);
        texture.update(None, &pixels, pitch as i32).unwrap();

        ParticleTexture { texture: texture, radius: radius }
    }

    pub fn draw(&self, drawer: &mut RenderDrawer, position: (i32, i32)) {
        let (x, y) = position;
        let r = self.radius as i32;
        drawer.copy(&self.texture, None, Some(Rect::new(x - r, y - r, 2 * r, 2 * r)));
    }
}


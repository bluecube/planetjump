use sdl2;
use rand;
use rand::Rng;
use std::cmp::min;
use std::f32;

const MULTISAMPLING: u32 = 16;

pub fn generate<F>(renderer: &sdl2::render::Renderer,
                   w: u32, h: u32,
                   f: F) -> Result<sdl2::render::Texture, String>
    where F: Fn(f32, f32) -> (u8, u8, u8, u8) {

    let scale = min(w, h) as f32;

    let pitch = w as usize * 4;
    let mut pixels = Vec::with_capacity(h as usize * pitch);

    let mut rng = rand::weak_rng();

    let x_offset = w as f32 / (2.0 * scale);
    let y_offset = h as f32 / (2.0 * scale);
    let px_size = 1.0 / scale;

    for y in 0..h {
        let yy = y as f32 / scale - y_offset;
        for x in 0..w {
            let xx = x as f32 / scale - x_offset;
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            let mut a = 0;

            for i in 0..MULTISAMPLING {
                let (rx, gx, bx, ax) = f(xx + rng.gen_range(0.0, px_size),
                                         yy + rng.gen_range(0.0, px_size));
                r += rx as u32;
                g += gx as u32;
                b += bx as u32;
                a += ax as u32;
            }

            r = (r + MULTISAMPLING / 2) / MULTISAMPLING;
            g = (g + MULTISAMPLING / 2) / MULTISAMPLING;
            b = (b + MULTISAMPLING / 2) / MULTISAMPLING;
            a = (a + MULTISAMPLING / 2) / MULTISAMPLING;

            pixels.push(a as u8);
            pixels.push(b as u8);
            pixels.push(g as u8);
            pixels.push(r as u8);
        }
    }

    let mut texture = try!(renderer.create_texture_static(sdl2::pixels::PixelFormatEnum::RGBA8888,
                                                          (w, h)));
    texture.set_blend_mode(sdl2::render::BlendMode::Blend);
    try!(texture.update(None, &pixels, pitch));

    return Ok(texture);
}

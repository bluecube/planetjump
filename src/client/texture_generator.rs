use sdl2;
use std::cmp::min;
use std::f32;

pub fn generate<F>(renderer: &sdl2::render::Renderer,
                   w: u32, h: u32,
                   f: F) -> Result<sdl2::render::Texture, String>
    where F: Fn(f32, f32) -> (u8, u8, u8, u8) {

    let scale = min(w, h) as f32;

    let pitch = w as usize * 4;
    let mut pixels = Vec::with_capacity(h as usize * pitch);

    let x_offset = (w + 1) as f32 / (2.0 * scale);
    let y_offset = (h + 1) as f32 / (2.0 * scale);

    for y in 0..h {
        let yy = y as f32 / scale - y_offset;
        for x in 0..w {
            let xx = x as f32 / scale - x_offset;
            let (r, g, b, a) = f(xx, yy);

            pixels.push(a);
            pixels.push(b);
            pixels.push(g);
            pixels.push(r);
        }
    }

    let mut texture = try!(renderer.create_texture_static(sdl2::pixels::PixelFormatEnum::RGBA8888,
                                                          (w, h)));
    texture.set_blend_mode(sdl2::render::BlendMode::Blend);
    try!(texture.update(None, &pixels, pitch));

    return Ok(texture);
}

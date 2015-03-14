extern crate game;
extern crate sdl2;

mod particle;
mod gfx;

use gfx::*;

use particle::*;

use sdl2::pixels::Color;

pub fn main() {
    let gfx = Gfx::new("Game");
    let mut drawer = gfx.get_drawer();
    drawer.set_draw_color(Color::RGB(43, 53, 56));

    let particle = ParticleTexture::new(gfx.get_renderer(), 16, Color::RGBA(173, 200, 206, 128));

    for (elapsed, fps) in gfx.get_loop_iterator() {
        println!("Fps: {}", fps);
        drawer.clear();
        particle.draw(&mut drawer, (50, 50));
        particle.draw(&mut drawer, (70, 60));
        drawer.present();
    }
}

extern crate shared;
extern crate sdl2;

mod gfx;
mod gfx_particle_type;

use gfx::*;

use gfx_particle_type::*;

use sdl2::pixels::Color;

pub fn main() {
    let gfx = Gfx::new("Game");
    let mut drawer = gfx.get_drawer();
    drawer.set_draw_color(Color::RGB(43, 53, 56));

    let particle_types = load_particle_types(gfx.get_renderer(),
                                             shared::particle_definitions::particle_types());

    for (elapsed, fps) in gfx.get_loop_iterator() {
        println!("Fps: {}", fps);
        drawer.clear();
        particle_types[0].draw(&mut drawer, 100, 100);
        drawer.present();
    }
}

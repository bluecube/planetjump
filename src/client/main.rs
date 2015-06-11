extern crate shared;
extern crate sdl2;
extern crate rand;

mod gfx_particle_type;
mod particle_drawing;
mod fps_limiter;
mod states;
mod font;
mod colors;

use gfx_particle_type::*;
use shared::geometry::*;
use shared::physics::particle::*;
use fps_limiter::*;

use sdl2::pixels::Color;

fn add_particles<'a>(count: u32,
                     position: Vector2D, pos_range: f32,
                     velocity: Vector2D, vel_range: f32,
                     particle_type: &'a GfxParticleType,
                     out: &mut Vec<Particle<'a, GfxParticleType>>) {
    let mut rng = rand::weak_rng();

    for _ in 0..count {
        let pos = Vector2D::random_radius(&mut rng, position, pos_range);
        let vel = Vector2D::random_radius(&mut rng, velocity, vel_range);

        let p = Particle::<'a, GfxParticleType>::new(pos, vel, 0, particle_type);

        out.push(p);
    }
}

pub fn main() {
    let mut sdl_context = sdl2::init().video().unwrap();
    let window = sdl_context.window("Planet Jump", 800, 600)
        .resizable()
        .build()
        .unwrap();
    let mut renderer = window.renderer()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();

    let particle_types = load_particle_types(&renderer,
                                             shared::particle_definitions::particle_types());

    let mut particles = Vec::<Particle<GfxParticleType>>::new();
    add_particles(100,
                  Vector2D::new(400.0, 400.0), 40.0,
                  Vector2D::new(0.0, -1.0), 0.1,
                  &particle_types[0],
                  &mut particles);
    add_particles(70,
                  Vector2D::new(600.0, 400.0), 20.0,
                  Vector2D::new(0.0, 20.0), 0.1,
                  &particle_types[0],
                  &mut particles);
    add_particles(50,
                  Vector2D::new(700.0, 300.0), 20.0,
                  Vector2D::new(5.0, 10.0), 0.1,
                  &particle_types[0],
                  &mut particles);
//    add_particles(1,
//                  Vector2D::new(200.0, 200.0), 0.0,
//                  Vector2D::zero(), 0.0,
//                  &particle_types[0],
//                  &mut particles);
//    add_particles(1,
//                  Vector2D::new(210.0, 200.0), 0.0,
//                  Vector2D::new(0.0, 0.05), 0.0,
//                  &particle_types[1],
//                  &mut particles);

//    tree.update(step);

    let mut event_pump = sdl_context.event_pump();
    let mut drawer = renderer.drawer();

    let mut state: Box<states::State> = Box::new(states::inmenu::in_main_menu());

    drawer.set_draw_color(Color::RGB(43, 53, 56));

    for elapsed in FpsLimiter::new(60) {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => return,
                _ => state.handle(event),
            }
        }

        match state.update() {
            states::UpdateResult::Stay => {},
            states::UpdateResult::Change(mut new_state) => {
                new_state.set_previous(state);
                state = new_state;
            },
            states::UpdateResult::Back(Some(new_state)) => {
                state = new_state;
                // We're not setting previous state because this already is the previous
            },
            states::UpdateResult::Back(None) => {
                return
            }
        }

        state.draw(&mut drawer);
        drawer.present();
    }
}

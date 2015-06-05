use std::iter::Iterator;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keycode::KeyCode;

static FPS: u32 = 60;
static FPS_SMOOTHING: f32 = 0.9;

pub struct Gfx<'a> {
    sdl: sdl2::Sdl,
    renderer: sdl2::render::Renderer<'a>,
}

struct GfxLoopIterator<'a> {
    event_pump: sdl2::event::EventPump<'a>,
    last_tick: u32,
    fps: f32,
}

impl<'a> Gfx<'a> {
    pub fn new(title: &str) -> Gfx<'a> {
        let sdl = sdl2::init().video().unwrap();
        let window = sdl.window(title, 800, 600)
            .resizable()
            .build()
            .unwrap();
        let renderer = window.renderer()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();

        Gfx { sdl: sdl,
              renderer: renderer }
    }

    pub fn get_renderer(&self) -> &sdl2::render::Renderer<'a> {
        &self.renderer
    }

    pub fn get_drawer<'b>(&'b mut self) -> sdl2::render::RenderDrawer<'b> {
        self.renderer.drawer()
    }

    pub fn get_loop_iterator(&mut self) -> GfxLoopIterator {
        GfxLoopIterator { event_pump: self.sdl.event_pump(),
                          last_tick: sdl2::timer::get_ticks(),
                          fps: FPS as f32 }
    }
}

/// Iterate over frames, returning miliseconds spent on last frame and smoothed FPS
impl<'a> Iterator for GfxLoopIterator<'a> {
    type Item = (u32, f32);

    fn next(&mut self) -> Option<(u32, f32)> {
        let now1 = sdl2::timer::get_ticks();
        let elapsed1 = now1 - self.last_tick;

        let interval = 1000 / FPS;
        if elapsed1 < interval {
            sdl2::timer::delay(interval - elapsed1);
        }

        let now2 = sdl2::timer::get_ticks();
        let elapsed2 = now2 - self.last_tick;
        self.last_tick = now2;
        let fps_raw = 1000.0 / (elapsed2 as f32);
        self.fps = (1.0 - FPS_SMOOTHING) * fps_raw + FPS_SMOOTHING * self.fps;

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Escape, .. } => {
                    return None
                },
                _ => {}
            }
        }

        Some((elapsed2, self.fps))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

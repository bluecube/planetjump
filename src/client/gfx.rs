use std::iter::Iterator;

use sdl2::sdl::{init, InitBuilder};
use sdl2::timer;
use sdl2::video::{Window, WindowBuilder};
use sdl2::render::{Renderer, RendererBuilder, RenderDrawer};
use sdl2::keycode::KeyCode;
use sdl2::event::{Event, EventPump};

static FPS: u32 = 60;
static FPS_SMOOTHING: f32 = 0.9;

pub struct Gfx<'a> {
    sdl: Sdl,
    renderer: Renderer<'a>,
}

struct GfxLoopIterator<'a> {
    event_pump: EventPump<'a>,
    last_tick: u32,
    fps: f32,
}

impl<'a> Gfx<'a> {
    pub fn new(title: &str) -> Gfx<'a> {
        let sdl = init().video().unwrap();
        let window = WindowBuilder::new(&sdl, title, 800, 600)
            .resizable()
            .build().unwrap();
        let renderer = RendererBuilder::new(window)
            .accelerated()
            .present_vsync()
            .build().unwrap();

        Gfx { sdl: sdl,
              renderer: renderer }
    }

    pub fn get_renderer(&self) -> &Renderer<'a> {
        &self.renderer
    }

    pub fn get_drawer<'b>(&'b mut self) -> RenderDrawer<'b> {
        self.renderer.drawer()
    }

    pub fn get_loop_iterator(&self) -> GfxLoopIterator {
        GfxLoopIterator { event_pump: self.sdl.event_pump(),
                          last_tick: timer::get_ticks(),
                          fps: FPS as f32 }
    }
}

/// Iterate over frames, returning miliseconds spent on last frame and smoothed FPS
impl<'a> Iterator for GfxLoopIterator<'a> {
    type Item = (u32, f32);

    fn next(&mut self) -> Option<(u32, f32)> {
        let now1 = timer::get_ticks();
        let elapsed1 = now1 - self.last_tick;

        let interval = 1000 / FPS;
        if elapsed1 < interval {
            timer::delay(interval - elapsed1);
        }

        let now2 = timer::get_ticks();
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

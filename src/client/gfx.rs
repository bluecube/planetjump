use std::iter::Iterator;

use sdl2::*;
use sdl2::video::{Window, WindowPos, RESIZABLE};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer, RenderDrawer};
use sdl2::keycode::KeyCode;
use sdl2::event::Event;

//static FPS: u32 = 60;
static FPS: u32 = 1;
static FPS_SMOOTHING: f32 = 0.9;

pub struct Gfx {
    sdl: Sdl,
    renderer: Renderer,
}

struct GfxLoopIterator<'a> {
    sdl: &'a Sdl,
    last_tick: u32,
    fps: f32,
}

impl Gfx {
    pub fn new(title: &str) -> Gfx {
        let sdl = init(INIT_VIDEO).unwrap();
        let window = Window::new(title,
                                 WindowPos::PosUndefined,
                                 WindowPos::PosUndefined,
                                 800, 600,
                                 RESIZABLE).unwrap();
        let renderer = Renderer::from_window(window,
                                             RenderDriverIndex::Auto,
                                             ACCELERATED).unwrap();
        Gfx { sdl: sdl,
              renderer: renderer }
    }

    pub fn get_renderer(&self) -> &Renderer {
        &self.renderer
    }

    pub fn get_drawer<'a>(&'a self) -> RenderDrawer<'a> {
        self.renderer.drawer()
    }

    pub fn get_loop_iterator(&self) -> GfxLoopIterator {
        GfxLoopIterator { sdl: &self.sdl,
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

        for event in self.sdl.event_pump().poll_iter() {
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

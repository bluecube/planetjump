extern crate sdl2;

static FPS_SMOOTHING: f32 = 0.99;

pub struct FpsLimiter {
    target_frame_time: u32,
    last_tick: u32,
    fps: f32,
}

/// Iterate over frames, returning miliseconds spent on last frame
impl Iterator for FpsLimiter {
    type Item = (u32, f32);

    fn next(&mut self) -> Option<(u32, f32)> {
        let now1 = sdl2::timer::get_ticks();
        let elapsed1 = now1 - self.last_tick;

        if elapsed1 < self.target_frame_time {
            sdl2::timer::delay(self.target_frame_time as u32 - elapsed1);
        }

        let now2 = sdl2::timer::get_ticks();
        let elapsed2 = now2 - self.last_tick;
        self.last_tick = now2;
        let fps_raw = 1000.0 / (elapsed2 as f32);
        self.fps = (1.0 - FPS_SMOOTHING) * fps_raw + FPS_SMOOTHING * self.fps;

        Some((elapsed2, self.fps))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl FpsLimiter {
    pub fn new(target_fps: u32) -> FpsLimiter {
        FpsLimiter { target_frame_time: (1000.0 / (target_fps as f32)) as u32,
                     last_tick: sdl2::timer::get_ticks(),
                     fps: target_fps as f32 }
    }
}

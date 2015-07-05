extern crate sdl2;

const FPS_SMOOTHING: f32 = 0.99;
const KP: f32 = 0.5;
const KI: f32 = 0.5;
const WINDUP_LIMIT: f32 = 1000.0;

pub struct FpsLimiter {
    target_frame_time: f32,
    last_tick: u32,
    integration_buffer: f32,
    fps: f32,
}

/// Iterate over frames, returning miliseconds spent on last frame
impl Iterator for FpsLimiter {
    type Item = (u32, f32);

    fn next(&mut self) -> Option<(u32, f32)> {
        let now = sdl2::timer::get_ticks();
        let elapsed = (now - self.last_tick) as f32;
        let error = self.target_frame_time - elapsed;

        self.integration_buffer += error; // TODO: shouldn't we use error * elapsed / target_frame_time?
        if self.integration_buffer > WINDUP_LIMIT {
            self.integration_buffer = WINDUP_LIMIT;
        }
        else if self.integration_buffer < -WINDUP_LIMIT {
            self.integration_buffer = -WINDUP_LIMIT;
        }

        self.last_tick = now;

        let to_sleep = (KP * error + KI * self.integration_buffer).round();
        if to_sleep > 0.0 {
            sdl2::timer::delay(to_sleep as u32);
        }

        let fps_raw = 1000.0 / if elapsed > 0.0 { elapsed } else { 0.5 };
            // If elapsed is 0, we assume that it is a rounding error, and it
            // should be 0.5
        self.fps = (1.0 - FPS_SMOOTHING) * fps_raw + FPS_SMOOTHING * self.fps;

        Some((elapsed as u32, self.fps))
    }
}

impl FpsLimiter {
    pub fn new(target_fps: u32) -> FpsLimiter {
        let target_frame_time = 1000.0 / (target_fps as f32);
        FpsLimiter { target_frame_time: target_frame_time,
                     last_tick: sdl2::timer::get_ticks() - (target_frame_time / 2.0).round() as u32,
                     integration_buffer: 0.0,
                     fps: target_fps as f32 }
    }
}

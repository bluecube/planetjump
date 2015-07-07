use geometry::*;

#[derive(Debug)]
pub struct InputState {
    pub aim: Vector2D,
    pub primary_fire: bool,
    pub secondary_fire: bool,
    pub jump: bool,
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            aim: Vector2D::zero(),
            primary_fire: false,
            secondary_fire: false,
            jump: false,
        }
    }
}

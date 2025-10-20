use std::collections::HashSet;
use winit::keyboard::KeyCode;
use crate::game::shape::{bind_shape, Shape};

#[derive(Default)]
pub struct Player {
    pub shape: Shape
}

pub fn bind_player() -> Player {
    Player {
        shape: bind_shape(-0.9, 0.0, 0.04, 0.25)
    }
}

impl Player {
    pub fn draw(&self) {
        self.shape.draw();
    }
}

const Y_VELOCITY: f32 = 0.01;

impl Player {
    fn move_up(&mut self) {
        self.shape.offset.1 = (self.shape.offset.1 + Y_VELOCITY).min(1.0 - self.shape.half_height - self.shape.start_location.1);
    }

    fn move_down(&mut self) {
        self.shape.offset.1 = (self.shape.offset.1 - Y_VELOCITY).max(-1.0 + self.shape.half_height - self.shape.start_location.1);
    }

    pub fn update(&mut self, keys_pressed: &HashSet<KeyCode>) {
        if keys_pressed.contains(&KeyCode::ArrowUp) {
            self.move_up()
        }
        if keys_pressed.contains(&KeyCode::ArrowDown) {
            self.move_down();
        }
    }
}
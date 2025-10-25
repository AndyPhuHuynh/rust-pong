use crate::game::{
    self, shape::{bind_shape, Shape}
};

#[derive(Default)]
pub struct Enemy {
    pub shape: Shape
}

pub fn bind_enemy() -> Enemy {
    Enemy {
        shape: bind_shape(0.9, 0.0, 0.04, 0.25)
    }
}

const Y_VELOCITY: f32 = 0.01;

impl Enemy {
    fn move_up(&mut self) {
        self.shape.offset.1 = (self.shape.offset.1 + Y_VELOCITY).min(1.0 - self.shape.half_height - self.shape.start_location.1);
    }

    fn move_down(&mut self) {
        self.shape.offset.1 = (self.shape.offset.1 - Y_VELOCITY).max(-1.0 + self.shape.half_height - self.shape.start_location.1);
    }

    pub fn update(& mut self, ball: &game::ball::Ball) {
        if ball.shape.center_y() > self.shape.center_y() {
            self.move_up();
        } else if ball.shape.center_y() < self.shape.center_y() {
            self.move_down();
        }
    }

    pub fn draw(&self) {
        self.shape.draw();
    }
}
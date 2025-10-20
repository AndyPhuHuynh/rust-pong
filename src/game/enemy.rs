use crate::game::{
    shape::{bind_shape, Shape}
};

#[derive(Default)]
pub struct Enemy {
    shape: Shape
}

pub fn bind_enemy() -> Enemy {
    Enemy {
        shape: bind_shape(0.9, 0.0, 0.04, 0.25)
    }
}

impl Enemy {
    pub fn draw(&self) {
        self.shape.draw();
    }
}
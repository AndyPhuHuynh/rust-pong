use crate::game::shape::{bind_shape, in_collision, Collision, Shape};

#[derive(Default)]
pub struct Ball {
    shape: Shape
}

pub fn bind_ball() -> Ball {
    let mut shape = bind_shape(0.0, 0.0, 0.02, 0.02);
    // shape.velocity = (0.0075, 0.005);
    shape.velocity = (0.003, 0.001);
    Ball {
        shape: shape
    } 
}

impl Ball {
    pub fn draw(&self) {
        self.shape.draw();
    }
}

impl Ball {
    fn move_x(&mut self) {
        self.shape.offset.0 += self.shape.velocity.0;
        if self.shape.offset.0 < -1.0 || self.shape.offset.0 > 1.0 {
            self.shape.velocity.0 *= -1.0;
        }
    }

    fn move_y(&mut self) {
        self.shape.offset.1 += self.shape.velocity.1;
        if self.shape.offset.1 < -1.0 || self.shape.offset.1 > 1.0 {
            self.shape.velocity.1 *= -1.0;
        }
    }

    pub fn update(&mut self, player: &Shape) {
        self.move_x();
        self.move_y();
        match in_collision(&self.shape, player) {
            Collision::Left | Collision::Right => {
                self.shape.velocity.0 *= -1.0;
            }
            Collision::Bottom | Collision::Top => {
                self.shape.velocity.1 *= -1.0;
            }
            Collision::None => {}
        }
    }
}
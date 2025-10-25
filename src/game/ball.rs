use crate::game::{shape::{bind_shape, in_collision, Collision, Shape}};

pub enum Edge {
    None,
    Left,
    Right
}

#[derive(Default)]
pub struct Ball {
    pub shape: Shape
}

pub fn bind_ball() -> Ball {
    let mut shape = bind_shape(0.0, 0.0, 0.02, 0.02);
    shape.velocity = (-0.0075, 0.005);
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
    fn move_x(&mut self) -> Edge {
        self.shape.offset.0 += self.shape.velocity.0;
        if self.shape.left_edge() < -1.0 {
            Edge::Left
        } else if self.shape.right_edge() > 1.0 {
            Edge::Right
        } else {
            Edge::None
        }
    }

    fn move_y(&mut self) {
        self.shape.offset.1 += self.shape.velocity.1;
        if self.shape.offset.1 < -1.0 || self.shape.offset.1 > 1.0 {
            self.shape.velocity.1 *= -1.0;
        }
    }

    pub fn update(&mut self, player: &Shape, enemy: &Shape) {
        match self.move_x() {
            Edge::None => {},
            Edge::Left => {
                println!("Score enemy!");
                self.shape.offset = (0.0, 0.0);
            },
            Edge::Right => {
                println!("Score player!");
                self.shape.offset = (0.0, 0.0);
            },
        }
        self.move_y();
        match in_collision(&self.shape, player) {
            Collision::None => {},
            _ => {
                self.shape.set_center_x(player.right_edge() + self.shape.half_width);
                self.shape.velocity.0 = self.shape.velocity.0.abs();
            }
        }
        match in_collision(&self.shape, enemy) {
            Collision::None => {},
            _ => {
                self.shape.set_center_x(enemy.left_edge() - self.shape.half_width);
                self.shape.velocity.0 = self.shape.velocity.0.abs() * - 1.0;
            }
        }
    }
}
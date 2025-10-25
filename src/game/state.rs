use std::collections::HashSet;

use winit::keyboard::KeyCode;

use crate::game::{
    ball::Ball, enemy::Enemy, player::Player
};

pub trait State {
    fn update(&self, player: &mut Player, enemy: &mut Enemy, ball: &mut Ball, keys_pressed: &HashSet<KeyCode>);
    fn draw(&self, player: &Player, enemy: &Enemy, ball: &Ball);
}

impl Default for Box<dyn State> {
    fn default() -> Self {
        Box::new(GameplayState)
    }
}

struct GameplayState;

impl State for GameplayState {
    fn update(&self, player: &mut Player, enemy: &mut Enemy, ball: &mut Ball, keys_pressed: &HashSet<KeyCode>) {
        player.update(&keys_pressed);
        enemy.update(ball);
        ball.update(&player.shape, &enemy.shape);
    }

    fn draw(&self, player: &Player, enemy: &Enemy, ball: &Ball) {
        player.draw();
        enemy.draw();
        ball.draw();
    }
}
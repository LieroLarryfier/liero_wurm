use crate::game::output::draw;
use crate::snake::{Direction, Element, Snake};
use game::game_loop;
use game::Level;
use std::error::Error;
use std::time::Duration;

pub mod game;
pub mod snake;

pub fn setup() -> Snake {
    let x = 4;
    let y = 5;

    let start = Element::new(x, y);

    let snake: Snake = Snake::new(start, Direction::RIGHT);

    snake
}

pub fn run(snake: &mut Snake, level: &mut Level) -> Result<(), Box<dyn Error>> {
    let duration = Duration::from_millis(500);

    println!("{:?}", snake);

    draw(&snake)?;

    loop {
        game_loop::game_loop(snake, level, duration)?;
    }
}

use crate::game::output::{draw_level, draw_food};
use crate::game::{input::handle_input, output::draw};
use crate::snake::{Direction, Element, Snake};
use game::Level;
use std::error::Error;
use std::thread;
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
    let duration = Duration::from_millis(100);

    println!("{:?}", snake);

    snake.move_forward();

    println!("{:?}", snake);

    draw(&snake)?;

    loop {
        handle_input(snake);

        thread::sleep(duration);
        snake.move_forward()?;

        if snake.food_found(level.food) {
            snake.eat()?;
            level.food = level.spawn_food();
        }

        draw_level(&level)?;
        draw_food(level.food)?;
        draw(&snake)?;

        if snake.check_collision() || snake.check_collision_level(&level) {
            break;
        }
    }

    Ok(())
}

use crate::game::output::draw_level;
use crate::game::{input::handle_input, output::draw};
use crate::snake::{Direction, Element, Snake};
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use game::Level;
use std::error::Error;
use std::process::Output;
use std::time::Duration;
use std::{process, thread};

pub mod game;
pub mod snake;

pub fn setup() -> Snake {

    let x = 4;
    let y = 5;

    let start = Element::new(x, y);

    let snake: Snake = Snake::new(start, Direction::RIGHT);

    snake
}

pub fn run(snake: &mut Snake, level: &Level) -> Result<(), Box<dyn Error>> {
    let duration = Duration::from_millis(100);

    println!("{:?}", snake);

    snake.move_forward();

    println!("{:?}", snake);

    draw(&snake)?;
    
    loop {
        handle_input(snake);

        thread::sleep(duration);
        snake.move_forward()?;
        draw_level(&level)?;
        draw(&snake)?;
    }

    Ok(())
}

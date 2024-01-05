use crate::snake::{Element, Snake, Direction};
use crate::game::{output::draw, input::handle_input};
use std::error::Error;
use std::{thread, process};
use std::time::Duration;
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}
};

pub mod snake;
pub mod game;

pub fn setup () -> Snake {
        
    let x = 4;
    let y = 5;

    let start = Element::new(x, y);
    
    let snake: Snake = Snake::new(start, Direction::RIGHT);

    snake
}

pub fn run (snake: &mut Snake) -> Result<(), Box<dyn Error>> {

    let duration = Duration::from_millis(100);

    println!("{:?}", snake);

    snake.move_forward();

    println!("{:?}", snake);

    draw(&snake)?;

    loop {
        handle_input(snake);

        thread::sleep(duration);
        snake.move_forward()?;
        draw(&snake)?;
    }

    Ok(())    
}
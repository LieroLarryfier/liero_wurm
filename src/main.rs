use liero_wurm::snake::{Element, Snake, Direction};
use liero_wurm::game::{output::draw, input::handle_input};
use std::error::Error;
use std::{thread, process, env};
use std::time::Duration;
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}
};

use liero_wurm;

fn main() {

    //env::set_var("RUST_BACKTRACE", "1");
    
    println!("Hello, world!");

    let mut snake: Snake = liero_wurm::setup();

    if let Err(e) = liero_wurm::run(&mut snake) {
        println!("Application Error: {e}");
        process::exit(1);
    }  
}


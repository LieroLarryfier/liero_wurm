use liero_wurm::game::Level;
use liero_wurm::game::{input::handle_input, output::draw};
use liero_wurm::snake::{Direction, Element, Snake};
use std::error::Error;
use std::time::Duration;
use std::{env, process, thread};

use liero_wurm;

fn main() {
    let level: Level = Level::new(20, 20);
    let mut snake: Snake = liero_wurm::setup();
    
    if let Err(e) = liero_wurm::run(&mut snake, &level) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}

use liero_wurm::game::{input::handle_input, output::draw};
use liero_wurm::snake::{Direction, Element, Snake};
use std::error::Error;
use std::time::Duration;
use std::{env, process, thread};

use liero_wurm;

fn main() {
    let mut snake: Snake = liero_wurm::setup();

    if let Err(e) = liero_wurm::run(&mut snake) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}

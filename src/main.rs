use liero_wurm::game::Level;
use liero_wurm::snake::Snake;
use std::process;

use liero_wurm;

fn main() {
    let mut level: Level = Level::new(25, 25);
    let mut snake: Snake = liero_wurm::setup();

    if let Err(e) = liero_wurm::run(&mut snake, &mut level) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}

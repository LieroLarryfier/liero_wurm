use liero_wurm::game::Level;
use liero_wurm::snake::Snake_old;
use std::process;

use liero_wurm;

fn main() {
    let mut level: Level = Level::new(25, 25);
    let mut snake: Snake_old = liero_wurm::setup_old();

    liero_wurm::run(&mut snake, &mut level);
}

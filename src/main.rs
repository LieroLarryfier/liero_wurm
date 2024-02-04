use liero_wurm::game::Level;
use liero_wurm::snake::Snake_old;

use liero_wurm;

fn main() {
    let mut level: Level = Level::default();
    let mut snake: Snake_old = liero_wurm::setup_old();

    liero_wurm::run(&mut snake, &mut level);
}

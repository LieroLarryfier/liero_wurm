mod game_loop;
pub mod input;
pub mod output;

pub struct Game {
    width: u16,
    height: u16,
    score: u32,
}

impl Game {
    pub fn new(width: u16, height: u16) -> Game {
        Game {
            width,
            height,
            score: 0,
        }
    }
}

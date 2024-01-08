use crate::snake::Element;

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

pub struct Level {
    pub walls: Vec<Element>,
}

impl Level {
    pub fn new(width: u16, height: u16) -> Level {
        let mut walls = vec![];
        for x in 0..width {
            walls.push(Element::new(x, 0));
            walls.push(Element::new(x, height));
        }
        for y in 0..height {
            walls.push(Element::new(0, y));
            walls.push(Element::new(width, y));
        }

        Level { walls }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Level::new(3, 5).walls.len(), 16);
    }
}

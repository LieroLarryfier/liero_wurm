use bevy::ecs::{component::Component, system::{Commands, Resource}};

use crate::snake::Element;

pub mod game_loop;
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

#[derive(Resource)]
pub struct Level {
    pub walls: Vec<Element>,
    pub food: Element,
}

impl Default for Level {
    fn default() -> Self {     
        let width = 20;
        let height = 20;
        let mut walls = vec![];
        for x in 0..=width {
            walls.push(Element::new(x, 0));
            walls.push(Element::new(x, height));
        }
        for y in 0..=height {
            walls.push(Element::new(0, y));
            walls.push(Element::new(width, y));
        }
        let food: Element = Element::new(width / 2, height / 2);

        Self { walls, food }
    }
}

impl Level {
    pub fn spawn_food(&mut self) -> Element {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let new_food = Element::new(rng.gen_range(1..24), rng.gen_range(1..24));

        new_food
    }

    fn spawn_expanding_food() {
        //After some time the food expands and kills the snake, so be quick - different color
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Level::default().walls.len(), 16);
    }
}

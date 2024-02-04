use crate::snake::{Direction, Player1Marker, Snake};
use bevy::ecs::{query::With, system::{Query, Res}};
use bevy::prelude::*;
use crossterm::event::Event;

pub fn handle_input(input: Res<Input<KeyCode>>, mut query: Query<&mut Snake, With<Player1Marker>>) {
    let mut snake = query.single_mut();
    
    if input.just_pressed( KeyCode::Up) {
        if snake.direction != Direction::Down {
            snake.direction = Direction::Up;
        }   
    }
    if input.just_pressed(KeyCode::Down) {
        if snake.direction != Direction::Up {
            snake.direction = Direction::Down;
        }
    }
    if input.just_pressed(KeyCode::Left) {
        if snake.direction != Direction::Right {
            snake.direction = Direction::Left;
        }
    }
    if input.just_pressed(KeyCode::Right) {
        if snake.direction != Direction::Left {
            snake.direction = Direction::Right;
            
        } 
    }
}



pub trait Input_old {
    fn read_input(&self) -> Event;
}

pub struct RealInput;

impl Input_old for RealInput {
    fn read_input(&self) -> Event {
        crossterm::event::read().expect("Failed to read key event")
    }
}

#[cfg(test)]

mod tests {
    use crate::snake::Element;
    use crossterm::event::KeyModifiers;

    use super::*;

    pub struct MockInput {
        pub event: Event,
    }

    impl Input_old for MockInput {
        fn read_input(&self) -> Event {
            self.event.clone()
        }
    }

    #[test]
    fn test_input() {

    }
}

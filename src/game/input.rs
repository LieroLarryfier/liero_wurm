use crate::snake::{Dead, Direction, Player1Marker};
use bevy::ecs::{query::With, system::{Query, Res}};
use bevy::prelude::*;
use crossterm::event::Event;

pub fn handle_input(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Direction, With<Player1Marker>>) {
    for mut direction in &mut query {
        if input.just_pressed( KeyCode::ArrowUp) {
            if *direction != Direction::Down {
                *direction = Direction::Up;
            }   
        }
        if input.just_pressed(KeyCode::ArrowDown) {
            if *direction != Direction::Up {
                *direction = Direction::Down;
            }
        }
        if input.just_pressed(KeyCode::ArrowLeft) {
            if *direction != Direction::Right {
                *direction = Direction::Left;
            }
        }
        if input.just_pressed(KeyCode::ArrowRight) {
            if *direction != Direction::Left {
                *direction = Direction::Right;    
            } 
        }
    }
}

pub fn handle_reset(input: Res<ButtonInput<KeyCode>>, mut dead_query: Query<&mut Dead>) { 
    let mut dead = dead_query.single_mut();
    if input.just_pressed( KeyCode::Escape) {
        dead.0 = false;
    }
}



pub trait InputOld {
    fn read_input(&self) -> Event;
}

pub struct RealInput;

impl InputOld for RealInput {
    fn read_input(&self) -> Event {
        crossterm::event::read().expect("Failed to read key event")
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    pub struct MockInput {
        pub event: Event,
    }

    impl InputOld for MockInput {
        fn read_input(&self) -> Event {
            self.event.clone()
        }
    }

    #[test]
    fn test_input() {

    }
}

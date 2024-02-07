use std::time::Instant;
use std::{error::Error, thread, time::Duration};

use crate::snake::Snake_old;

use super::input;
use super::input::RealInput;
use super::output;
use crate::game::Level;

pub trait TimeTrait {
    fn get_time(&self) -> Instant;
    fn get_elapsed_time(&self, instant: Instant) -> Duration;
} 

pub struct RealTime;

impl TimeTrait for RealTime {
    fn get_time(&self) -> Instant {
        Instant::now()
    }

    fn get_elapsed_time(&self, instant: Instant) -> Duration {
        instant.elapsed()
    }
}

#[cfg(test)]

mod tests {
    use crate::snake::{Direction, Element};

    use super::*;

    struct MockTime;

    impl TimeTrait for MockTime {
        fn get_time(&self) -> Instant {
            Instant::now()
        }
    
        fn get_elapsed_time(&self, instant: Instant) -> Duration {
            Duration::from_millis(100)
        }
    }

    #[test]
    fn test_game_loop_eat() {
        let snake = &mut Snake_old::new(Element::new(1, 1), Direction::Right);
        let level = &mut Level::default();
        let duration = Duration::from_millis(100);
        let time = MockTime{};

        assert_eq!(snake.head.0.x, 3);
        assert_eq!(snake.head.0.y, 1);

        //game_loop(snake, level, duration, &time);

        assert_eq!(snake.head.0.x, 4);
        assert_eq!(snake.head.0.y, 1);

        //level.food = Element::new(5, 1);

        //game_loop(snake, level, duration, &time);

        assert_eq!(snake.body.len(), 4);
        assert_eq!(snake.head.0.x, 5);
    }

    #[test]
    fn test_game_loop_turn() {
        let snake = &mut Snake_old::new(Element::new(1, 1), Direction::Right);
        let level = &mut Level::default();
        let duration = Duration::from_millis(100);
        let time = MockTime{};

        assert_eq!(snake.head.0.x, 3);
        assert_eq!(snake.head.0.y, 1);

        //game_loop(snake, level, duration, &time);

        assert_eq!(snake.head.0.x, 4);
        assert_eq!(snake.head.0.y, 1);

        //game_loop(snake, level, duration, &time);

        assert_eq!(snake.body.len(), 3);
        assert_eq!(snake.head.0.x, 5);
        assert_eq!(snake.head.0.y, 1);

        snake.direction = Direction::Down;

        //game_loop(snake, level, duration, &time);

        assert_eq!(snake.head.0.x, 5);
        assert_eq!(snake.head.0.y, 2);

        snake.direction = Direction::Left;

        //game_loop(snake, level, duration, &time);

        assert_eq!(snake.head.0.x, 4);
        assert_eq!(snake.head.0.y, 2);
    }

}

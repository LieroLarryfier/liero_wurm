use std::time::Instant;
use std::{error::Error, thread, time::Duration};

use crate::snake::Snake;

use super::input;
use super::input::RealInput;
use super::output;
use crate::game::Level;

pub fn game_loop(
    snake: &mut Snake,
    level: &mut Level,
    duration: Duration,
) -> Result<(), Box<dyn Error>> {
    let input = RealInput {};

    let mut last_frame_time = Instant::now();

    input::handle_input(snake, &input);

    let elapsed_time = last_frame_time.elapsed();
    if elapsed_time >= duration {

        snake.move_forward()?;

        if snake.food_found(level.food) {
            snake.eat()?;
            level.food = level.spawn_food();
        }
        last_frame_time = Instant::now();
    }
    output::draw_level(&level)?;
    output::draw_food(level.food)?;
    output::draw(&snake)?;

    if snake.check_collision() || snake.check_collision_level(&level) {
        panic!("ouch2")
    }

    Ok(())
}

#[cfg(test)]

mod tests {
    use crate::snake::{Direction, Element};

    use super::*;

    #[test]
    fn test_game_loop_eat() {
        let snake = &mut Snake::new(Element::new(1, 1), Direction::RIGHT);
        let level = &mut Level::new(20, 20);
        let duration = Duration::from_millis(100);

        assert_eq!(snake.head.x, 3);
        assert_eq!(snake.head.y, 1);

        game_loop(snake, level, duration);

        assert_eq!(snake.head.x, 4);
        assert_eq!(snake.head.y, 1);

        level.food = Element::new(5, 1);

        game_loop(snake, level, duration);

        assert_eq!(snake.body.len(), 4);
        assert_eq!(snake.head.x, 5);
    }

    #[test]
    fn test_game_loop_turn() {
        let snake = &mut Snake::new(Element::new(1, 1), Direction::RIGHT);
        let level = &mut Level::new(20, 20);
        let duration = Duration::from_millis(50);

        assert_eq!(snake.head.x, 3);
        assert_eq!(snake.head.y, 1);

        game_loop(snake, level, duration);

        assert_eq!(snake.head.x, 4);
        assert_eq!(snake.head.y, 1);

        game_loop(snake, level, duration);

        assert_eq!(snake.body.len(), 3);
        assert_eq!(snake.head.x, 5);
        assert_eq!(snake.head.y, 1);

        snake.direction = Direction::DOWN;

        game_loop(snake, level, duration);

        assert_eq!(snake.head.x, 5);
        assert_eq!(snake.head.y, 2);

        snake.direction = Direction::LEFT;

        game_loop(snake, level, duration);

        assert_eq!(snake.head.x, 4);
        assert_eq!(snake.head.y, 2);
    }

    #[test]
    fn test_game_loop_time() {
        //introduce mocks for the time 
        todo!();
    }
}

use crate::snake::{Direction, Snake};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState};

use std::time::Duration;

pub trait Input {
    fn read_input(&self) -> Event;
}

pub struct RealInput;

impl Input for RealInput {
    fn read_input(&self) -> Event {
        crossterm::event::read().expect("Failed to read key event")
    }
}

pub fn handle_input<T: Input>(snake: &mut Snake, input: &T) -> Option<Direction> {
    if crossterm::event::poll(Duration::from_millis(100)).expect("Failed to poll for input") {
        if let crossterm::event::Event::Key(KeyEvent {
            code,
            modifiers,
            state,
            kind,
        }) = input.read_input()
        {
            if kind == KeyEventKind::Press {
                match code {
                    KeyCode::Esc => {
                        std::process::exit(0);
                    }
                    KeyCode::Up => {
                        if snake.direction != Direction::DOWN {
                            snake.direction = Direction::UP;
                            Some(Direction::UP)
                        } else {
                            None
                        }
                    }
                    KeyCode::Down => {
                        if snake.direction != Direction::UP {
                            snake.direction = Direction::DOWN;
                            Some(Direction::DOWN)
                        } else {
                            None
                        }
                    }
                    KeyCode::Left => {
                        if snake.direction != Direction::RIGHT {
                            snake.direction = Direction::LEFT;
                            Some(Direction::LEFT)
                        } else {
                            None
                        }
                    }
                    KeyCode::Right => {
                        if snake.direction != Direction::LEFT {
                            snake.direction = Direction::RIGHT;
                            Some(Direction::RIGHT)
                        } else {
                            None
                        }
                    }
                    _ => {
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
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

    impl Input for MockInput {
        fn read_input(&self) -> Event {
            self.event.clone()
        }
    }

    impl MockInput {
        fn set_custom_keycode(&mut self, custom_keycode: KeyCode) -> () {
            let custom_input = KeyEvent::new(custom_keycode, KeyModifiers::empty());
            self.event = crossterm::event::Event::Key(custom_input);
        }
    }

    #[test]
    fn test_input() {

        let snake = &mut Snake::new(Element::new(1, 1), Direction::RIGHT);
        let mut mock_input = MockInput {event: crossterm::event::Event::Key(KeyEvent::new_with_kind(KeyCode::Right, KeyModifiers::empty(), KeyEventKind::Press))};
        println!("{:?}", snake.head);
        assert_eq!(snake.head.x, 3);
        assert_eq!(snake.head.y, 1);

        mock_input.set_custom_keycode(KeyCode::Down);
        assert_eq!(handle_input(snake, &mock_input), Some(Direction::DOWN));
        assert_eq!(snake.direction, Direction::DOWN);
        mock_input.set_custom_keycode(KeyCode::Left);
        assert_eq!(handle_input(snake, &mock_input), Some(Direction::LEFT));
        assert_eq!(snake.direction, Direction::LEFT);

        mock_input.set_custom_keycode(KeyCode::Down);
        assert_eq!(handle_input(snake, &mock_input), Some(Direction::DOWN));
        assert_eq!(snake.direction, Direction::DOWN);

    }
}

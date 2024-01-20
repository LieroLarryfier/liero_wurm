use crate::snake::{Direction, Snake};
use crossterm::event::{Event, KeyCode, KeyEvent};
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

pub fn handle_input<T: Input>(snake: &mut Snake, input: &T) {
    if crossterm::event::poll(Duration::from_millis(100)).expect("Failed to poll for input") {
        if let crossterm::event::Event::Key(KeyEvent {
            code,
            modifiers,
            state,
            kind,
        }) = input.read_input()
        {
            match code {
                KeyCode::Esc => {
                    std::process::exit(0);
                }
                KeyCode::Up => {
                    if snake.direction != Direction::DOWN {
                        snake.direction = Direction::UP;
                    }
                }
                KeyCode::Down => {
                    if snake.direction != Direction::UP {
                        snake.direction = Direction::DOWN;
                    }
                }
                KeyCode::Left => {
                    if snake.direction != Direction::RIGHT {
                        snake.direction = Direction::LEFT;
                    }
                }
                KeyCode::Right => {
                    if snake.direction != Direction::LEFT {
                        snake.direction = Direction::RIGHT;
                    }
                }
                _ => {}
            }
        }
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
        let mut mock_input = MockInput {event: crossterm::event::Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::empty()))};

        mock_input.set_custom_keycode(KeyCode::Down);
        handle_input(snake, &mock_input);
        assert_eq!(snake.direction, Direction::DOWN);
        
        mock_input.set_custom_keycode(KeyCode::Left);
        handle_input(snake, &mock_input);
        assert_eq!(snake.direction, Direction::LEFT);

        mock_input.set_custom_keycode(KeyCode::Down);
        handle_input(snake, &mock_input);
        assert_eq!(snake.direction, Direction::DOWN);
    }
}

use crate::snake::{Direction, Snake};
use crossterm::{
    cursor,
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, ClearType},
    ExecutableCommand, QueueableCommand,
};
use std::time::Duration;

pub fn handle_input(snake: &mut Snake) {
    if poll(Duration::from_millis(100)).expect("Failed to poll for input") {
        if let crossterm::event::Event::Key(KeyEvent {
            code,
            modifiers,
            state,
            kind,
        }) = read().expect("Failed to read key event")
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

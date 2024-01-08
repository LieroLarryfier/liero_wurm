use crate::snake::Snake;
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::io::{self, Write};

use super::Level;

pub fn draw(snake: &Snake) -> io::Result<()> {
    let mut stdout = io::stdout();

    //stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::Hide)?;

    // Draw snake
    for pos in &snake.body {
        stdout
            .queue(cursor::MoveTo(pos.x, pos.y))?
            .queue(style::PrintStyledContent("■".magenta()))?;
    }

    stdout.flush()?;
    Ok(())
}

pub fn draw_level(level: &Level) -> io::Result<()> {
    let mut stdout = io::stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::Hide)?;

    // Draw level
    for pos in &level.walls {
        stdout
            .queue(cursor::MoveTo(pos.x, pos.y))?
            .queue(style::PrintStyledContent("■".green()))?;
    }

    stdout.flush()?;
    Ok(())
}

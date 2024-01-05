use crate::snake::Snake;
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::io::{self, Write};

pub fn draw(snake: &Snake) -> io::Result<()> {
    let mut stdout = io::stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
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

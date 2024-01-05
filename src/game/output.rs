use std::io::{self, Write};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}
};
use crate::snake::Snake;


pub fn draw(snake: &Snake) -> io::Result<()> {
    let mut stdout = io::stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::Hide)?;

    // Draw snake
    for pos in &snake.body {
        stdout
            .queue(cursor::MoveTo(pos.x,pos.y))?
            .queue(style::PrintStyledContent( "■".magenta()))?;
    }

    stdout.flush()?;
    Ok(())
}
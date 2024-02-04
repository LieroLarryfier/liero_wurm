use crate::snake::{Element, Head, Player1Marker, Snake, Snake_old};
use bevy::{prelude::*, render::camera::ScalingMode};
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::io::{self, Write};

use super::Level;

#[derive(Component)]
pub struct MainCameraMarker;

pub fn setup_camera(mut commands: Commands) {
    let mut main_2d_camera_bundle = Camera2dBundle::default();

    main_2d_camera_bundle.projection.scaling_mode = ScalingMode::AutoMax { max_width: 20.0, max_height: 20.0 };
    main_2d_camera_bundle.transform = Transform::from_xyz(10.0, 10.0, 0.0);

    commands.spawn((
        main_2d_camera_bundle,
        MainCameraMarker,
    ));
}

pub fn draw_snake(mut query: Query<(&Snake, &mut Transform), With<Player1Marker>>) {
    for (snake, mut transform) in &mut query {
        transform.translation.x = snake.head.0.x.into();
        transform.translation.y = snake.head.0.y.into();
        //print!("found snake: {:?}, transform: {:?}", snake, transform);
    }
}

pub fn draw(snake: &Snake_old) -> io::Result<()> {
    let mut stdout = io::stdout();

    //stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    terminal::enable_raw_mode()?;
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

pub fn draw_food(food: Element) -> io::Result<()> {
    let mut stdout = io::stdout();

    //stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    //stdout.execute(cursor::Hide)?;

    // Draw food

    stdout
        .queue(cursor::MoveTo(food.x, food.y))?
        .queue(style::PrintStyledContent("■".yellow()))?;

    stdout.flush()?;
    Ok(())
}

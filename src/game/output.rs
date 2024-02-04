use crate::snake::{CollisionEvent, Element, Head, Player1Marker, Snake, Snake_old};
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

pub fn draw_level(mut commands: Commands, level: Res<Level>) {   
    // Draw level
    for pos in &level.walls {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.5, 0.0),
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
        
        transform: Transform::from_translation(Vec3::new(pos.x.into(), pos.y.into(), 0.0)),
        ..default()
        });
    }
}

pub fn draw_collision(mut events: EventReader<CollisionEvent>) {
    for collision_event in events.read() {
        println!("collision happened: {:?}", collision_event);
    }
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
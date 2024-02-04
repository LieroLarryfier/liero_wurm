use crate::game::game_loop::RealTime;
use crate::game::output::{draw, draw_snake, setup_camera};
use crate::snake::{Direction, Element, SnakePlugin, Snake_old};
use game::game_loop;
use game::Level;
use std::error::Error;
use std::time::Duration;
use bevy::prelude::*;

pub mod game;
pub mod snake;



pub fn setup_old() -> Snake_old {
    let x = 4;
    let y = 5;

    let start = Element::new(x, y);

    let snake: Snake_old = Snake_old::new(start, Direction::RIGHT);

    snake
}

fn hello_world() {
    println!("hello world!");
}

pub fn run_old(snake: &mut Snake_old, level: &mut Level) -> Result<(), Box<dyn Error>> {
    let duration = Duration::from_millis(100);
    let time = RealTime {};

    println!("{:?}", snake);

    draw(&snake)?;

    loop {
        game_loop::game_loop(snake, level, duration, &time)?;
    }
}

pub fn run(snake: &mut Snake_old, level: &mut Level) {
    let duration = Duration::from_millis(100);
    let time = RealTime {};

    println!("{:?}", snake);
    App::new()
        .add_plugins((DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window { 
                        title: "Snake Wurm".into(),
                        resolution: (400.0, 400.0).into(),
                        ..default()
                    }),
                    ..default()
            })
            , SnakePlugin))
        .add_systems(Startup, setup_camera)
        .add_systems(Update, draw_snake)
        .run();
}
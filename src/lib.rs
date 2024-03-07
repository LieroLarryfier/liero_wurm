use crate::game::input::handle_input;
use crate::game::output::{ draw_collision, draw_level, draw_snake, setup_camera};
use crate::snake::{CollisionEvent, SnakePlugin};
use game::{setup_food, spawn_food, FoodEatenEvent};
use game::Level;
use bevy::prelude::*;

pub mod game;
pub mod snake;

const WINDOW_SIZE: f32 = 400.0;
const LEVEL_SIZE: u16 = 200;


pub fn run() {
    App::new()
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins((DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window { 
                        title: "Snake Wurm".into(),
                        resolution: (WINDOW_SIZE, WINDOW_SIZE).into(),
                        decorations: false,
                        resize_constraints: WindowResizeConstraints { min_width: 200.0, min_height: 200.0, max_width: 4000.0, max_height: 4000.0 },
                        transparent: true,
                        ..default()
                    }),
                    ..default()
            })
            , SnakePlugin))
        
        .add_event::<CollisionEvent>()
        .add_event::<FoodEatenEvent>()
        .insert_resource(Level::default())
        .add_systems(Startup, (setup_camera, setup_food, draw_level))
        .add_systems(Update, (draw_snake, spawn_food, handle_input, draw_collision))
        .run();
}
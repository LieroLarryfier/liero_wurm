use crate::game::game_loop::RealTime;
use crate::game::input::handle_input;
use crate::game::output::{ draw_collision, draw_level, draw_snake, setup_camera};
use crate::snake::{CollisionEvent, Direction, Element, SnakePlugin};
use bevy::window::{CompositeAlphaMode, EnabledButtons, WindowMode};
use game::output::draw_food;
use game::{setup_food, spawn_food, FoodEatenEvent};
use game::Level;
use bevy::prelude::*;

pub mod game;
pub mod snake;

const WINDOW_SIZE: f32 = 400.0;
const LEVEL_SIZE: u16 = 20;


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
                        resize_constraints: WindowResizeConstraints { min_width: 20.0, min_height: 20.0, max_width: 400.0, max_height: 400.0 },
                        transparent: true,
                        ..default()
                    }),
                    ..default()
            })
            , SnakePlugin))
        
        .add_event::<CollisionEvent>()
        .add_event::<FoodEatenEvent>()
        .insert_resource(Level::default())
        .add_systems(Startup, (setup_camera, setup_food))
        .add_systems(Update, (draw_level, draw_snake, draw_food, spawn_food, handle_input, draw_collision))
        .run();
}
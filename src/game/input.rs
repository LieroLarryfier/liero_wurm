use crate::snake::{BodyMarker, Dead, Direction, Player1Marker, SnakeBundle};
use bevy::ecs::{query::With, system::{Query, Res}};
use bevy::prelude::*;
use crossterm::event::Event;

use super::Game;

pub fn handle_input(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Direction, With<Player1Marker>>) {
    for mut direction in &mut query {
        if input.just_pressed( KeyCode::ArrowUp) {
            if *direction != Direction::Down {
                *direction = Direction::Up;
            }   
        }
        if input.just_pressed(KeyCode::ArrowDown) {
            if *direction != Direction::Up {
                *direction = Direction::Down;
            }
        }
        if input.just_pressed(KeyCode::ArrowLeft) {
            if *direction != Direction::Right {
                *direction = Direction::Left;
            }
        }
        if input.just_pressed(KeyCode::ArrowRight) {
            if *direction != Direction::Left {
                *direction = Direction::Right;    
            } 
        }
    }
}

pub fn handle_reset(input: Res<ButtonInput<KeyCode>>, mut dead_query: Query<(Entity, &Dead)>, mut game: ResMut<Game>, mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,) { 

    let (entity, dead) = dead_query.single_mut();
    if input.just_pressed( KeyCode::Escape) {
        println!("dead");
        commands.entity(entity).despawn();   

        game.score = 0;

        let layout = TextureAtlasLayout::from_grid(Vec2::new(10.0, 10.0), 5, 1, None, None);
  
        commands.spawn((
            Player1Marker,
            Dead(false),
            SnakeBundle {
                ..Default::default()
            },
            SpriteSheetBundle {
                texture: asset_server.load("sprite.png"),
                atlas: TextureAtlas {
                    layout: texture_atlas_layouts.add(layout.clone()),
                    index: 0
                },
                transform: Transform::from_translation(Vec3::new(-10.0, -10.0, 1.0)),
                ..default()
            }
        ));

        let default_body = SnakeBundle::default().body;

        for pos in &default_body.0 {
            println!("add_body");
            commands.spawn((
                BodyMarker,
                SpriteSheetBundle {
                    texture: asset_server.load("sprite.png"),
                    atlas: TextureAtlas {
                        layout: texture_atlas_layouts.add(layout.clone()),
                        index: 4
                    },      
                transform: Transform::from_translation(Vec3::new(pos.x.into(), pos.y.into(), 0.0)),
                ..default()
                }
            ));
        }   
    }
}



pub trait InputOld {
    fn read_input(&self) -> Event;
}

pub struct RealInput;

impl InputOld for RealInput {
    fn read_input(&self) -> Event {
        crossterm::event::read().expect("Failed to read key event")
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    pub struct MockInput {
        pub event: Event,
    }

    impl InputOld for MockInput {
        fn read_input(&self) -> Event {
            self.event.clone()
        }
    }

    #[test]
    fn test_input() {

    }
}

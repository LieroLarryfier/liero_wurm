use bevy::{ecs::{component::Component, entity::Entity, event::{Event, EventReader}, system::{Commands, Query, Resource}}, math::Vec2, render::color::Color, sprite::{Sprite, SpriteBundle}, transform::commands};
use bevy::prelude::*;

use crate::{snake::Element, LEVEL_SIZE};

pub mod game_loop;
pub mod input;
pub mod output;

pub struct Game {
    width: u16,
    height: u16,
    score: u32,
}

impl Game {
    pub fn new(width: u16, height: u16) -> Game {
        Game {
            width,
            height,
            score: 0,
        }
    }
}

#[derive(Resource)]
pub struct Level {
    pub walls: Vec<Element>,
}

impl Default for Level {
    fn default() -> Self {     
        let width = LEVEL_SIZE;
        let height = LEVEL_SIZE;
        let mut walls = vec![];
        for x in 0..=width {
            walls.push(Element::new(x, 0));
            walls.push(Element::new(x, height));
        }
        for y in 0..=height {
            walls.push(Element::new(0, y));
            walls.push(Element::new(width, y));
        }
        
        Self { walls }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Food {
    pub position: Element,
}

#[derive(Event, Debug)]
pub struct FoodEatenEvent;

pub fn setup_food(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.7),
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new((LEVEL_SIZE / 2).into(), (LEVEL_SIZE / 2).into(), 0.0)),
            ..default()
        },
        Food {
            position: Element::new((LEVEL_SIZE / 2).into(), (LEVEL_SIZE / 2).into()),
        }
    ));
}

pub fn spawn_food(mut query: Query<(Entity, &Food)>, mut event: EventReader<FoodEatenEvent>, mut commands: Commands) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let (entity, food) = query.single_mut();

    let rng_x = rng.gen_range(1..LEVEL_SIZE.into());
    let rng_y = rng.gen_range(1..LEVEL_SIZE.into());
    
    for ev in event.read() {

        commands.entity(entity).despawn();
        
        println!("food eaten event: {:?}", food);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 0.7),
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(rng_x.into(), rng_y.into(), 0.0)),
                ..default()
            },
            Food {
                position: Element::new(rng_x, rng_y),
            }
        ));

        
        //food.position = Element::new(rng.gen_range(1..LEVEL_SIZE.into()), rng.gen_range(1..LEVEL_SIZE.into()));

        println!("new food {:?}", food);
    }
}

impl Level {
   
    pub fn spawn_food(&mut self) -> Element {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let new_food = Element::new(rng.gen_range(1..24), rng.gen_range(1..24));

        new_food
    }

    fn spawn_expanding_food() {
        //After some time the food expands and kills the snake, so be quick - different color
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Level::default().walls.len(), 16);
    }
}

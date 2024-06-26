use bevy::{ecs::{component::Component, entity::Entity, event::{Event, EventReader}, system::{Commands, Query, Resource}}, math::Vec2, render::color::Color, sprite::{Sprite, SpriteBundle}};
use bevy::prelude::*;

use crate::{snake::Element, LEVEL_SIZE, POINT_INCREASE};

pub mod input;
pub mod output;

#[derive(Resource)]
pub struct Game {
    score: u16,
}

impl Default for Game {
    fn default() -> Self {
        Game {
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
        for x in (0..=width).step_by(10) {
            walls.push(Element::new(x, 0));
            walls.push(Element::new(x, height));
        }
        for y in (0..=height).step_by(10) {
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
                custom_size: Some(Vec2::new(10.0, 10.0)),
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

    let max = LEVEL_SIZE/10;

    let rng_x = rng.gen_range(1..max)*10;
    let rng_y = rng.gen_range(1..max)*10;
    
    for ev in event.read() {

        commands.entity(entity).despawn();
        
        println!("food eaten event: {:?}", ev);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 0.7),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(rng_x.into(), rng_y.into(), 0.0)),
                ..default()
            },
            Food {
                position: Element::new(rng_x, rng_y),
            }
        ));

        println!("new food {:?}", food);
    }
}

#[derive(Component)]
pub struct Scoreboard;

pub fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn((
        // Create a TextBundle that has a Text with a
        // single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts 
            // into a `String`, such as `&str`
            "0",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        ) // Set the alignment of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(13.0),
            right: Val::Px(15.0),
            ..default()
        }),
        Scoreboard,
    ));
}

pub fn increase_score(mut game: ResMut<Game>,  mut event: EventReader<FoodEatenEvent>) {
    for ev in event.read() {
        game.score += POINT_INCREASE;
        println!("{:?}", game.score);
    }
}

pub fn update_scoreboard(mut scoreboard: Query<&mut Text, With<Scoreboard>>, game: Res<Game>) {
    if game.is_changed() {
        if let Ok(mut scoreboard) = scoreboard.get_single_mut() {
            scoreboard.sections[0].value = game.score.to_string();
        }
    }
}

impl Level {
    pub fn spawn_food(&mut self) -> Element {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let new_food = Element::new(rng.gen_range(1..24), rng.gen_range(1..24));

        new_food
    }

    fn _spawn_expanding_food() {
        //After some time the food expands and kills the snake, so be quick - different color
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        println!("{:?}", Level::default().walls);
        assert_eq!(Level::default().walls.len(), 84);
    }
}

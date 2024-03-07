use crate::snake::{Body, BodyMarker, CollisionEvent, Element, Head, Player1Marker, SnakeBundle};
use bevy::{prelude::*, render::camera::ScalingMode};

use super::{Food, Level};

#[derive(Component)]
pub struct MainCameraMarker;

pub fn setup_camera(mut commands: Commands) {
    let mut main_2d_camera_bundle = Camera2dBundle::default();

    main_2d_camera_bundle.projection.scaling_mode = ScalingMode::AutoMax { max_width: 200.0, max_height: 200.0 };
    main_2d_camera_bundle.transform = Transform::from_xyz(100.0, 100.0, 0.0);

    commands.spawn((
        main_2d_camera_bundle,
        MainCameraMarker,
    ));
}

pub fn draw_snake(mut query: Query<(&Head, &mut Transform), With<Player1Marker>>, body: Query<&Body>, body_entity: Query<Entity, With<BodyMarker>>, mut commands: Commands) {
    for (head, mut transform) in &mut query {
        transform.translation.x = head.0.x.into();
        transform.translation.y = head.0.y.into();
        //print!("found snake: {:?}, transform: {:?}", snake, transform);
    }

    //Despawn Body
    for entity in body_entity.iter() {
        commands.entity(entity).despawn();
    }
    
    // Draw body
    for body1 in &body {
        for pos in body1.0.iter() {
        commands.spawn((
            BodyMarker,
            SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 1.0, 0.0),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
        
        transform: Transform::from_translation(Vec3::new(pos.x.into(), pos.y.into(), 0.0)),
        ..default()
        }));
    }
}
}

pub fn draw_level(mut commands: Commands, level: Res<Level>) {   
    // Draw level
    for pos in &level.walls {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
        
        transform: Transform::from_translation(Vec3::new(pos.x.into(), pos.y.into(), 0.0)),
        ..default()
        });
    }
}

pub fn draw_element(mut commands: Commands, query: Query<&Element>) {
    for pos in &query {
        println!("draw element: {:?}", pos);
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(10.0, 10.0)),
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

pub fn draw_food(mut commands: Commands, query: Query<&Food>) {
    let food = query.single();
    let pos = &food.position;
}

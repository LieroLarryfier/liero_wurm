use std::{collections::VecDeque, fmt::Error};

use bevy::prelude::*;

use crate::game::{Food, FoodEatenEvent, Level};

#[derive(Debug, Clone, PartialEq, Component)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Debug)]
pub struct Player1Marker;

#[derive(Component, Debug)]
pub struct BodyMarker;

#[derive(Debug, Bundle)]
pub struct SnakeBundle {
    pub head: Head,
    pub body: Body,
    pub direction: Direction,
}

impl Default for SnakeBundle {
    fn default() -> Self {

        let start_x: u16 = 30;
        let start_y: u16 = 30;
        let mut start_body = Body(VecDeque::new(), 0);
        start_body.0.push_back(Element::new(start_x, start_y));
        start_body.0.push_back(Element::new(start_x-1, start_y));
        start_body.0.push_back(Element::new(start_x-2, start_y));

        Self {
            head: Head(Element::new(start_x, start_y)),
            body: Body(start_body.0, 0),
            direction: Direction::Right,
        }
    }
}

#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub struct Head (pub Element);

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Body (pub VecDeque<Element>, u16);

#[derive(Debug, Copy, Clone, PartialEq, Component)]
pub struct Element {
    pub x: u16,
    pub y: u16,
}

impl Element {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(SnakeTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .add_systems(Startup, (add_snake).chain())
        .add_systems(Update, (check_collision_level, move_snake, food_found).chain() );
    }
}

pub fn add_snake(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,) {
    println!("add_snake");

    let layout = TextureAtlasLayout::from_grid(Vec2::new(10.0, 10.0), 5, 1, None, None);
  
    commands.spawn((
        Player1Marker,
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

#[derive(Resource)]
struct SnakeTimer(Timer);

fn move_snake(time: Res<Time>, mut timer: ResMut<SnakeTimer>, mut query: Query<(&mut Head, &mut Body, &Direction), With<Player1Marker>>, mut _event: EventReader<FoodEatenEvent>) {
    //move this into body, dont reset every time
     
    if timer.0.tick(time.delta()).just_finished() {
        for (mut head, mut body, direction) in &mut query {
            match direction {
                Direction::Up => head.0.y += 10,
                Direction::Down => head.0.y -= 10,
                Direction::Left => head.0.x -= 10,
                Direction::Right => head.0.x += 10,
            };
            body.0.push_front(head.0);
            if body.1 == 0 {
                body.0.pop_back();   
            } else {
                body.1 -= 1;
            }
        }
    println!("{:?}", query);
    }
}

#[derive(Event, Debug)]
pub struct CollisionEvent(CollisionType);

#[derive(Debug)]
enum CollisionType {
    Level,
    Snake
}

fn check_collision_level(level: Res<Level>, mut query: Query<&mut Head, With<Player1Marker>>, mut collision_event: EventWriter<CollisionEvent>) {
    
    let mut walls = level.walls.iter();
    for head in &mut query {
        if walls.any(|&pos| pos == head.0) {
            collision_event.send(CollisionEvent(CollisionType::Level));
            
        } 
    }
}

pub fn food_found(mut snake_query: Query<(&Head, &mut Body), With<Player1Marker>>, food_query: Query<&Food>, mut food_found_event: EventWriter<FoodEatenEvent>) {
    
    let food= food_query.single();

    for (head, mut body) in &mut snake_query {

    if head.0 == food.position {
        println!("snake {:?}, food_found: {:?}", head.0, food.position);
        food_found_event.send(FoodEatenEvent);
        body.1 = 1;
    }     
}
}


    pub fn new(start: Element, direction: Direction) -> SnakeBundle {
        let mut instance = SnakeBundle {
            head: Head(Element::new(start.x, start.y)),
            body: Body(VecDeque::new(), 0),
            direction,
        };

        let length: u16 = 3;

        instance.body.0.push_front(start);
        for _ in 1..length {
            //instance.eat();
        }

        instance
    }

    pub fn eat(mut snake: SnakeBundle) -> Result<(), Error> {
        let _old_head = snake.head.clone();

        let new_head = match snake.direction {
            Direction::Up => Head(Element::new(snake.head.0.x, snake.head.0.y - 10)),
            Direction::Down => Head(Element::new(snake.head.0.x, snake.head.0.y + 10)),
            Direction::Left => Head(Element::new(snake.head.0.x.checked_sub(10).expect("ouch"), snake.head.0.y)),
            Direction::Right => Head(Element::new(snake.head.0.x + 10, snake.head.0.y)),
        };

        snake.head = new_head;
        snake.body.0.push_front(new_head.0);

        Ok(())
    }

    pub fn move_forward(mut snake: SnakeBundle) -> Result<(), Error> {
        //snake.eat()?;
        snake.body.0.pop_back();

        Ok(())
    }

    //TODO: multigrab with big mouth
    fn multigrab() {
        todo!();
    }

    //TODO: shoot to make a hole
    fn shoot() {
        todo!();
    }

    //TODO: split your head
    fn split_personalities() {
        todo!();
    }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        //assert_eq!(Snake_old::new(Element::new(3, 3), Direction::Up).body.len(), 3);
    }

    #[test]
    fn test_eat() {
        //let mut snake = Snake_old::new(Element::new(3, 3), Direction::Up);
        //assert_eq!(snake.body.len(), 3);
        //snake.eat();
        //assert_eq!(snake.body.len(), 4);
    }

    #[test]
    fn test_move() {
        //let mut snake = Snake_old::new(Element::new(3, 3), Direction::Up);
        //assert_eq!(snake.body.len(), 3);
        //snake.move_forward();
        //assert_eq!(snake.body.len(), 3);
    }

    #[test]
    fn test_direction_up() {
        //let snake = Snake_old::new(Element::new(10, 10), Direction::Up);

        //assert_eq!(snake.body.front().unwrap().x, 10);
        //assert_eq!(snake.body.front().unwrap().y, 8);
    }

    #[test]
    fn test_direction_down() {
        //let snake = Snake_old::new(Element::new(10, 10), Direction::Down);

        //assert_eq!(snake.body.front().unwrap().x, 10);
        //assert_eq!(snake.body.front().unwrap().y, 12);
    }

    #[test]
    fn test_direction_left() {
        //let snake = Snake_old::new(Element::new(10, 10), Direction::Left);

        //assert_eq!(snake.body.front().unwrap().x, 8);
        //assert_eq!(snake.body.front().unwrap().y, 10);
    }

    #[test]
    fn test_direction_right() {
        //let snake = Snake_old::new(Element::new(10, 10), Direction::Right);

        //assert_eq!(snake.body.front().unwrap().x, 12);
        //assert_eq!(snake.body.front().unwrap().y, 10);
    }

    #[test]
    //Test a sharp turn.
    fn test_turn() {
        //let mut snake = Snake_old::new(Element::new(0, 0), Direction::Right);

        //assert_eq!(snake.head.0.x, 2);
        //assert_eq!(snake.head.0.y, 0);
        //snake.direction = Direction::Down;
        //snake.move_forward();
        //snake.direction = Direction::Left;
        //snake.move_forward();
        //assert_eq!(snake.head.0.x, 1);
        //assert_eq!(snake.head.0.y, 1);
    }
}

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

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(SnakeTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .add_systems(Startup, add_snake)
        .add_systems(Update, (check_collision_level, move_snake, food_found).chain() );
    }
}

pub fn add_snake(mut commands: Commands) {
    commands.spawn((
        Player1Marker,
        Snake {
            ..Default::default()
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.0, 0.5),
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-10.0, -10.0, 0.0)),
            ..default()
        }
    ));   
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            head: Head(Element::new(3, 3)),
            body: Body(VecDeque::new()),
            direction: Direction::Right,
        }
    }
}

#[derive(Component, Debug)]
pub struct Player1Marker;

#[derive(Debug, Component)]
pub struct Snake {
    pub head: Head,
    body: Body,
    pub direction: Direction,
}

#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub struct Head (pub Element);

#[derive(Component, Debug, Clone, PartialEq)]
struct Body (pub VecDeque<Element>);

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

#[derive(Resource)]
struct SnakeTimer(Timer);

fn move_snake(time: Res<Time>, mut timer: ResMut<SnakeTimer>, mut query: Query<&mut Snake, With<Player1Marker>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut snake in &mut query {

            match snake.direction {
                Direction::Up => snake.head.0.y += 1,
                Direction::Down => snake.head.0.y -= 1,
                Direction::Left => snake.head.0.x -= 1,
                Direction::Right => snake.head.0.x += 1,
            };

            println!("head: {:?}", snake);
        }
    }
}

#[derive(Event, Debug)]
pub struct CollisionEvent(CollisionType);

#[derive(Debug)]
enum CollisionType {
    Level,
    Snake
}

fn check_collision_level(level: Res<Level>, query: Query<&Snake, With<Player1Marker>>, mut collision_event: EventWriter<CollisionEvent>) {
    let snake = query.single();
    let head = snake.head;
    let mut iter = level.walls.iter();
    
    if iter.any(|&pos| pos == head.0) {
        collision_event.send(CollisionEvent(CollisionType::Level));
    }
}

pub fn food_found(snake_query: Query<&Snake, With<Player1Marker>>, food_query: Query<&Food>, mut food_found_event: EventWriter<FoodEatenEvent>) {
    let snake = snake_query.single();
    let food= food_query.single();

    if snake.head.0 == food.position {
        println!("snake {:?}, food_found: {:?}", snake.head.0, food.position);
        food_found_event.send(FoodEatenEvent);
    }     
}

#[derive(Debug)]
pub struct Snake_old {
    pub head: Head,
    pub body: VecDeque<Element>,
    pub direction: Direction,
}

impl Snake_old {
    pub fn new(start: Element, direction: Direction) -> Snake_old {
        let mut instance = Snake_old {
            head: Head(Element::new(start.x, start.y)),
            body: VecDeque::new(),
            direction,
        };

        let length: u16 = 3;

        instance.body.push_front(start);
        for _ in 1..length {
            instance.eat();
        }

        instance
    }

    pub fn eat(&mut self) -> Result<(), Error> {
        let _old_head = self.head.clone();

        let new_head = match self.direction {
            Direction::Up => Head(Element::new(self.head.0.x, self.head.0.y - 1)),
            Direction::Down => Head(Element::new(self.head.0.x, self.head.0.y + 1)),
            Direction::Left => Head(Element::new(self.head.0.x.checked_sub(1).expect("ouch"), self.head.0.y)),
            Direction::Right => Head(Element::new(self.head.0.x + 1, self.head.0.y)),
        };

        self.head = new_head;
        self.body.push_front(new_head.0);

        Ok(())
    }

    pub fn move_forward(&mut self) -> Result<(), Error> {
        self.eat()?;
        self.body.pop_back();

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

    pub fn check_collision(&self) -> bool {
        let head = self.body.front().expect("Snake_old has no body!");
        let mut iter = self.body.iter();
        iter.next(); // Skip head

        iter.any(|&pos| pos == *head)
    }

    pub fn check_collision_level(&self, level: &Level) -> bool {
        let head = self.head;
        let mut iter = level.walls.iter();
        iter.next(); // Skip head

        iter.any(|&pos| pos == head.0)
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Snake_old::new(Element::new(3, 3), Direction::Up).body.len(), 3);
    }

    #[test]
    fn test_eat() {
        let mut snake = Snake_old::new(Element::new(3, 3), Direction::Up);
        assert_eq!(snake.body.len(), 3);
        snake.eat();
        assert_eq!(snake.body.len(), 4);
    }

    #[test]
    fn test_move() {
        let mut snake = Snake_old::new(Element::new(3, 3), Direction::Up);
        assert_eq!(snake.body.len(), 3);
        snake.move_forward();
        assert_eq!(snake.body.len(), 3);
    }

    #[test]
    fn test_direction_up() {
        let snake = Snake_old::new(Element::new(10, 10), Direction::Up);

        assert_eq!(snake.body.front().unwrap().x, 10);
        assert_eq!(snake.body.front().unwrap().y, 8);
    }

    #[test]
    fn test_direction_down() {
        let snake = Snake_old::new(Element::new(10, 10), Direction::Down);

        assert_eq!(snake.body.front().unwrap().x, 10);
        assert_eq!(snake.body.front().unwrap().y, 12);
    }

    #[test]
    fn test_direction_left() {
        let snake = Snake_old::new(Element::new(10, 10), Direction::Left);

        assert_eq!(snake.body.front().unwrap().x, 8);
        assert_eq!(snake.body.front().unwrap().y, 10);
    }

    #[test]
    fn test_direction_right() {
        let snake = Snake_old::new(Element::new(10, 10), Direction::Right);

        assert_eq!(snake.body.front().unwrap().x, 12);
        assert_eq!(snake.body.front().unwrap().y, 10);
    }

    #[test]
    //Test a sharp turn.
    fn test_turn() {
        let mut snake = Snake_old::new(Element::new(0, 0), Direction::Right);

        assert_eq!(snake.head.0.x, 2);
        assert_eq!(snake.head.0.y, 0);
        snake.direction = Direction::Down;
        snake.move_forward();
        snake.direction = Direction::Left;
        snake.move_forward();
        assert_eq!(snake.head.0.x, 1);
        assert_eq!(snake.head.0.y, 1);
    }
}

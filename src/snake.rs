use std::{collections::VecDeque, fmt::Error};

use bevy::prelude::*;

use crate::game::Level;

#[derive(Debug, Clone, PartialEq, Component)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
pub struct Snake_old {
    pub head: Head,
    pub body: VecDeque<Element>,
    pub direction: Direction,
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(SnakeTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_systems(Startup, add_snake)
        .add_systems(Update, move_snake );
    }
}

pub fn add_snake(mut commands: Commands) {
    commands.spawn(Snake {
        ..Default::default()
    });   
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            head: Head(Element::new(3, 3)),
            body: Body(VecDeque::new()),
            direction: Direction::RIGHT,
        }
    }
}

#[derive(Debug, Bundle)]
pub struct Snake {
    head: Head,
    body: Body,
    direction: Direction,
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

fn move_snake(time: Res<Time>, mut timer: ResMut<SnakeTimer>, mut query: Query<&mut Head>) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut head in &mut query {
            head.0.x += 1;
            println!("head: {:?}", head);
        }
    }
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
            Direction::UP => Head(Element::new(self.head.0.x, self.head.0.y - 1)),
            Direction::DOWN => Head(Element::new(self.head.0.x, self.head.0.y + 1)),
            Direction::LEFT => Head(Element::new(self.head.0.x.checked_sub(1).expect("ouch"), self.head.0.y)),
            Direction::RIGHT => Head(Element::new(self.head.0.x + 1, self.head.0.y)),
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

    pub fn food_found(&self, food: Element) -> bool {
        if self.head.0 == food {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Snake_old::new(Element::new(3, 3), Direction::UP).body.len(), 3);
    }

    #[test]
    fn test_eat() {
        let mut snake = Snake_old::new(Element::new(3, 3), Direction::UP);
        assert_eq!(snake.body.len(), 3);
        snake.eat();
        assert_eq!(snake.body.len(), 4);
    }

    #[test]
    fn test_move() {
        let mut snake = Snake_old::new(Element::new(3, 3), Direction::UP);
        assert_eq!(snake.body.len(), 3);
        snake.move_forward();
        assert_eq!(snake.body.len(), 3);
    }

    #[test]
    fn test_direction_up() {
        let snake = Snake_old::new(Element::new(10, 10), Direction::UP);

        assert_eq!(snake.body.front().unwrap().x, 10);
        assert_eq!(snake.body.front().unwrap().y, 8);
    }

    #[test]
    fn test_direction_down() {
        let snake = Snake_old::new(Element::new(10, 10), Direction::DOWN);

        assert_eq!(snake.body.front().unwrap().x, 10);
        assert_eq!(snake.body.front().unwrap().y, 12);
    }

    #[test]
    fn test_direction_left() {
        let snake = Snake_old::new(Element::new(10, 10), Direction::LEFT);

        assert_eq!(snake.body.front().unwrap().x, 8);
        assert_eq!(snake.body.front().unwrap().y, 10);
    }

    #[test]
    fn test_direction_right() {
        let snake = Snake_old::new(Element::new(10, 10), Direction::RIGHT);

        assert_eq!(snake.body.front().unwrap().x, 12);
        assert_eq!(snake.body.front().unwrap().y, 10);
    }

    #[test]
    //Test a sharp turn.
    fn test_turn() {
        let mut snake = Snake_old::new(Element::new(0, 0), Direction::RIGHT);

        assert_eq!(snake.head.0.x, 2);
        assert_eq!(snake.head.0.y, 0);
        snake.direction = Direction::DOWN;
        snake.move_forward();
        snake.direction = Direction::LEFT;
        snake.move_forward();
        assert_eq!(snake.head.0.x, 1);
        assert_eq!(snake.head.0.y, 1);
    }
}

use std::{collections::VecDeque, fmt::Error};

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
pub struct Snake {
    head: Element,
    pub body: VecDeque<Element>,
    pub direction: Direction,
}

#[derive(Debug, Copy, Clone)]
pub struct Element {
    pub x: u16,
    pub y: u16,
}

impl Element {
    pub fn new(x: u16, y: u16) -> Self {
        Self{ x, y }
    }
}

impl Snake {
    
    pub fn new(start: Element, direction: Direction) -> Snake {  
        
        let mut instance = Snake {
            head: Element::new(start.x, start.y),
            body: VecDeque::new(),
            direction,
        };

        instance.body.push_front(start);
        instance.eat();
        instance.eat();

        instance
    }

    fn eat(&mut self) -> Result<(), &'static str> {
        let _old_head = self.head.clone();
        
        let new_head = match self.direction {
            Direction::UP => {
                Element::new(self.head.x, self.head.y-1)
            },
            Direction::DOWN => {
                Element::new(self.head.x, self.head.y+1)
            },
            Direction::LEFT => {
                Element::new(self.head.x-1, self.head.y)
            },
            Direction::RIGHT => {
                Element::new(self.head.x+1, self.head.y)
            },
        };

        self.head = new_head;
        self.body.push_front(new_head);

        Ok(())
    }

    pub fn move_forward(&mut self) -> Result<(), Error> {
        self.eat();
        self.body.pop_back();

        Ok(())
    }

    //TODO: multigrab with big mouth
    fn multigrab() {
        todo!();
    }

    //TODO: shoot to make a hole
    fn shoot() {}

    //TODO: split your head
    fn split_personalities() {}    
}

    #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Snake::new(Element::new(3,3), Direction::UP).body.len(), 3);
    }

    #[test]
    fn test_eat() {
        let mut snake = Snake::new(Element::new(3,3), Direction::UP);
        assert_eq!(snake.body.len(), 3);
        snake.eat();
        assert_eq!(snake.body.len(), 4);
    }

    #[test]
    fn test_move() {
        let mut snake = Snake::new(Element::new(3,3), Direction::UP);
        assert_eq!(snake.body.len(), 3);
        snake.move_forward();
        assert_eq!(snake.body.len(), 3);
    }

    #[test]
    fn test_direction_up() {

        let snake = Snake::new(Element::new(10,10) ,Direction::UP);

        assert_eq!(snake.body.front().unwrap().x, 10);
        assert_eq!(snake.body.front().unwrap().y, 8);  
    }

    #[test]
    fn test_direction_down() {

        let snake = Snake::new(Element::new(10,10) ,Direction::DOWN);

        assert_eq!(snake.body.front().unwrap().x, 10);
        assert_eq!(snake.body.front().unwrap().y, 12);  
    }

    #[test]
    fn test_direction_left() {

        let snake = Snake::new(Element::new(10,10) ,Direction::LEFT);

        assert_eq!(snake.body.front().unwrap().x, 8);
        assert_eq!(snake.body.front().unwrap().y, 10);  
    }

    #[test]
    fn test_direction_right() {

        let snake = Snake::new(Element::new(10,10) ,Direction::RIGHT);

        assert_eq!(snake.body.front().unwrap().x, 12);
        assert_eq!(snake.body.front().unwrap().y, 10);  
    }
}


use liero_wurm::{self, snake::Direction};

#[test]
fn test_setup() {
    assert_eq!(Direction::RIGHT, liero_wurm::setup().direction);
}

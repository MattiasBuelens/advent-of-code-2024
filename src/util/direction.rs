use super::Vector2D;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    N,
    S,
    W,
    E,
}

#[allow(unused)]
impl Direction {
    pub fn step(self) -> Vector2D {
        match self {
            Direction::N => Vector2D::new(0, -1),
            Direction::S => Vector2D::new(0, 1),
            Direction::W => Vector2D::new(-1, 0),
            Direction::E => Vector2D::new(1, 0),
        }
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
            Direction::E => Direction::W,
        }
    }

    pub fn rotate_left(self) -> Direction {
        match self {
            Direction::N => Direction::W,
            Direction::W => Direction::S,
            Direction::S => Direction::E,
            Direction::E => Direction::N,
        }
    }

    pub fn rotate_right(self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::W => Direction::N,
            Direction::S => Direction::W,
        }
    }

    pub fn all() -> [Direction; 4] {
        [Direction::N, Direction::S, Direction::W, Direction::E]
    }
}

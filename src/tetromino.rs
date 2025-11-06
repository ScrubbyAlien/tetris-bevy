use std::path::Path;
use bevy::prelude::*;

pub struct Tetromino {
    image: Handle<Image>,
    up: u16,
    right: u16,
    down: u16,
    left: u16,
    current_direction: Direction,
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Tetromino {
    pub fn get_sprite(&self) -> Sprite {
        Sprite::from_image(self.image.clone())
    }

    pub fn rotate_clockwise(&mut self) {
        match self.current_direction {
            Direction::Up => self.current_direction = Direction::Right,
            Direction::Right => self.current_direction = Direction::Down,
            Direction::Down => self.current_direction = Direction::Left,
            Direction::Left => self.current_direction = Direction::Up,
        }
    }

    pub fn rotate_counter_clockwise(&mut self) {
        match self.current_direction {
            Direction::Up => self.current_direction = Direction::Left,
            Direction::Left => self.current_direction = Direction::Down,
            Direction::Down => self.current_direction = Direction::Right,
            Direction::Right => self.current_direction = Direction::Up,
        }
    }

    pub fn get_rotation(&self) -> u16 {
        match self.current_direction {
            Direction::Up => self.up,
            Direction::Right => self.right,
            Direction::Down => self.down,
            Direction::Left => self.left,
        }
    }
}

// 0 1 0 0
// 0 1 0 0
// 0 1 1 0
// 0 0 0 0
pub fn get_l_right(image: Handle<Image>) -> Tetromino {
    Tetromino {
        image,
        up: 0b0100_0100_0110_0000,
        right: 0b0000_1110_0100_0000,
        down: 0b1100_0100_0100_0000,
        left: 0b0010_1110_0000_0000,
        current_direction: Direction::Up,
    }
}
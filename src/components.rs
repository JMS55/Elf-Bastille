use crate::{WORLD_HEIGHT, WORLD_WIDTH};
use specs::storage::{BTreeStorage, DenseVecStorage, VecStorage};
use specs::Component;
use specs_derive::Component;
use std::cmp::Ord;

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn get_adjacent(&self) -> Vec<Self> {
        let mut adjacent = Vec::with_capacity(4);
        // Above
        if self.y != 0 {
            adjacent.push(Self {
                x: self.x,
                y: self.y - 1,
            });
        }
        // Below
        if self.y != WORLD_HEIGHT {
            adjacent.push(Self {
                x: self.x,
                y: self.y + 1,
            });
        }
        // Left
        if self.x != 0 {
            adjacent.push(Self {
                x: self.x - 1,
                y: self.y,
            });
        }
        // Right
        if self.x != WORLD_WIDTH {
            adjacent.push(Self {
                x: self.x + 1,
                y: self.y,
            });
        }
        adjacent
    }

    pub fn get_distance_from(&self, other: &Self) -> u32 {
        (self.x.max(other.x) - self.x.min(other.x)) + (self.y.max(other.y) - self.y.min(other.y))
    }
}

#[derive(Component, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
#[storage(DenseVecStorage)]
pub struct Movement {
    pub target: Option<Position>,
    pub path: Vec<Position>,
    pub move_speed: u32,
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
#[storage(VecStorage)]
pub struct Sprite {
    pub name: &'static str,
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
#[storage(BTreeStorage)]
pub struct Elf;

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
#[storage(BTreeStorage)]
pub struct Tree {
    pub durability: u32,
}

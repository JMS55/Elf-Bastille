use crate::{WORLD_HEIGHT, WORLD_WIDTH};
use specs::storage::{BTreeStorage, DenseVecStorage, NullStorage, VecStorage};
use specs::{Component, Entity, ReadStorage};
use specs_derive::Component;
use std::cmp::Ord;

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn get_adjacent(self) -> Vec<Self> {
        let mut adjacent = Vec::with_capacity(4);
        // Above
        if self.y != 0 {
            adjacent.push(Self {
                x: self.x,
                y: self.y - 1,
            });
        }
        // Below
        if self.y != WORLD_HEIGHT - 1 {
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
        if self.x != WORLD_WIDTH - 1 {
            adjacent.push(Self {
                x: self.x + 1,
                y: self.y,
            });
        }
        adjacent
    }

    pub fn get_distance_from(self, other: Self) -> u32 {
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

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd, Default)]
#[storage(NullStorage)]
pub struct Elf;

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
#[storage(BTreeStorage)]
pub struct Tree {
    pub durability: u32,
}

#[derive(Component, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
#[storage(BTreeStorage)]
pub struct ItemStorage {
    pub items: Vec<Entity>,
    pub volume_limit: u32,
    pub weight_limit: Option<u32>,
}

impl ItemStorage {
    // Try inserting an item, returning the item back if it fails
    pub fn try_insert(&self, item: Entity, item_data: &ReadStorage<Item>) -> Result<(), Entity> {
        let mut suceeded = true;

        // Volume check
        let stored_volume: u32 = self
            .items
            .iter()
            .map(|entity| {
                item_data
                    .get(*entity)
                    .expect("Entity did not have an Item component")
                    .volume
            })
            .sum();
        let item_volume = item_data
            .get(item)
            .expect("Entity did not have an Item component")
            .volume;
        if stored_volume + item_volume > self.volume_limit {
            suceeded = false;
        }

        // Weight check
        if let Some(weight_limit) = self.weight_limit {
            let stored_weight: u32 = self
                .items
                .iter()
                .map(|entity| {
                    item_data
                        .get(*entity)
                        .expect("Entity did not have an Item component")
                        .weight
                })
                .sum();
            let item_weight = item_data
                .get(item)
                .expect("Entity did not have an Item component")
                .weight;
            if stored_weight + item_weight > weight_limit {
                suceeded = false;
            }
        }

        match suceeded {
            true => Ok(()),
            false => Err(item),
        }
    }
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
#[storage(BTreeStorage)]
pub struct Item {
    pub volume: u32,
    pub weight: u32,
}

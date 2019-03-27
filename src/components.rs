use crate::{WORLD_HEIGHT, WORLD_WIDTH};
use specs::storage::{BTreeStorage, DenseVecStorage, NullStorage, VecStorage};
use specs::{Component, Entity};
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
pub struct Displayable {
    pub entity_name: &'static str,
    pub texture_atlas_index: u32,
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
    pub stored_volume: u32,
    pub stored_weight: u32,
    pub volume_limit: u32,
    pub weight_limit: Option<u32>,
}

impl ItemStorage {
    pub fn new(volume_limit: u32, weight_limit: Option<u32>) -> Self {
        Self {
            items: Vec::new(),
            stored_volume: 0,
            stored_weight: 0,
            volume_limit,
            weight_limit,
        }
    }

    // Try inserting an item, returning the item back if it fails
    pub fn try_insert(
        &mut self,
        item_entity: Entity,
        item: &Item,
        self_item: Option<&mut Item>, // Item belonging to the same entity as this component
    ) -> Result<(), Entity> {
        let mut succeeded = true;

        if self.stored_volume + item.volume > self.volume_limit {
            succeeded = false;
        }

        if let Some(weight_limit) = self.weight_limit {
            if self.stored_weight + item.weight > weight_limit {
                succeeded = false;
            }
        }

        match succeeded {
            true => {
                self.stored_volume += item.volume;
                self.stored_weight += item.weight;
                if let Some(self_item) = self_item {
                    self_item.weight += item.weight;
                }
                self.items.push(item_entity);
                Ok(())
            }
            false => Err(item_entity),
        }
    }

    // TODO: Wait for Vec::remove_item() to be stabilized
    // pub fn remove(&mut self, item_entity: &Entity, item: &Item, self_item: Option<&mut Item>) {
    //     self.items.remove_item(item_entity);
    //     self.stored_volume -= item.volume;
    //     self.stored_weight -= item.weight;
    //     if let Some(self_item) = self_item {
    //         self_item.weight -= item.weight;
    //     }
    // }
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
#[storage(BTreeStorage)]
pub struct Item {
    pub volume: u32,
    pub weight: u32,
}

use crate::systems::craft::CraftableEntityType;
use fixed::types::I32F32;
use specs::storage::{BTreeStorage, NullStorage};
use specs::{Component, Entity, LazyUpdate};
use specs_derive::Component;
use std::time::Duration;

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Container {
    pub children: Vec<Entity>,
    pub stored_volume: u32,
    pub stored_weight: u32,
    pub volume_limit: u32,
    pub weight_limit: Option<u32>,
}

impl Container {
    pub fn new(volume_limit: u32, weight_limit: Option<u32>) -> Self {
        Self {
            children: Vec::new(),
            stored_volume: 0,
            stored_weight: 0,
            volume_limit,
            weight_limit,
        }
    }
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct ContainerChild {
    pub parent_container: Entity,
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Damageable {
    pub durability: u32,
    pub on_break_callback: Option<fn(Entity, &LazyUpdate)>,
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct PhysicalProperties {
    pub volume: u32,
    pub weight: u32,
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Displayable {
    pub texture_atlas_index: u32,
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct AI {
    pub set_action_callback: fn(Entity, &LazyUpdate),
}

#[derive(Component, PartialEq)]
#[storage(BTreeStorage)]
pub enum EntityType {
    Axe,
    Crate,
    Cup,
    Dirt,
    Elf,
    Tree,
    Wall,
    Wood,
}

#[derive(Component, Clone, PartialEq)]
#[storage(BTreeStorage)]
pub struct Position {
    pub x: I32F32,
    pub y: I32F32,
    pub z: I32F32,
}

impl Position {
    pub fn is_adjacent_to(&self, other: &Self) -> bool {
        other
            == &Self {
                x: self.x + I32F32::from(1),
                y: self.y,
                z: self.z,
            }
            || other
                == &Self {
                    x: self.x - I32F32::from(1),
                    y: self.y,
                    z: self.z,
                }
            || other
                == &Self {
                    x: self.x,
                    y: self.y + I32F32::from(1),
                    z: self.z,
                }
            || other
                == &Self {
                    x: self.x,
                    y: self.y - I32F32::from(1),
                    z: self.z,
                }
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Walkable;

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct MoveSpeed {
    pub speed: I32F32,
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct TimeTracker {
    pub time_passed: Duration,
    pub callback: fn(Duration, Entity, &LazyUpdate),
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct ActionInsertIntoContainer {
    pub entity: Entity,
    pub target_container: Entity,
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct ActionCraft {
    pub type_to_craft: CraftableEntityType,
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct ActionAttack {
    pub weapon: Entity,
    pub target_entity: Entity,
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct ActionMoveTo {
    pub target: Position,
    pub path: Vec<Position>,
}

impl ActionMoveTo {
    pub fn new(target: Position) -> Self {
        Self {
            target,
            path: Vec::new(),
        }
    }
}

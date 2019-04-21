use fixed::types::I32F32;
use specs::storage::{BTreeStorage, NullStorage};
use specs::{Component, Entity, LazyUpdate};
use specs_derive::Component;
use std::time::Duration;

#[derive(Component, Clone, Debug)]
#[storage(BTreeStorage)]
pub struct Container {
    // Does not support nesting
    pub entities: Vec<Entity>,
    pub stored_volume: u32,
    pub stored_weight: u32,
    pub volume_limit: u32,
    pub weight_limit: Option<u32>,
}

impl Container {
    pub fn new(volume_limit: u32, weight_limit: Option<u32>) -> Self {
        Self {
            entities: Vec::new(),
            stored_volume: 0,
            stored_weight: 0,
            volume_limit,
            weight_limit,
        }
    }
}

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct ContainerChild {
    pub parent: Entity,
}

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct Damageable {
    pub durability: u32,
}

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct Growable {
    pub age: Duration,
}

impl Growable {
    pub fn new() -> Self {
        Self {
            age: Duration::from_nanos(0),
        }
    }
}

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct PhysicalProperties {
    pub volume: u32,
    pub weight: u32,
}

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct Displayable {
    pub texture_atlas_index: u32,
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct AI {
    pub set_action: fn(Entity, &LazyUpdate),
}

#[derive(Component, PartialEq, Debug)]
#[storage(BTreeStorage)]
pub enum EntityType {
    Tree,
    Elf,
    Wood,
    Axe,
    Crate,
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Loot {
    pub create_loot: fn(&LazyUpdate),
}

#[derive(Component, Copy, Clone, Hash, PartialEq, Eq, Debug)]
#[storage(BTreeStorage)]
pub struct Position {
    pub x: I32F32,
    pub y: I32F32,
    pub z: I32F32,
}

impl Position {
    pub fn new(x: I32F32, y: I32F32, z: I32F32) -> Self {
        Self { x, y, z }
    }

    pub fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }

    pub fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct MarkedForDeath;

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct MovementInfo {
    pub speed: I32F32,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Walkable;

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct ActionInsertIntoContainer {
    pub entity: Entity,
    pub container: Entity,
}

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct ActionTakeFromContainer {
    pub entity_type: EntityType,
    pub container: Entity,
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct ActionCraft {
    pub craft: fn(&LazyUpdate),
}

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct ActionAttack {
    pub target: Entity,
}

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct ActionMoveTowards {
    pub target: Position,
    pub path: Vec<Position>,
}

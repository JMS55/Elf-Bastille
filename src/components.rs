use specs::storage::{BTreeStorage, NullStorage};
use specs::{Component, Entity, LazyUpdate};
use specs_derive::Component;
use std::time::Duration;

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Container {
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

    pub fn try_insert(entity: Entity) -> Result<(), Entity> {
        // Remember to add ContainerChild
        unimplemented!("TODO")
    }

    pub fn try_take_out(entity_type: EntityType, destination: Entity) -> Result<Entity, ()> {
        // Remember to remove ContainerChild
        unimplemented!("TODO")
    }
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct ContainerChild {
    pub parent: Entity,
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Damageable {
    pub durability: u32,
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Growable {
    age: Duration,
}

impl Growable {
    pub fn new() -> Self {
        Self {
            age: Duration::from_nanos(0),
        }
    }
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
    pub set_action: fn(&LazyUpdate), // Responsible for deleting the old action component
}

#[derive(Component)]
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

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct MarkedForDeath;

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct MovementSpeed(u32);

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Walkable;

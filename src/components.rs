use crate::actions::Action;
use crate::context::Context;
use slotmap::DefaultKey;

pub struct Storage {
    pub entities: Vec<DefaultKey>,
    pub stored_volume: u32,
    pub stored_weight: u32,
    pub volume_limit: u32,
    pub weight_limit: Option<u32>,
}

impl Storage {
    pub fn new(volume_limit: u32, weight_limit: Option<u32>) -> Self {
        Self {
            entities: Vec::new(),
            stored_volume: 0,
            stored_weight: 0,
            volume_limit,
            weight_limit,
        }
    }

    pub fn try_insert(entity: DefaultKey) -> Result<(), DefaultKey> {
        // Remember to add StorageChild
        unimplemented!("TODO")
    }

    pub fn try_take(entity_type: EntityType, destination: DefaultKey) -> Result<DefaultKey, ()> {
        // Remember to remove StorageChild
        unimplemented!("TODO")
    }
}

#[derive(Copy, Clone)]
pub struct StorageChild {
    pub parent: DefaultKey,
}

#[derive(Copy, Clone)]
pub struct Damageable {
    pub durability: u32,
    pub damaged_by: AttackType,
}

#[derive(Copy, Clone)]
pub struct Growth(pub u32);

#[derive(Copy, Clone)]
pub struct Weapon {
    pub attack_type: AttackType,
}

#[derive(Copy, Clone)]
pub struct PhysicalProperties {
    pub volume: u32,
    pub weight: u32,
}

#[derive(Copy, Clone)]
pub struct Displayable {
    pub texture_atlas_index: u32,
}

#[derive(Copy, Clone)]
pub struct AI {
    pub get_action: fn(&Context, DefaultKey) -> Action,
}

#[derive(Copy, Clone)]
pub enum EntityType {
    Tree,
    Elf,
    Wood,
    Axe,
    Crate,
}

#[derive(Copy, Clone)]
pub struct Loot {
    pub create_loot: fn(&mut Context),
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Copy, Clone)]
pub enum AttackType {
    Cut,
}

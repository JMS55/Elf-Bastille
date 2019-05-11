use crate::util::Timer;
use fixed::types::I32F32;
use specs::storage::{BTreeStorage, NullStorage};
use specs::Component;
use specs_derive::Component;
use std::time::Duration;

#[derive(Component, Clone)]
#[storage(BTreeStorage)]
pub struct WorldLocation {
    pub x: I32F32,
    pub y: I32F32,
    pub z: I32F32,
    pub is_walkable: bool,
    pub texture_atlas_index: u32,
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Tree {
    pub growth_stage: TreeGrowthStage,
    pub growth_timer: Timer,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            growth_stage: TreeGrowthStage::Stage1,
            growth_timer: Timer::new(Duration::from_secs(5), false),
        }
    }
}

pub enum TreeGrowthStage {
    Stage1,
    Stage2,
    Stage3,
    Stage4,
    Stage5,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Dirt;

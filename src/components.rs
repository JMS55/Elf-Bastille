use crate::util::Timer;
use fixed::types::I32F32;
use specs::storage::{BTreeStorage, NullStorage};
use specs::Component;
use specs_derive::Component;
use std::time::Duration;

// Components //

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct LocationInfo {
    pub location: Location,
    pub is_walkable: bool,
    pub texture_atlas_index: u32,
}

#[derive(Clone)]
pub struct Location {
    pub x: I32F32,
    pub y: I32F32,
    pub z: I32F32,
}

impl Location {
    pub fn new<T: Into<I32F32>>(x: T, y: T, z: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
}

// Entities //

// #[derive(Component)]
// #[storage(BTreeStorage)]
// pub struct Elf {}

/*
    Elf component stores list of actions
    At the start of the turn, delete all action components, then add the correct action component
    Action systems run. If action completed, tell elf to advance actions

    How to represent actions?
*/

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

// Actions //
// #[derive(Component)]
// #[storage(BTreeStorage)]
// pub struct Movement {}

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

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct MovementInfo {
    pub speed: I32F32,
}

// Entities //

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Elf {
    pub action_queue: Vec<Action>,
}

impl Elf {
    pub fn new() -> Self {
        Self {
            action_queue: Vec::new(),
        }
    }

    pub fn queue_action(&mut self, action: ActionMove) {
        self.action_queue.push(Action::Move(action));
    }
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

// Actions //

pub enum Action {
    Move(ActionMove),
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct ActionMove {
    goal: Location,
    path: Vec<Location>,
}

impl ActionMove {
    pub fn new(goal: Location) -> Self {
        Self {
            goal,
            path: Vec::new(),
        }
    }
}

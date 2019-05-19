use crate::util::Timer;
use specs::storage::{BTreeStorage, NullStorage};
use specs::Component;
use specs_derive::Component;
use std::time::Duration;

// Components //

#[derive(Component, PartialEq, Eq, Hash)]
#[storage(BTreeStorage)]
pub struct LocationInfo {
    pub location: Location,
    pub is_walkable: bool,
    pub texture_atlas_index: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Location {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct MovementInfo {
    pub tiles_per_move: u32,
    pub timer: Timer,
}

impl MovementInfo {
    pub fn new(tiles_per_move: u32, cooldown_timer: Duration) -> Self {
        Self {
            tiles_per_move,
            timer: Timer::new(cooldown_timer, false),
        }
    }
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
    pub goal: Location,
    pub should_pathfind: bool,
    pub path: Vec<Location>,
}

impl ActionMove {
    pub fn new(goal: Location) -> Self {
        Self {
            goal,
            should_pathfind: true,
            path: Vec::new(),
        }
    }
}

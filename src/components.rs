use specs::storage::{BTreeStorage, NullStorage};
use specs::{Component, Entity, LazyUpdate, Read};
use specs_derive::Component;
use std::collections::HashSet;
use std::time::Duration;

// Components //

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Texture {
    pub atlas_index: u32,
}

#[derive(Component, PartialEq, Eq, Hash)]
#[storage(BTreeStorage)]
pub struct LocationInfo {
    pub location: Location,
    pub is_walkable: bool,
}

impl LocationInfo {
    pub fn new(location: Location, is_walkable: bool) -> Self {
        Self {
            location,
            is_walkable,
        }
    }
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

    pub fn is_adjacent_to(&self, other: &Self) -> bool {
        for offset in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let offsetted = Location::new(self.x + offset.0, self.y + offset.1, self.z);
            if &offsetted == other {
                return true;
            }
        }
        false
    }
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct MovementInfo {
    pub time_per_move: Duration,
    pub time_since_last_move: Duration,
}

impl MovementInfo {
    pub fn new(time_per_move: Duration) -> Self {
        Self {
            time_per_move,
            time_since_last_move: time_per_move,
        }
    }
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Inventory {
    pub stored_entities: HashSet<Entity>,
    pub volume_free: u32,
    pub weight_free: u32,
    pub max_volume: u32,
    pub max_weight: u32,
}

impl Inventory {
    pub fn new(max_volume: u32, max_weight: u32) -> Self {
        Self {
            stored_entities: HashSet::new(),
            volume_free: max_volume,
            weight_free: max_weight,
            max_volume,
            max_weight,
        }
    }
}

#[derive(Component, Copy, Clone)]
#[storage(BTreeStorage)]
pub struct StorageInfo {
    pub volume: u32,
    pub weight: u32,
}

impl StorageInfo {
    pub fn new(volume: u32, weight: u32) -> Self {
        Self { volume, weight }
    }
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct IsStored {
    pub inventory: Entity,
}

impl IsStored {
    pub fn new(inventory: Entity) -> Self {
        Self { inventory }
    }
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Attackable {
    pub durabillity_left: u32,
    pub max_durabillity: u32,
    pub vulnerable_to: WeaponType,
    pub on_destroy: Option<fn(Entity, &Read<LazyUpdate>)>, // Entity is the entity belonging to this component
}

impl Attackable {
    pub fn new(
        max_durabillity: u32,
        vulnerable_to: WeaponType,
        on_destroy: Option<fn(Entity, &Read<LazyUpdate>)>,
    ) -> Self {
        Self {
            durabillity_left: max_durabillity,
            max_durabillity,
            vulnerable_to,
            on_destroy,
        }
    }
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Weapon {
    pub damage_per_use: u32,
    pub uses_left: u32,
    pub max_uses: u32,
    pub recovery_time: Duration,
    pub weapon_type: WeaponType,
}

impl Weapon {
    pub fn new(
        damage_per_use: u32,
        max_uses: u32,
        recovery_time: Duration,
        weapon_type: WeaponType,
    ) -> Self {
        Self {
            damage_per_use,
            uses_left: max_uses,
            max_uses,
            recovery_time,
            weapon_type,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum WeaponType {
    Sword,
    Axe,
}

#[derive(Component, PartialEq, Eq)]
#[storage(BTreeStorage)]
pub enum CraftingMaterial {}

#[derive(Component, PartialEq, Eq)]
#[storage(BTreeStorage)]
pub enum CraftingAid {}

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

    pub fn queue_action<T: Into<Action>>(&mut self, action: T) {
        self.action_queue.push(action.into());
    }
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Tree {
    pub growth_stage: TreeGrowthStage,
    pub growth_timer: Duration,
    pub max_logs_dropped: u32, // Number of logs at the max growth stage, with each stage lower dropping 2 less, and the last dropping 0
}

impl Tree {
    pub fn new(max_logs_dropped: u32) -> Self {
        Self {
            growth_stage: TreeGrowthStage::Stage1,
            growth_timer: Duration::from_secs(0),
            max_logs_dropped,
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
    Store(ActionStore),
    Attack(ActionAttack),
    Craft(ActionCraft),
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

impl Into<Action> for ActionMove {
    fn into(self) -> Action {
        Action::Move(self)
    }
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct ActionStore {
    pub entity: Entity,
    pub destination: Entity,
    pub time_to_complete: Duration,
    pub time_passed_so_far: Duration,
    pub reserved: Option<StorageInfo>,
    pub entity_location: Option<Location>,
}

impl ActionStore {
    pub fn new(entity: Entity, destination: Entity, time_to_complete: Duration) -> Self {
        Self {
            entity,
            destination,
            time_to_complete,
            time_passed_so_far: Duration::from_secs(0),
            reserved: None,
            entity_location: None,
        }
    }
}

impl Into<Action> for ActionStore {
    fn into(self) -> Action {
        Action::Store(self)
    }
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct ActionAttack {
    pub target: Entity,
    pub recovery_time_left: Duration,
}

impl ActionAttack {
    pub fn new(target: Entity) -> Self {
        Self {
            target,
            recovery_time_left: Duration::from_secs(0),
        }
    }
}

impl Into<Action> for ActionAttack {
    fn into(self) -> Action {
        Action::Attack(self)
    }
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct ActionCraft {
    pub materials: Vec<CraftingMaterial>,
    pub aids: HashSet<CraftingAid>,
    pub output_storage_info: StorageInfo,
    pub create_output: fn(&Read<LazyUpdate>) -> Entity, // Returns the new entity created
    pub time_to_complete: Duration,
    pub time_passed_so_far: Duration,
    pub did_reserve: bool,
}

impl ActionCraft {
    pub fn new(
        materials: Vec<CraftingMaterial>,
        aids: HashSet<CraftingAid>,
        output_storage_info: StorageInfo,
        create_output: fn(&Read<LazyUpdate>) -> Entity,
        time_to_complete: Duration,
    ) -> Self {
        Self {
            materials,
            aids,
            output_storage_info,
            create_output,
            time_to_complete,
            time_passed_so_far: Duration::from_secs(0),
            did_reserve: false,
        }
    }
}

impl Into<Action> for ActionCraft {
    fn into(self) -> Action {
        Action::Craft(self)
    }
}

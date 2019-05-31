use crate::components::ActionAttack;
use crate::DELTA_TIME;
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage};

pub struct AttackSystem;

impl<'a> System<'a> for AttackSystem {
    type SystemData = (
        ReadStorage<'a, ActionAttack>,
        Read<'a, LazyUpdate>,
        Entities<'a>,
    );

    fn run(&mut self, (action_attack_data, lazy_update, entities): Self::SystemData) {}
}

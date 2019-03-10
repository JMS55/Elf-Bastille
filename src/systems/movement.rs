use crate::components::{Movement, Position};
use specs::{Entities, Join, System, WriteStorage};

pub struct MovementSystem {
    // pub updates_since_last_pathfind: u32,
}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Movement>,
    );

    fn run(&mut self, (enitities, mut position_data, mut movement_data): Self::SystemData) {
        for (entitiy, position) in (&enitities, &mut position_data).join() {
            if let Some(movement) = movement_data.get_mut(entitiy) {
                if let Some(target) = movement.target {}
            }
        }
    }
}

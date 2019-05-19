use crate::components::{ActionMove, LocationInfo, MovementInfo};
use specs::{Join, System, WriteStorage};
pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, ActionMove>,
        WriteStorage<'a, MovementInfo>,
        WriteStorage<'a, LocationInfo>,
    );

    fn run(
        &mut self,
        (mut action_move_data, mut movement_info, mut location_data): Self::SystemData,
    ) {
        // TODO
    }
}

use crate::components::{ActionMoveTowards, MovementInfo, Position};
use microprofile::scope;
use specs::{Join, ReadStorage, System, WriteStorage};
use std::collections::HashSet;

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, ActionMoveTowards>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, MovementInfo>,
    );

    fn run(
        &mut self,
        (mut action_move_towards_data, mut position_data, mut movement_info_data): Self::SystemData,
    ) {
        microprofile::scope!("systems", "movement");

        // Local copy of positions to use for obstacle detection, should be kept in sync when changing movement entity positions
        let mut obstacles = (&position_data)
            .join()
            .map(|position| position.to_owned())
            .collect::<HashSet<Position>>();

        let mut data = (
            &mut action_move_towards_data,
            &mut position_data,
            &movement_info_data,
        )
            .join()
            .collect::<Vec<_>>();
        data.sort_unstable_by(
            |(action_move_towards_a, _, _), (action_move_towards_b, _, _)| {
                action_move_towards_a
                    .path
                    .len()
                    .cmp(&action_move_towards_b.path.len())
            },
        );
        for (action_move_towards, position, movement_info) in data {
            /*
            TODO
                1. Check to make sure node is completely unoccupied
                2. figure out where path node is in relation to current position
                3. Add move_speed to the appropriate axis, but stopping when at the next node
                4. If leftover move_speed from step 3, goto step 1
                5. Update obstacles list
            */
        }
    }
}

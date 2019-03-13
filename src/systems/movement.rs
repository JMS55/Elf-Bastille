use crate::components::{Movement, Position};
use rayon::iter::ParallelIterator;
use specs::{Join, ParJoin, System, WriteStorage};
use std::collections::HashSet;

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (WriteStorage<'a, Position>, WriteStorage<'a, Movement>);

    fn run(&mut self, (mut position_data, mut movement_data): Self::SystemData) {
        let obstacle_positions = (&position_data, !&movement_data)
            .join()
            .map(|(position, _)| *position)
            .collect::<HashSet<_>>();
        (&mut position_data, &mut movement_data)
            .par_join()
            .for_each(|(position, movement)| {
                for _ in 0..movement.move_speed {
                    if let Some(potential_new_position) = movement.path.pop() {
                        if !obstacle_positions.contains(&potential_new_position) {
                            *position = potential_new_position;
                        }
                    } else {
                        movement.target = None;
                        break;
                    }
                }
            });
    }
}

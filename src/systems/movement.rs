use crate::components::{Movement, Position};
use microprofile::scope;
use specs::{Join, System, WriteStorage};
use std::collections::HashSet;

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (WriteStorage<'a, Position>, WriteStorage<'a, Movement>);

    fn run(&mut self, (mut position_data, mut movement_data): Self::SystemData) {
        microprofile::scope!("systems", "movement");

        // Local copy of positions to use for obstacle detection, should be kept in sync when changing movement entity positions
        let mut obstacle_positions = (&position_data)
            .join()
            .map(|position| position.to_owned())
            .collect::<HashSet<_>>();

        let mut data = (&mut position_data, &mut movement_data)
            .join()
            .collect::<Vec<_>>();
        data.sort_unstable_by(|(_, movement_a), (_, movement_b)| {
            movement_a.path.len().cmp(&movement_b.path.len())
        });
        for (position, movement) in data {
            for _ in 0..movement.move_speed {
                match movement.path.pop() {
                    Some(potential_new_position) => {
                        if !obstacle_positions.contains(&potential_new_position) {
                            obstacle_positions.remove(position);
                            obstacle_positions.insert(potential_new_position);
                            *position = potential_new_position;
                        }
                    }
                    None => break,
                }
            }

            movement.target = None;
            movement.path.clear();
        }
    }
}

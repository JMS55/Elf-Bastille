use crate::components::{ActionMoveTowards, MovementInfo, Position};
use crate::misc::Obstacles;
use microprofile::scope;
use specs::{Join, ReadStorage, System, WriteStorage};

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, ActionMoveTowards>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, MovementInfo>,
    );

    fn run(
        &mut self,
        (mut action_move_towards_data, mut position_data, movement_info_data): Self::SystemData,
    ) {
        microprofile::scope!("systems", "movement");

        // Local copy of positions to use for obstacle detection, should be kept in sync when changing movement entity positions
        let mut obstacles = Obstacles::new();
        for position in (&position_data).join() {
            obstacles.insert(*position);
        }

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
            obstacles.remove(*position);
            let mut move_speed_left = movement_info.speed;
            while let Some(path_node) = action_move_towards.path.pop() {
                if obstacles.contains(path_node) {
                    break;
                } else {
                    let mut axis = None;
                    if path_node.x != position.x {
                        axis = Some((&mut position.x, path_node.x));
                    } else if path_node.y != position.y {
                        axis = Some((&mut position.y, path_node.y));
                    } else if path_node.z != position.z {
                        axis = Some((&mut position.z, path_node.z));
                    }
                    if let Some(axis) = axis {
                        let difference = (*axis.0 - axis.1).abs();
                        if move_speed_left > difference {
                            move_speed_left -= difference;
                            *axis.0 = axis.1;
                        } else {
                            if *axis.0 > axis.1 {
                                *axis.0 -= move_speed_left;
                            } else {
                                *axis.0 += move_speed_left;
                            }
                            break;
                        }
                    }
                }
            }
            obstacles.insert(*position);
        }
    }
}

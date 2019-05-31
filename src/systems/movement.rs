use crate::components::{ActionMove, Location, LocationInfo, MovementInfo};
use crate::DELTA_TIME;
use specs::{Entities, Join, LazyUpdate, Read, System, WriteStorage};
use std::collections::HashMap;
use std::time::Duration;

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, ActionMove>,
        WriteStorage<'a, MovementInfo>,
        WriteStorage<'a, LocationInfo>,
        Read<'a, LazyUpdate>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (mut action_move_data, mut movement_info_data, mut location_data, lazy_update, entities): Self::SystemData,
    ) {
        let mut obstacles = (&location_data, (&action_move_data).maybe())
            .join()
            .map(|(location, action_move)| (location.location, action_move.is_some()))
            .collect::<HashMap<Location, bool>>();

        for (movement_info, action_move, location, entity) in (
            &mut movement_info_data,
            (&mut action_move_data).maybe(),
            (&mut location_data).maybe(),
            &entities,
        )
            .join()
        {
            movement_info.time_since_last_move += DELTA_TIME;
            if let Some(action_move) = action_move {
                if let Some(path_node) = action_move.path.get(0) {
                    let location =
                        location.expect("Entity with ActionMove did not have LocationInfo");
                    let mut wait_time = movement_info.time_per_move;
                    if path_node.z != location.location.z {
                        wait_time *= 2;
                    }
                    if movement_info.time_since_last_move >= wait_time {
                        if !obstacles.contains_key(path_node) {
                            obstacles.remove(&location.location);
                            location.location = *path_node;
                            obstacles.insert(location.location, true);
                            action_move.path.remove(0);
                            movement_info.time_since_last_move = Duration::from_secs(0);
                        } else {
                            if !obstacles[path_node] {
                                action_move.should_pathfind = true;
                            }
                            continue;
                        }
                    }
                } else {
                    lazy_update.remove::<ActionMove>(entity);
                }
            }
        }
    }
}

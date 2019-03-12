use crate::components::{Movement, Position};
use specs::{Join, System, WriteStorage};

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (WriteStorage<'a, Position>, WriteStorage<'a, Movement>);

    fn run(&mut self, (mut position_data, mut movement_data): Self::SystemData) {
        let position_data_raw = &position_data as *const WriteStorage<'a, Position>;
        for (position, movement) in (&mut position_data, &mut movement_data).join() {
            if let Some(_) = movement.target {
                if !(unsafe { &*position_data_raw })
                    .join()
                    .collect::<Vec<_>>()
                    .contains(&&movement.target.unwrap())
                {
                    *position = movement.target.unwrap();
                    movement.target = None;
                }
            }
        }
    }
}

// get a* path to target
// move along path based on movement.move_speed, not doing so if obstacle in the way
// if at target set target to None

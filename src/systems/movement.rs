use crate::components::{Movement, Position};
use specs::{Join, System, WriteStorage};

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (WriteStorage<'a, Position>, WriteStorage<'a, Movement>);

    fn run(&mut self, (mut position_data, mut movement_data): Self::SystemData) {
        for (position, movement) in (&mut position_data, &mut movement_data).join() {
            println!("{:?}", movement.target);
            *position = movement.target.unwrap();
        }
    }
}

// get a* path to target
// move along path based on movement.move_speed, not doing so if obstacle in the way
// if at target set target to None

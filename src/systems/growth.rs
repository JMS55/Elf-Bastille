use crate::components::{Displayable, Growable};
use crate::DELTA_TIME;
use microprofile::scope;
use specs::{Entities, Join, System, WriteStorage};
use std::time::Duration;

pub struct GrowthSystem;

impl<'a> System<'a> for GrowthSystem {
    type SystemData = (
        WriteStorage<'a, Growable>,
        WriteStorage<'a, Displayable>,
        Entities<'a>,
    );

    fn run(&mut self, (mut growable_data, mut displayable_data, entities): Self::SystemData) {
        microprofile::scope!("systems", "growth");

        for (growable, entity) in (&mut growable_data, &entities).join() {
            growable.age += DELTA_TIME;
            let new_texture_index = match growable.age {
                age if age >= Duration::from_secs(7 * 4) => 1,
                age if age >= Duration::from_secs(7 * 3) => 10,
                age if age >= Duration::from_secs(7 * 2) => 9,
                age if age >= Duration::from_secs(7) => 8,
                _ => 7,
            };
            if let Some(displayable) = displayable_data.get_mut(entity) {
                displayable.texture_atlas_index = new_texture_index;
            }
        }
    }
}

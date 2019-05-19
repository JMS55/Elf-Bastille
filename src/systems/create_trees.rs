use crate::components::{Dirt, Location, LocationInfo, Tree};
use crate::util::Timer;
use specs::{Builder, Entities, Join, LazyUpdate, Read, ReadStorage, System};
use std::time::Duration;

pub struct CreateTreesSystem {
    timer: Timer,
}

impl CreateTreesSystem {
    pub fn new() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(20), true),
        }
    }
}

impl<'a> System<'a> for CreateTreesSystem {
    type SystemData = (
        ReadStorage<'a, LocationInfo>,
        ReadStorage<'a, Tree>,
        ReadStorage<'a, Dirt>,
        Read<'a, LazyUpdate>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (location_data, tree_data, dirt_data, lazy_update, entities): Self::SystemData,
    ) {
        if self.timer.triggered() {
            let mut new_tree_locations = Vec::new();
            'dirt_loop: for (dirt_location, _) in (&location_data, &dirt_data).join() {
                // Randomness skip
                if rand::random() {
                    continue 'dirt_loop;
                }

                // Skip if an entity is above the dirt tile
                if (&location_data).join().any(|entity_location| {
                    entity_location.location.x == dirt_location.location.x
                        && entity_location.location.y == dirt_location.location.y
                        && entity_location.location.z == dirt_location.location.z + 1
                }) {
                    continue 'dirt_loop;
                }

                // Skip if any other tree exists in a radius of 2
                for tree_location in (&location_data, &tree_data)
                    .join()
                    .map(|(tree_location, _)| &tree_location.location)
                    .chain(&new_tree_locations)
                {
                    if (tree_location.x - dirt_location.location.x).abs() <= 2
                        && (tree_location.y - dirt_location.location.y).abs() <= 2
                        && (tree_location.z - dirt_location.location.z).abs() <= 2
                    {
                        continue 'dirt_loop;
                    }
                }

                // Create new tree
                let tree_location = Location::new(
                    dirt_location.location.x,
                    dirt_location.location.y,
                    dirt_location.location.z + 1,
                );
                new_tree_locations.push(tree_location);
                lazy_update
                    .create_entity(&entities)
                    .with(Tree::new())
                    .with(LocationInfo {
                        location: tree_location,
                        is_walkable: false,
                        texture_atlas_index: 2,
                    })
                    .build();
            }
        }
    }
}

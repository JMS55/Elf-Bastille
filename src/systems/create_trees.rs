use crate::components::{
    Attackable, Dirt, Location, LocationInfo, StorageInfo, Texture, Tree, TreeGrowthStage,
    WeaponType,
};
use crate::DELTA_TIME;
use rand::Rng;
use rand_pcg::Mcg128Xsl64;
use specs::{Builder, Entities, Entity, Join, LazyUpdate, Read, ReadStorage, System};
use std::time::Duration;

pub struct CreateTreesSystem {
    timer: Duration,
    rng: Mcg128Xsl64,
}

impl CreateTreesSystem {
    pub fn new() -> Self {
        Self {
            timer: Duration::from_secs(20),
            rng: Mcg128Xsl64::new(0xcafef00dd15ea5e5),
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
        self.timer += DELTA_TIME;
        if self.timer >= Duration::from_secs(20) {
            self.timer = Duration::from_secs(0);

            let mut new_tree_locations = Vec::new();
            'dirt_loop: for (dirt_location, _) in (&location_data, &dirt_data).join() {
                // Randomness skip
                if self.rng.gen::<bool>() {
                    continue 'dirt_loop;
                }

                // Skip if an entity is above the dirt tile
                if (&location_data).join().any(|entity_location| {
                    entity_location.location.x == dirt_location.location.x
                        && entity_location.location.y == dirt_location.location.y
                        && entity_location.location.z > dirt_location.location.z
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
                    .with(Tree::new(self.rng.gen_range(7, 14)))
                    .with(Texture { atlas_index: 2 })
                    .with(LocationInfo::new(tree_location, false))
                    .with(Attackable::new(
                        30,
                        WeaponType::Axe,
                        Some(create_pile_of_logs),
                    ))
                    .build();
            }
        }
    }
}

fn create_pile_of_logs(tree_entity: Entity, lazy_update: &Read<LazyUpdate>) {
    lazy_update.exec_mut(move |world| {
        let number_of_logs = {
            let tree_data = world.read_storage::<Tree>();
            let tree = tree_data
                .get(tree_entity)
                .expect("tree_entity did not have a Tree component in create_pile_of_logs()");
            match tree.growth_stage {
                TreeGrowthStage::Stage1 => 0,
                TreeGrowthStage::Stage2 => tree.max_logs_dropped - 6,
                TreeGrowthStage::Stage3 => tree.max_logs_dropped - 4,
                TreeGrowthStage::Stage4 => tree.max_logs_dropped - 2,
                TreeGrowthStage::Stage5 => tree.max_logs_dropped,
            }
        };

        let tree_location = world
            .read_storage::<LocationInfo>()
            .get(tree_entity)
            .expect("tree_entity did not have a LocationInfo component in create_pile_of_logs()")
            .location;

        for _ in 0..=number_of_logs {
            world
                .create_entity()
                .with(Texture { atlas_index: 7 })
                .with(LocationInfo::new(tree_location, false))
                .with(StorageInfo::new(10, 25, "Log"))
                .build();
        }
    });
}

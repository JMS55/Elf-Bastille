use crate::components::{Loot, MarkedForDeath};
use microprofile::scope;
use rayon::iter::ParallelIterator;
use specs::{LazyUpdate, ParJoin, Read, ReadStorage, System};

pub struct LootSystem;

impl<'a> System<'a> for LootSystem {
    type SystemData = (
        ReadStorage<'a, Loot>,
        ReadStorage<'a, MarkedForDeath>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (loot_data, marked_for_death_data, lazy_update): Self::SystemData) {
        microprofile::scope!("systems", "loot");

        (&loot_data, &marked_for_death_data)
            .par_join()
            .for_each(|(loot, _)| {
                (loot.on_removed_callback)(&lazy_update);
            });
    }
}

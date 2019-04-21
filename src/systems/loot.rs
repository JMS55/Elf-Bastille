use crate::components::Loot;
use microprofile::scope;
use rayon::iter::ParallelIterator;
use specs::{LazyUpdate, ParJoin, Read, ReadStorage, System};

pub struct LootSystem;

impl<'a> System<'a> for LootSystem {
    type SystemData = (ReadStorage<'a, Loot>, Read<'a, LazyUpdate>);

    fn run(&mut self, (loot_data, lazy_world): Self::SystemData) {
        microprofile::scope!("systems", "loot");

        &loot_data.par_join().for_each(|loot| {
            (loot.create_loot)(&lazy_world);
        });
    }
}

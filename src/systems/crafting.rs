use crate::components::ActionCraft;
use microprofile::scope;
use rayon::iter::ParallelIterator;
use specs::{LazyUpdate, ParJoin, Read, ReadStorage, System};

pub struct CraftingSystem;

impl<'a> System<'a> for CraftingSystem {
    type SystemData = (ReadStorage<'a, ActionCraft>, Read<'a, LazyUpdate>);

    fn run(&mut self, (action_craft_data, lazy_world): Self::SystemData) {
        microprofile::scope!("systems", "crafting");

        &action_craft_data.par_join().for_each(|action_craft| {
            (action_craft.craft)(&lazy_world);
        });
    }
}

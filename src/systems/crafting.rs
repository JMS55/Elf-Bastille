use crate::components::ActionCraft;
use microprofile::scope;
use rayon::iter::ParallelIterator;
use specs::{Entities, LazyUpdate, ParJoin, Read, ReadStorage, System};

pub struct CraftingSystem;

impl<'a> System<'a> for CraftingSystem {
    type SystemData = (
        ReadStorage<'a, ActionCraft>,
        Read<'a, LazyUpdate>,
        Entities<'a>,
    );

    fn run(&mut self, (action_craft_data, lazy_world, entities): Self::SystemData) {
        microprofile::scope!("systems", "crafting");

        (&action_craft_data, &entities)
            .par_join()
            .for_each(|(action_craft, entity)| {
                (action_craft.craft)(&lazy_world);
                lazy_world.remove::<ActionCraft>(entity);
            });
    }
}

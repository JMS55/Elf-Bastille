use crate::components::AI;
use microprofile::scope;
use rayon::iter::ParallelIterator;
use specs::{LazyUpdate, ParJoin, Read, ReadStorage, System};

pub struct AISystem;

impl<'a> System<'a> for AISystem {
    type SystemData = (ReadStorage<'a, AI>, Read<'a, LazyUpdate>);

    fn run(&mut self, (ai_data, lazy_world): Self::SystemData) {
        microprofile::scope!("systems", "ai");

        &ai_data.par_join().for_each(|ai| {
            (ai.set_action)(&lazy_world);
        });
    }
}

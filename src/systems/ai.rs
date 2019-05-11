use crate::components::AI;
use rayon::iter::ParallelIterator;
use specs::{Entities, LazyUpdate, ParJoin, Read, ReadStorage, System};

pub struct AISystem;

impl<'a> System<'a> for AISystem {
    type SystemData = (ReadStorage<'a, AI>, Read<'a, LazyUpdate>, Entities<'a>);

    fn run(&mut self, (ai_data, lazy_world, entities): Self::SystemData) {
        (&ai_data, &entities).par_join().for_each(|(ai, entity)| {
            (ai.set_action_callback)(entity, &lazy_world);
        });
    }
}

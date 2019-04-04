use crate::components::MarkedForDeath;
use microprofile::scope;
use rayon::iter::ParallelIterator;
use specs::{Entities, ParJoin, ReadStorage, System};

pub struct CleanupDeadSystem;

impl<'a> System<'a> for CleanupDeadSystem {
    type SystemData = (Entities<'a>, ReadStorage<'a, MarkedForDeath>);

    fn run(&mut self, (entities, marked_for_death_data): Self::SystemData) {
        microprofile::scope!("systems", "cleanup_dead");

        (&entities, &marked_for_death_data)
            .par_join()
            .for_each(|(entity, _)| entities.delete(entity).expect("Unable to delete entity"));
    }
}

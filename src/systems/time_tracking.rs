use crate::components::TimeTracker;
use crate::DELTA_TIME;
use rayon::iter::ParallelIterator;
use specs::{Entities, LazyUpdate, ParJoin, Read, System, WriteStorage};

pub struct TimeTrackingSystem;

impl<'a> System<'a> for TimeTrackingSystem {
    type SystemData = (
        WriteStorage<'a, TimeTracker>,
        Read<'a, LazyUpdate>,
        Entities<'a>,
    );

    fn run(&mut self, (mut time_tracker_data, lazy_world, entities): Self::SystemData) {
        (&mut time_tracker_data, &entities)
            .par_join()
            .for_each(|(time_tracker, entity)| {
                time_tracker.time_passed += DELTA_TIME;
                (time_tracker.callback)(time_tracker.time_passed, entity, &lazy_world);
            });
    }
}

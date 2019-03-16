use crate::components::{Item, ItemStorage};
use rayon::iter::ParallelIterator;
use specs::{ParJoin, ReadStorage, System, WriteStorage};

pub struct ItemStorageSystem;

impl<'a> System<'a> for ItemStorageSystem {
    type SystemData = (WriteStorage<'a, ItemStorage>, ReadStorage<'a, Item>);

    // Checks to make sure all entities inside an ItemStorage still exist and have an Item component
    fn run(&mut self, (mut item_storage_data, item_data): Self::SystemData) {
        (&mut item_storage_data)
            .par_join()
            .for_each(|item_storage| {
                item_storage
                    .items
                    .retain(|item| item_data.get(*item).is_some())
            });
    }
}

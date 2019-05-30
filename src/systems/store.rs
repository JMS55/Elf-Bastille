use crate::components::{ActionStore, Inventory, IsStored, Storable};
use specs::{Entities, Join, LazyUpdate, Read, System, WriteStorage};

pub struct StoreSystem;

impl<'a> System<'a> for StoreSystem {
    type SystemData = (
        WriteStorage<'a, ActionStore>,
        WriteStorage<'a, Inventory>,
        WriteStorage<'a, Storable>,
        WriteStorage<'a, IsStored>,
        Read<'a, LazyUpdate>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
            mut action_store_data,
            mut inventory_data,
            mut storable_data,
            mut is_stored_data,
            lazy_update,
            entities,
        ): Self::SystemData,
    ) {
        for action_store in (&mut action_store_data).join() {
            match &action_store.space_reserved {
                None => {}
                Some(space_reserved) => {}
            }
        }


        unimplemented!("TODO");
        /*
            Handles taking entity from (other inventory / on ground) and puts it into destination inventory
                Implicit source requirements:
                    If source is from inventory, be next to inventory
                    If source is on ground, be next to it
                Implicit destination requirements:
                    If destination is inventory, be next to it

            One time:
                if entity has IsStored: Remove and add back space to inventory
                if entity has LocationInfo: Remove
                Decrease free space in destination

            Ongoing:
                increase time
                Check if entity is dead:
                    Add back space to destination, remove action
                Check if destination is dead:
                    Remove action
                if enough time has passed:
                    Add IsStored component to entity. Put entity into destination. Remove action.
        */
    }
}

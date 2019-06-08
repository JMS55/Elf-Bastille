use crate::components::{ActionStore, Inventory, IsStored, LocationInfo, StorageInfo};
use crate::DELTA_TIME;
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage};

pub struct StorageSystem;

impl<'a> System<'a> for StorageSystem {
    type SystemData = (
        WriteStorage<'a, ActionStore>,
        WriteStorage<'a, Inventory>,
        ReadStorage<'a, StorageInfo>,
        WriteStorage<'a, IsStored>,
        ReadStorage<'a, LocationInfo>,
        Read<'a, LazyUpdate>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
            mut action_store_data,
            mut inventory_data,
            storage_info_data,
            mut is_stored_data,
            location_data,
            lazy_update,
            entities,
        ): Self::SystemData,
    ) {
        for (action_store, action_entity) in (&mut action_store_data, &entities).join() {
            // Runs once at the start //
            if action_store.reserved.is_none() {
                let entity_storage_info = storage_info_data
                    .get(action_store.entity)
                    .expect("Entity of ActionStore had no StorageInfo component");
                let destination_inventory = inventory_data
                    .get_mut(action_store.destination)
                    .expect("Destination entity of ActionStore had no Inventory component");
                let destination_location = location_data
                    .get(action_store.destination)
                    .expect("Destination entity of ActionStore had no LocationInfo component")
                    .location;

                // Assign action_store.entity_location
                if let Some(is_stored) = is_stored_data.get(action_store.entity) {
                    action_store.entity_location = Some(location_data.get(is_stored.inventory).expect("Entity of ActionStore had previous IsStored component but the parent Inventory entity had no LocationInfo component").location.clone());
                } else {
                    action_store.entity_location = Some(
                        location_data
                            .get(action_store.entity)
                            .expect("Entity of ActionStore had no IsStored component, but also had no LocationInfo component")
                            .location
                            .clone(),
                    );
                }

                // Reserve volume and weight in destination if there's room and locations are correct
                if destination_inventory.volume_free >= entity_storage_info.volume
                    && destination_inventory.weight_free >= entity_storage_info.weight
                    && action_store
                        .entity_location
                        .unwrap()
                        .is_adjacent_to(&destination_location)
                {
                    destination_inventory.volume_free -= entity_storage_info.volume;
                    destination_inventory.weight_free -= entity_storage_info.weight;
                    action_store.reserved = Some(entity_storage_info.clone());

                    // Remove old components
                    lazy_update.remove::<LocationInfo>(action_store.entity);
                    if let Some(is_stored) = is_stored_data.get(action_store.entity) {
                        let previous_inventory = inventory_data.get_mut(is_stored.inventory).expect("Inventory entity of IsStored component did not have an Inventory component during ActionStore");
                        previous_inventory
                            .stored_entities
                            .remove(&action_store.entity);
                        previous_inventory.volume_free += entity_storage_info.volume;
                        previous_inventory.weight_free += entity_storage_info.weight;
                        is_stored_data.remove(action_store.entity);
                    }
                } else {
                    lazy_update.remove::<ActionStore>(action_entity);
                    continue;
                }
            }

            // Runs until action completes //
            action_store.time_passed_so_far += DELTA_TIME;

            // Checks
            if !entities.is_alive(action_store.destination) {
                lazy_update.remove::<ActionStore>(action_entity);
                continue;
            }
            let destination_location = location_data
                .get(action_store.destination)
                .expect("Destination entity of ActionStore had no LocationInfo component")
                .location;
            if !entities.is_alive(action_store.entity)
                || !action_store
                    .entity_location
                    .unwrap()
                    .is_adjacent_to(&destination_location)
            {
                let reserved = action_store.reserved.unwrap();
                let destination_inventory = inventory_data
                    .get_mut(action_store.destination)
                    .expect("Destination entity of ActionStore had no Inventory component");
                destination_inventory.volume_free += reserved.volume;
                destination_inventory.weight_free += reserved.weight;
                lazy_update.remove::<ActionStore>(action_entity);
                continue;
            }

            if action_store.time_passed_so_far >= action_store.time_to_complete {
                is_stored_data
                    .insert(action_store.entity, IsStored::new(action_store.destination))
                    .unwrap();
                let destination_inventory = inventory_data
                    .get_mut(action_store.destination)
                    .expect("Destination entity of ActionStore had no Inventory component");
                destination_inventory
                    .stored_entities
                    .insert(action_store.entity);
                lazy_update.remove::<ActionStore>(action_entity);
            }
        }
    }
}

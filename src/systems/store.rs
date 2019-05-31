use crate::components::{ActionStore, Inventory, IsStored, LocationInfo, StorageInfo};
use crate::DELTA_TIME;
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage};

pub struct StoreSystem;

impl<'a> System<'a> for StoreSystem {
    type SystemData = (
        WriteStorage<'a, ActionStore>,
        WriteStorage<'a, Inventory>,
        ReadStorage<'a, StorageInfo>,
        WriteStorage<'a, IsStored>,
        WriteStorage<'a, LocationInfo>,
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
            mut location_info_data,
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
                    .expect("Destination entity of ActionMove had no Inventory component");

                // Reserve volume and weight in destination if there's room
                if destination_inventory.volume_free >= entity_storage_info.volume
                    && destination_inventory.weight_free >= entity_storage_info.weight
                {
                    destination_inventory.volume_free -= entity_storage_info.volume;
                    destination_inventory.weight_free -= entity_storage_info.weight;
                    action_store.reserved = Some(entity_storage_info.clone());

                    // Remove old components
                    location_info_data.remove(action_store.entity);
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
                }
            }

            // Runs until action completes //
            action_store.time_passed_so_far += DELTA_TIME;

            if !entities.is_alive(action_store.destination) {
                lazy_update.remove::<ActionStore>(action_entity);
            }

            if !entities.is_alive(action_store.entity) {
                let reserved = action_store.reserved.unwrap();
                let destination_inventory = inventory_data
                    .get_mut(action_store.destination)
                    .expect("Destination entity of ActionMove had no Inventory component");
                destination_inventory.volume_free += reserved.volume;
                destination_inventory.weight_free += reserved.weight;
                lazy_update.remove::<ActionStore>(action_entity);
            }

            if action_store.time_passed_so_far >= action_store.time_to_complete {
                is_stored_data
                    .insert(action_store.entity, IsStored::new(action_store.destination))
                    .unwrap();
                let destination_inventory = inventory_data
                    .get_mut(action_store.destination)
                    .expect("Destination entity of ActionMove had no Inventory component");
                destination_inventory
                    .stored_entities
                    .insert(action_store.entity);
                lazy_update.remove::<ActionStore>(action_entity);
            }
        }
    }
}

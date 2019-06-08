use crate::components::{
    ActionCraft, CraftingAid, CraftingMaterial, Inventory, IsStored, LocationInfo, StorageInfo,
};
use crate::DELTA_TIME;
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage};
use std::collections::HashSet;

pub struct CraftSystem;

impl<'a> System<'a> for CraftSystem {
    type SystemData = (
        WriteStorage<'a, ActionCraft>,
        ReadStorage<'a, CraftingMaterial>,
        ReadStorage<'a, CraftingAid>,
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
            mut action_craft_data,
            crafting_material_data,
            crafting_aid_data,
            mut inventory_data,
            storage_info_data,
            mut is_stored_data,
            location_data,
            lazy_update,
            entities,
        ): Self::SystemData,
    ) {
        for (action_craft, elf_inventory, elf_location, action_entity) in (
            &mut action_craft_data,
            &mut inventory_data,
            &location_data,
            &entities,
        )
            .join()
        {
            // Runs once at the start //
            if !action_craft.did_reserve {
                let mut crafting_material_entities =
                    HashSet::with_capacity(action_craft.materials.len());

                // Check that all materials required are present
                for crafting_material in &action_craft.materials {
                    for entity in elf_inventory.stored_entities.clone() {
                        if crafting_material_data.get(entity) == Some(&crafting_material)
                            && !crafting_material_entities.contains(&entity)
                        {
                            crafting_material_entities.insert(entity);
                        }
                    }
                }
                if crafting_material_entities.len() != action_craft.materials.len() {
                    lazy_update.remove::<ActionCraft>(action_entity);
                    continue;
                }

                // Check that there is room for output after materials are removed, reserve it, and then delete materials
                let mut new_volume_free = elf_inventory.volume_free;
                let mut new_weight_free = elf_inventory.weight_free;
                for crafting_material_entity in &crafting_material_entities {
                    let crafting_material_storage_info =
                        storage_info_data.get(*crafting_material_entity).expect(
                            "Entity from Inventory in ActionCraft had no StorageInfo component",
                        );
                    new_volume_free += crafting_material_storage_info.volume;
                    new_weight_free += crafting_material_storage_info.weight;
                }
                if new_volume_free >= action_craft.output_storage_info.volume
                    && new_weight_free >= action_craft.output_storage_info.weight
                {
                    action_craft.did_reserve = true;
                    elf_inventory.volume_free =
                        new_volume_free - action_craft.output_storage_info.volume;
                    elf_inventory.weight_free =
                        new_weight_free - action_craft.output_storage_info.weight;
                    for crafting_material_entity in crafting_material_entities {
                        elf_inventory
                            .stored_entities
                            .remove(&crafting_material_entity);
                        entities.delete(crafting_material_entity).unwrap();
                    }
                } else {
                    lazy_update.remove::<ActionCraft>(action_entity);
                    continue;
                }
            }

            // Runs until action completes //
            action_craft.time_passed_so_far += DELTA_TIME;

            // Check that all aids required are present
            let mut all_aids_present = true;
            'outer: for crafting_aid in &action_craft.aids {
                for entity in &elf_inventory.stored_entities {
                    if crafting_aid_data.get(*entity) == Some(crafting_aid) {
                        continue 'outer;
                    }
                }

                for (entity, _) in (&entities, &location_data).join().filter(|(_, location)| {
                    location.location.is_adjacent_to(&elf_location.location)
                }) {
                    if crafting_aid_data.get(entity) == Some(crafting_aid) {
                        continue 'outer;
                    }
                }

                all_aids_present = false;
                break 'outer;
            }
            if !all_aids_present {
                elf_inventory.volume_free += action_craft.output_storage_info.volume;
                elf_inventory.weight_free += action_craft.output_storage_info.weight;
                lazy_update.remove::<ActionCraft>(action_entity);
                continue;
            }

            if action_craft.time_passed_so_far >= action_craft.time_to_complete {
                let output_entity = (action_craft.create_output)(&lazy_update);
                is_stored_data
                    .insert(output_entity, IsStored::new(action_entity))
                    .unwrap();
                elf_inventory.stored_entities.insert(output_entity);
                lazy_update.remove::<ActionCraft>(action_entity);
            }
        }
    }
}

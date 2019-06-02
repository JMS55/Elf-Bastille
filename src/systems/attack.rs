use crate::components::{
    ActionAttack, Attackable, Inventory, Location, LocationInfo, StorageInfo, Weapon,
};
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage};

pub struct AttackSystem;

impl<'a> System<'a> for AttackSystem {
    type SystemData = (
        ReadStorage<'a, ActionAttack>,
        WriteStorage<'a, Weapon>,
        WriteStorage<'a, Attackable>,
        WriteStorage<'a, Inventory>,
        ReadStorage<'a, StorageInfo>,
        ReadStorage<'a, LocationInfo>,
        Read<'a, LazyUpdate>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
            action_attack_data,
            mut weapon_data,
            mut attackable_data,
            mut inventory_data,
            storage_info_data,
            location_data,
            lazy_update,
            entities,
        ): Self::SystemData,
    ) {
        for (action_attack, elf_inventory, elf_location, action_entity) in (
            &action_attack_data,
            &mut inventory_data,
            &location_data,
            &entities,
        )
            .join()
        {
            // Check that target still exists
            if !entities.is_alive(action_attack.target) {
                lazy_update.remove::<ActionAttack>(action_entity);
                continue;
            }

            // Check that target hasn't already been killed by another entity in a previous iteration of this system
            let target_attackable = attackable_data
                .get_mut(action_attack.target)
                .expect("Target of ActionAttack had no Attackable component");
            if target_attackable.durabillity_left == 0 {
                lazy_update.remove::<ActionAttack>(action_entity);
                continue;
            }

            let target_location = location_data
                .get(action_attack.target)
                .expect("Target of ActionAttack had no LocationInfo component");
            // Check if adjacent to target
            if elf_location.is_adjacent_to(target_location) {
                // Check for a weapon that matches with target
                let mut found_weapon = false;
                for entity in &elf_inventory.stored_entities.clone() {
                    if let Some(weapon) = weapon_data.get_mut(*entity) {
                        if weapon.weapon_type == target_attackable.vulnerable_to {
                            // Damage target
                            target_attackable.durabillity_left = target_attackable
                                .durabillity_left
                                .checked_sub(weapon.damage_per_use)
                                .unwrap_or(0);
                            if target_attackable.durabillity_left == 0 {
                                entities.delete(action_attack.target).unwrap();
                                lazy_update.remove::<ActionAttack>(action_entity);
                            }

                            // Subtract use from weapon
                            weapon.uses_left = weapon.uses_left.checked_sub(1).unwrap_or(0);
                            if weapon.uses_left == 0 {
                                let weapon_storage_info = storage_info_data.get(*entity).unwrap();
                                elf_inventory.stored_entities.remove(entity);
                                elf_inventory.volume_free += weapon_storage_info.volume;
                                elf_inventory.weight_free += weapon_storage_info.weight;
                                let _ = entities.delete(*entity);
                            }

                            found_weapon = true;
                            break;
                        }
                    }
                }
                if !found_weapon {
                    lazy_update.remove::<ActionAttack>(action_entity);
                }
            } else {
                lazy_update.remove::<ActionAttack>(action_entity);
            }
        }
    }
}

impl LocationInfo {
    fn is_adjacent_to(&self, other: &Self) -> bool {
        for offset in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let offsetted = Location::new(
                self.location.x + offset.0,
                self.location.y + offset.1,
                self.location.z,
            );
            if offsetted == other.location {
                return true;
            }
        }
        false
    }
}
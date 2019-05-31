use crate::components::{
    ActionAttack, Attackable, Inventory, Location, LocationInfo, Weapon,
};
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage};

pub struct AttackSystem;

impl<'a> System<'a> for AttackSystem {
    type SystemData = (
        ReadStorage<'a, ActionAttack>,
        WriteStorage<'a, Weapon>,
        WriteStorage<'a, Attackable>,
        ReadStorage<'a, Inventory>,
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
            inventory_data,
            location_data,
            lazy_update,
            entities,
        ): Self::SystemData,
    ) {
        for (action_attack, elf_inventory, elf_location, action_entity) in (
            &action_attack_data,
            &inventory_data,
            &location_data,
            &entities,
        )
            .join()
        {
            // Check that target still exists
            // TODO: Have system maintain list of dead entities for use here as well
            if !entities.is_alive(action_attack.target) {
                lazy_update.remove::<ActionAttack>(action_entity);
            }
            let target_location = location_data
                .get(action_attack.target)
                .expect("Target of ActionAttack had no LocationInfo component");
            // Check if adjacent to target
            if elf_location.is_adjacent_to(target_location) {
                let target_attackable = attackable_data
                    .get_mut(action_attack.target)
                    .expect("Target of ActionAttack had no Attackable component");
                // Check for a weapon that matches with target
                if let Some(weapon) = elf_inventory
                    .stored_entities
                    .iter()
                    .filter_map(|entity| weapon_data.get_mut(*entity))
                    .find(|weapon| weapon.weapon_type == target_attackable.vulnerable_to)
                {
                    // Damage target
                    target_attackable.durabillity_left = target_attackable
                        .durabillity_left
                        .checked_sub(weapon.damage_per_use)
                        .unwrap_or(0);
                    if target_attackable.durabillity_left == 0 {
                        let _ = entities.delete(action_attack.target);
                        lazy_update.remove::<ActionAttack>(action_entity);
                    }

                    // Subtract use from weapon
                    weapon.uses_left = weapon.uses_left
                        .checked_sub(1)
                        .unwrap_or(0);
                    // TODO: delete weapon and update inventory
                } else {
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

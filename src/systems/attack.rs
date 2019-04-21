use crate::components::{ActionAttack, Damageable, MarkedForDeath};
use microprofile::scope;
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage};

pub struct AttackSystem;

impl<'a> System<'a> for AttackSystem {
    type SystemData = (
        ReadStorage<'a, ActionAttack>,
        WriteStorage<'a, Damageable>,
        WriteStorage<'a, MarkedForDeath>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            action_attack_data,
            mut damageable_data,
            mut marked_for_death_data,
            entities,
            lazy_world,
        ): Self::SystemData,
    ) {
        microprofile::scope!("systems", "attack");

        for (action_attack, entity) in (&action_attack_data, &entities).join() {
            let damageable = damageable_data
                .get_mut(action_attack.target)
                .expect("ActionAttack target was not Damageable");
            damageable.durability -= 1;
            if damageable.durability == 0 {
                marked_for_death_data
                    .insert(action_attack.target, MarkedForDeath)
                    .expect("Could not mark ActionAttack target as dead");
            }
            lazy_world.remove::<ActionAttack>(entity);
        }
    }
}

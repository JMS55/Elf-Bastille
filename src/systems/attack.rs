use crate::components::{ActionAttack, Damageable, MarkedForDeath};
use microprofile::scope;
use specs::{Join, ReadStorage, System, WriteStorage};

pub struct AttackSystem;

impl<'a> System<'a> for AttackSystem {
    type SystemData = (
        ReadStorage<'a, ActionAttack>,
        WriteStorage<'a, Damageable>,
        WriteStorage<'a, MarkedForDeath>,
    );

    fn run(
        &mut self,
        (action_attack_data, mut damageable_data, mut marked_for_death_data): Self::SystemData,
    ) {
        microprofile::scope!("systems", "attack");

        for action_attack in (&action_attack_data).join() {
            let damageable = damageable_data
                .get_mut(action_attack.target)
                .expect("TODO: EVEN MOR ERRRRR");
            damageable.durability -= 1;
            if damageable.durability == 0 {
                marked_for_death_data
                    .insert(action_attack.target, MarkedForDeath)
                    .expect("TODO: AHHHH ERRRR");
            }
        }
    }
}

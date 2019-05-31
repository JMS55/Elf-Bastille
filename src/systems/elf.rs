use crate::components::*;
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage};

pub struct ElfSystem;

impl<'a> System<'a> for ElfSystem {
    type SystemData = (
        WriteStorage<'a, Elf>,
        ReadStorage<'a, ActionMove>,
        ReadStorage<'a, ActionStore>,
        ReadStorage<'a, ActionAttack>,
        Read<'a, LazyUpdate>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
            mut elf_data,
            action_move_data,
            action_store_data,
            action_attack_data,
            lazy_update,
            entities,
        ): Self::SystemData,
    ) {
        for (elf, elf_entity, _, _, _) in (
            &mut elf_data,
            &entities,
            !&action_move_data,
            !&action_store_data,
            !&action_attack_data,
        )
            .join()
        {
            if !elf.action_queue.is_empty() {
                match elf.action_queue.remove(0) {
                    Action::Move(action) => lazy_update.insert(elf_entity, action),
                    Action::Store(action) => lazy_update.insert(elf_entity, action),
                    Action::Attack(action) => lazy_update.insert(elf_entity, action),
                }
            }
        }
    }
}

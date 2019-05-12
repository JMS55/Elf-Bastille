use crate::components::{Action, ActionMove, Elf};
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage};

pub struct ElfSystem;

impl<'a> System<'a> for ElfSystem {
    type SystemData = (
        WriteStorage<'a, Elf>,
        ReadStorage<'a, ActionMove>,
        Read<'a, LazyUpdate>,
        Entities<'a>,
    );

    fn run(&mut self, (mut elf_data, action_move_data, lazy_update, entities): Self::SystemData) {
        for (elf, elf_entity, _) in (&mut elf_data, &entities, !&action_move_data).join() {
            match elf.action_queue.remove(0) {
                Action::Move(action) => lazy_update.insert(elf_entity, action),
            }
        }
    }
}

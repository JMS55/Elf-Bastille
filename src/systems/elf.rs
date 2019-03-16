use crate::components::{Elf, Movement, Position, Tree};
use microprofile::scope;
use specs::{Entities, Join, ReadStorage, System, WriteStorage};

pub struct ElfSystem;

impl<'a> System<'a> for ElfSystem {
    type SystemData = (
        ReadStorage<'a, Elf>,
        WriteStorage<'a, Tree>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Movement>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (elf_data, mut tree_data, position_data, mut movement_data, entities): Self::SystemData,
    ) {
        microprofile::scope!("systems", "elf");

        for (_, elf_position, elf_movement) in
            (&elf_data, &position_data, &mut movement_data).join()
        {
            // If adjacent to alive tree, damage it, marking it for deletion if killed
            if let Some((entity, tree, _)) = (&entities, &mut tree_data, &position_data)
                .join()
                .find(|(_, tree, tree_position)| {
                    tree.durability != 0 && elf_position.get_adjacent().contains(tree_position)
                })
            {
                tree.durability -= 1;
                if tree.durability == 0 {
                    entities.delete(entity).unwrap();
                }
            }
            // If not adjacent to tree set target as closet alive tree (ignoring obstacles)
            else {
                {
                    elf_movement.target = (&tree_data, &position_data)
                        .join()
                        .filter_map(|(tree, tree_position)| match tree.durability {
                            0 => None,
                            _ => Some(*tree_position),
                        })
                        .min_by_key(|tree_position| elf_position.get_distance_from(*tree_position))
                }
            }
        }
    }
}

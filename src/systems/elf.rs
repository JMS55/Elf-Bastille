use crate::components::{Elf, Movement, Position, Tree};
use specs::{Entities, Join, ReadStorage, System, WriteStorage};
use std::collections::{HashSet, VecDeque};

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
        for (_, elf_position, elf_movement) in
            (&elf_data, &position_data, &mut movement_data).join()
        {
            // If adjacent to alive tree, damage it, marking it for deletion if killed
            if let Some((entity, tree, _)) = (&entities, &mut tree_data, &position_data)
                .join()
                .filter(|(_, tree, tree_position)| {
                    tree.durability != 0 && elf_position.get_adjacent().contains(tree_position)
                })
                .next()
            {
                tree.durability -= 1;
                if tree.durability == 0 {
                    entities.delete(entity).unwrap();
                }
            } else {
                if elf_movement.target == None {
                    let mut frontier = VecDeque::new();
                    let mut visited = HashSet::new();
                    frontier.push_back(*elf_position);

                    while !frontier.is_empty() {
                        let visiting = frontier.pop_front().unwrap();

                        // Check if visiting is adjacent to a tree that's not dead
                        let alive_tree_positions = (&tree_data, &position_data)
                            .join()
                            .filter_map(|(tree, position)| match tree.durability {
                                0 => None,
                                _ => Some(position),
                            })
                            .collect::<HashSet<_>>();
                        if visiting
                            .get_adjacent()
                            .iter()
                            .filter(|adjacent| alive_tree_positions.contains(adjacent))
                            .count()
                            != 0
                        {
                            elf_movement.target = Some(visiting);
                            break;
                        }

                        // Get unoccupied adjacent tiles
                        let positions = &position_data.join().collect::<HashSet<_>>();
                        for adjacent in visiting
                            .get_adjacent()
                            .into_iter()
                            .filter(|adjacent| !positions.contains(adjacent))
                        {
                            if !visited.contains(&adjacent) {
                                frontier.push_back(adjacent);
                                visited.insert(visiting);
                            }
                        }
                    }
                }
            }
        }
    }
}

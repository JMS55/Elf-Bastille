use crate::components::{Elf, Movement, Position, Tree};
use crate::{WORLD_HEIGHT, WORLD_WIDTH};
use specs::{Join, ReadStorage, System, WriteStorage};
use std::collections::{HashMap, VecDeque};

pub struct ElfSystem;

impl<'a> System<'a> for ElfSystem {
    type SystemData = (
        ReadStorage<'a, Elf>,
        ReadStorage<'a, Tree>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Movement>,
    );

    fn run(&mut self, (elf_data, tree_data, position_data, mut movement_data): Self::SystemData) {
        'elf_loop: for (_, elf_position, elf_movement) in
            (&elf_data, &position_data, &mut movement_data).join()
        {
            // TODO: Check if elf is adjacent to tree first. If not check if has target. If not do below

            let mut frontier = VecDeque::new();
            let mut came_from = HashMap::new();
            let mut target = None;
            frontier.push_back(*elf_position);
            came_from.insert(*elf_position, None);

            'frontier_loop: while !frontier.is_empty() {
                let current = frontier.pop_front().unwrap();

                for (_, tree_position) in (&tree_data, &position_data).join() {
                    if current == *tree_position {
                        target = Some(current);
                        break 'frontier_loop;
                    }
                }

                // Get neighbors
                for offset in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
                    let next = (current.x as i32 + offset.0, current.y as i32 + offset.1);
                    // World bounds check
                    if next.0 < 0
                        || next.1 < 0
                        || next.0 > WORLD_WIDTH as i32
                        || next.1 > WORLD_HEIGHT as i32
                    {
                        continue;
                    }
                    let next = Position {
                        x: next.0 as u32,
                        y: next.1 as u32,
                    };
                    // Unoccupied check
                    // if !position_data
                    //     .join()
                    //     .filter(|position| position == &&next)
                    //     .collect::<Vec<&Position>>()
                    //     .is_empty()
                    // {
                    //     continue;
                    // }

                    if !came_from.contains_key(&next) {
                        frontier.push_back(next);
                        came_from.insert(next, Some(current));
                    }
                }
            }

            if let Some(mut current) = target {
                let mut path = Vec::new();
                while current != *elf_position {
                    path.push(current);
                    current = came_from.get(&current).unwrap().unwrap();
                }
                elf_movement.target = Some(path[path.len() - 1]);
            }
        }
    }
}

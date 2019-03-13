use crate::components::{Movement, Position};
use rayon::iter::ParallelIterator;
use specs::{Join, ParJoin, ReadStorage, System, WriteStorage};
use std::cmp::{Ord, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};

pub struct PathfindingSystem;

impl<'a> System<'a> for PathfindingSystem {
    type SystemData = (WriteStorage<'a, Movement>, ReadStorage<'a, Position>);

    // A* pathfind in parallel to adjacent tile to target, using non movement positions for collision
    fn run(&mut self, (mut movement_data, position_data): Self::SystemData) {
        let obstacle_positions = (&position_data, !&movement_data)
            .join()
            .map(|(position, _)| *position)
            .collect::<HashSet<_>>();
        (&mut movement_data, &position_data)
            .par_join()
            .for_each(|(movement, position)| {
                if movement.path.is_empty() && movement.target.is_some() {
                    if let Some(goal) = movement
                        .target
                        .unwrap()
                        .get_adjacent()
                        .iter()
                        .filter(|position| !obstacle_positions.contains(position))
                        .next()
                    {
                        let mut frontier = BinaryHeap::new();
                        let mut came_from = HashMap::new();
                        let mut cost_so_far = HashMap::new();
                        frontier.push(FrontierState::new(*position, 0));
                        came_from.insert(*position, None);
                        cost_so_far.insert(*position, 0);

                        while !frontier.is_empty() {
                            let visiting = frontier.pop().unwrap();

                            if visiting.position == *goal {
                                break;
                            }

                            for adjacent in visiting
                                .position
                                .get_adjacent()
                                .into_iter()
                                .filter(|adjacent| !obstacle_positions.contains(adjacent))
                            {
                                let new_cost = cost_so_far.get(&visiting.position).unwrap() + 1;
                                if !cost_so_far.contains_key(&adjacent)
                                    || new_cost < cost_so_far[&adjacent]
                                {
                                    cost_so_far.insert(adjacent, new_cost);
                                    let priority = new_cost + goal.get_distance_from(&adjacent);
                                    frontier.push(FrontierState::new(adjacent, priority));
                                    came_from.insert(adjacent, Some(visiting));
                                }
                            }
                        }

                        let mut path = Vec::with_capacity(came_from.len());
                        let mut current = *goal;
                        while current != *position {
                            path.push(current);
                            current = came_from.get(&current).unwrap().unwrap().position;
                        }
                        movement.path = path;
                    }
                }
            });
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct FrontierState {
    position: Position,
    cost: u32,
}

impl FrontierState {
    fn new(position: Position, cost: u32) -> Self {
        Self { position, cost }
    }
}

impl Ord for FrontierState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for FrontierState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

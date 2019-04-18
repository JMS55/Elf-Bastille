use crate::components::{ActionMoveTowards, Position};
use crate::misc::Obstacles;
use fixed::types::I32F32;
use microprofile::scope;
use rayon::iter::ParallelIterator;
use specs::{Join, ParJoin, ReadStorage, System, WriteStorage};
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::{BinaryHeap, HashMap};

pub struct PathfindingSystem;

impl<'a> System<'a> for PathfindingSystem {
    type SystemData = (
        WriteStorage<'a, ActionMoveTowards>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, (mut action_move_towards_data, position_data): Self::SystemData) {
        microprofile::scope!("systems", "pathfinding");

        let mut obstacles = Obstacles::new();
        for position in (&position_data).join() {
            obstacles.insert(*position);
        }

        (&mut action_move_towards_data, &position_data)
            .par_join()
            .for_each(|(action_move_towards, position)| {
                let position = position.floor();
                if let Some(goal) = action_move_towards
                    .target
                    .get_adjacent()
                    .into_iter()
                    // TODO: Check to make sure each tile is Walkable
                    .find(|adjacent_to_target| !obstacles.contains(**adjacent_to_target))
                {
                    let mut frontier = BinaryHeap::new();
                    let mut came_from = HashMap::new();
                    let mut cost_so_far = HashMap::new();
                    frontier.push(FrontierState::new(position, I32F32::from(0)));
                    came_from.insert(position, None);
                    cost_so_far.insert(position, I32F32::from(0));

                    while !frontier.is_empty() {
                        let visiting = frontier.pop().unwrap();

                        if visiting.position == *goal {
                            break;
                        }

                        // TODO: Check to make sure each tile is Walkable
                        for adjacent in visiting.position.get_valid_neighbors(&obstacles) {
                            let new_cost = cost_so_far[&visiting.position] + I32F32::from(1);
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

                    let mut current = *goal;
                    while current != position {
                        action_move_towards.path.push(current);
                        current = came_from[&current].unwrap().position;
                    }
                }
            });
    }
}

impl Position {
    fn get_adjacent(&self) -> [Self; 4] {
        [
            Self::new(self.x + I32F32::from(1), self.y, self.z),
            Self::new(self.x - I32F32::from(1), self.y, self.z),
            Self::new(self.x, self.y + I32F32::from(1), self.z),
            Self::new(self.x, self.y - I32F32::from(1), self.z),
        ]
    }

    fn get_valid_neighbors(&self, obstacles: &Obstacles) -> Vec<Self> {
        let mut neighbors = Vec::new();
        for adjacent in self.get_adjacent().iter() {
            if !obstacles.contains(*adjacent) {
                let below = Self::new(adjacent.x, adjacent.y, adjacent.z - I32F32::from(1));
                if obstacles.contains(below) {
                    neighbors.push(*adjacent);
                } else {
                    neighbors.push(below);
                }
            } else {
                let above_self = Self::new(self.x, self.y, self.z + I32F32::from(1));
                let above_adjacent =
                    Self::new(adjacent.x, adjacent.y, adjacent.z + I32F32::from(1));
                if !obstacles.contains(above_self) && !obstacles.contains(above_adjacent) {
                    neighbors.push(above_adjacent);
                }
            }
        }
        neighbors
    }

    fn get_distance_from(&self, other: &Self) -> I32F32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct FrontierState {
    position: Position,
    cost: I32F32,
}

impl FrontierState {
    fn new(position: Position, cost: I32F32) -> Self {
        Self { position, cost }
    }
}

impl Ord for FrontierState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.x.cmp(&other.position.x))
            .then_with(|| self.position.y.cmp(&other.position.y))
            .then_with(|| self.position.z.cmp(&other.position.z))
    }
}

impl PartialOrd for FrontierState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

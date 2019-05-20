use crate::components::{ActionMove, Location, LocationInfo};
use specs::{Join, ReadStorage, System, WriteStorage};
use std::cmp::{Ord, Ordering};
use std::collections::{BinaryHeap, HashMap};

pub struct PathfindSystem;

impl<'a> System<'a> for PathfindSystem {
    type SystemData = (WriteStorage<'a, ActionMove>, ReadStorage<'a, LocationInfo>);

    fn run(&mut self, (mut action_move_data, location_data): Self::SystemData) {
        let obstacles = (&location_data, !&action_move_data)
            .join()
            .map(|(location, _)| (location.location, location.is_walkable))
            .collect::<HashMap<Location, bool>>();

        for (action_move, location) in (&mut action_move_data, &location_data)
            .join()
            .filter(|(action_move, _)| action_move.should_pathfind)
        {
            action_move.should_pathfind = false;
            action_move.path.clear();

            let mut frontier = BinaryHeap::new();
            let mut came_from = HashMap::new();
            let mut cost_so_far = HashMap::new();
            frontier.push(FrontierNode::new(location.location, 0));
            came_from.insert(location.location, None);
            cost_so_far.insert(location.location, 0);

            while let Some(visiting) = frontier.pop() {
                if visiting.location == action_move.goal {
                    break;
                }

                for (neighbor, cost) in visiting.get_neighbors(&obstacles) {
                    let new_cost = cost_so_far[&visiting.location] + cost;
                    if !cost_so_far.contains_key(&neighbor) || new_cost < cost_so_far[&neighbor] {
                        cost_so_far.insert(neighbor, new_cost);
                        let priority = new_cost
                            + (neighbor.x - action_move.goal.x).abs() as u32
                            + (neighbor.y - action_move.goal.y).abs() as u32
                            + (neighbor.z - action_move.goal.z).abs() as u32 * 2;
                        frontier.push(FrontierNode::new(neighbor, priority));
                        came_from.insert(neighbor, Some(visiting.location));
                    }
                }
            }

            let mut current = action_move.goal;
            while current != location.location {
                action_move.path.push(current);
                if let Some(new_current) = came_from.remove(&current) {
                    current = new_current.unwrap();
                } else {
                    break;
                }
            }
            action_move.path.reverse();
        }
    }
}

#[derive(PartialEq, Eq)]
struct FrontierNode {
    location: Location,
    cost: u32,
}

impl FrontierNode {
    fn new(location: Location, cost: u32) -> Self {
        Self { location, cost }
    }

    fn get_neighbors(&self, obstacles: &HashMap<Location, bool>) -> Vec<(Location, u32)> {
        let mut neighbors = Vec::new();
        for offset in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let adjacent = Location::new(
                self.location.x + offset.0,
                self.location.y + offset.1,
                self.location.z,
            );
            if obstacles.contains_key(&adjacent) {
                let above_self =
                    Location::new(self.location.x, self.location.y, self.location.z + 1);
                let above_adjacent = Location::new(adjacent.x, adjacent.y, adjacent.z + 1);
                if !obstacles.contains_key(&above_self)
                    && !obstacles.contains_key(&above_adjacent)
                    && obstacles[&adjacent]
                {
                    neighbors.push((above_adjacent, 2));
                }
            } else {
                let below_adjacent = Location::new(adjacent.x, adjacent.y, adjacent.z - 1);
                let below_below_adjacent = Location::new(adjacent.x, adjacent.y, adjacent.z - 2);
                if obstacles.contains_key(&below_adjacent) {
                    if obstacles[&below_adjacent] {
                        neighbors.push((adjacent, 1));
                    }
                } else if obstacles.get(&below_below_adjacent) == Some(&true) {
                    neighbors.push((below_adjacent, 2));
                }
            }
        }
        neighbors
    }
}

impl Ord for FrontierNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.location.cmp(&other.location))
    }
}

impl PartialOrd for FrontierNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

use crate::components::{ActionMoveTowards, MovementInfo, Position};
use microprofile::scope;
use rayon::iter::ParallelIterator;
use specs::{Join, ParJoin, ReadStorage, System, WriteStorage};
use std::collections::{BinaryHeap, HashMap, HashSet};

pub struct PathfindingSystem;

impl<'a> System<'a> for PathfindingSystem {
    type SystemData = (
        WriteStorage<'a, ActionMoveTowards>,
        ReadStorage<'a, MovementInfo>,
        ReadStorage<'a, Position>,
    );

    fn run(
        &mut self,
        (mut action_move_towards_data, movement_info_data, position_data): Self::SystemData,
    ) {
        microprofile::scope!("systems", "pathfinding");

        let obstacles = (&position_data).join().collect::<HashSet<&Position>>();
        (
            &mut action_move_towards_data,
            &movement_info_data,
            &position_data,
        )
            .par_join()
            .for_each(|(action_move_towards, movement_info, position)| {
                if let Some(target) = action_move_towards
                    .target
                    .get_adjacent()
                    .iter()
                    .find(|position| !obstacles.contains(position))
                {
                    // TODO: A*
                }
            });
    }
}

impl Position {
    fn get_adjacent(&self) -> [Position; 4] {
        [
            Position::new(self.x + 1, self.y, self.z),
            Position::new(self.x - 1, self.y, self.z),
            Position::new(self.x, self.y, self.z + 1),
            Position::new(self.x, self.y, self.z - 1),
        ]
    }

    fn get_valid_neighbors(&self, obstacles: &HashSet<&Position>) -> Vec<Position> {
        let mut neighbors = Vec::new();
        for adjacent in self.get_adjacent().iter() {
            if obstacles.contains(adjacent) {
                let above = Position::new(adjacent.x, adjacent.z, adjacent.y + 1);
                if !obstacles.contains(&above) {
                    neighbors.push(above);
                }
            } else {
                let under = Position::new(adjacent.x, adjacent.z, adjacent.y - 1);
                if obstacles.contains(&under) {
                    neighbors.push(*adjacent);
                } else {
                    neighbors.push(under);
                }
            }
        }
        neighbors
    }
}

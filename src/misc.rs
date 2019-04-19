use crate::components::Position;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Obstacles {
    obstacles: HashSet<Position>,
}

impl Obstacles {
    pub fn new() -> Self {
        Self {
            obstacles: HashSet::new(),
        }
    }

    pub fn contains(&self, position: Position) -> bool {
        for obstacle in &self.obstacles {
            if position.floor() == obstacle.floor()
                || position.floor() == obstacle.ceil()
                || position.ceil() == obstacle.floor()
                || position.ceil() == obstacle.ceil()
            {
                return true;
            }
        }
        false
    }

    pub fn insert(&mut self, position: Position) {
        self.obstacles.insert(position);
    }

    pub fn remove(&mut self, position: Position) {
        self.obstacles.remove(&position);
    }
}

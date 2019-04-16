use crate::components::Position;
use std::collections::HashSet;

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
        self.obstacles.contains(&position.floor()) || self.obstacles.contains(&position.ceil())
    }

    pub fn insert(&mut self, position: Position) {
        self.obstacles.insert(position.floor());
        self.obstacles.insert(position.ceil());
    }

    pub fn remove(&mut self, position: Position) {
        self.obstacles.insert(position.floor());
        self.obstacles.insert(position.ceil());
    }
}

use specs::storage::{BTreeStorage, DenseVecStorage, VecStorage};
use specs::Component;
use specs_derive::Component;

#[derive(Component, Copy, Clone, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Component, Clone, Debug)]
#[storage(DenseVecStorage)]
pub struct Movement {
    pub target: Option<Position>,
    pub path: Vec<Position>,
    pub move_speed: u32,
}

#[derive(Component, Copy, Clone, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub name: &'static str,
}

#[derive(Component, Copy, Clone, Debug)]
#[storage(BTreeStorage)]
pub struct Elf;

#[derive(Component, Copy, Clone, Debug)]
#[storage(BTreeStorage)]
pub struct Tree {
    pub durability: u32,
}

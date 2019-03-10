use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Movement {
    pub path: Vec<Position>,
    pub move_speed: u32,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Sprite {
    pub name: &'static str,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Elf;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Tree {
    pub durability: u32,
}

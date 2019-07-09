use specs::storage::VecStorage;
use specs::Component;
use specs_derive::Component;

#[derive(Component, Copy, Clone, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Copy, Clone, Debug)]
#[storage(VecStorage)]
pub struct Texture {
    pub atlas_index: u32,
}

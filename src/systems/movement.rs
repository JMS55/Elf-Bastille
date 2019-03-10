use specs::System;

pub struct MovementSystem;
impl<'a> System<'a> for MovementSystem {
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData) {}
}

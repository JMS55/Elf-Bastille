use specs::System;

pub struct ElfSystem;
impl<'a> System<'a> for ElfSystem {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {}
}

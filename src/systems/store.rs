use specs::System;

pub struct StoreSystem;

impl<'a> System<'a> for StoreSystem {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {
        unimplemented!("TODO");
        /*
            Questions: How to handle storage reservation, especially in the context of errors

            Panic if destination has no Inventory component
            Check if destination is still alive. Finish action if not.
            Check if destination has room. Finish action if not.
            Remove entity if it hasn't yet (LocationInfo if on ground, or maybe from storage if this handles taking out as well?)
            Increase time passed
            If enough time passed, add to destination
        */
    }
}

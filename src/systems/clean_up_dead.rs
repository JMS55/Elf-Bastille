use crate::components::{Container, ContainerChild, MarkedForDeath, PhysicalProperties};
use microprofile::scope;
use specs::{Entities, Join, ReadStorage, System, WriteStorage};

// TODO: What should happen when Containers die?
pub struct CleanUpDeadSystem;

impl<'a> System<'a> for CleanUpDeadSystem {
    type SystemData = (
        ReadStorage<'a, MarkedForDeath>,
        ReadStorage<'a, ContainerChild>,
        ReadStorage<'a, PhysicalProperties>,
        WriteStorage<'a, Container>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
            marked_for_death_data,
            container_child_data,
            physical_properties_data,
            mut container_data,
            entities,
        ): Self::SystemData,
    ) {
        microprofile::scope!("systems", "clean_up_dead");

        for (_, entity) in (&marked_for_death_data, &entities).join() {
            if let Some(container_child) = container_child_data.get(entity) {
                let parent_container = container_data.get_mut(container_child.parent).expect(
                    "Parent Entity of ContainerChild did not have Container component during CleanUpDeadSystem",
                );
                let physical_properties = physical_properties_data.get(entity).expect(
                    "ContainerChild did not have PhysicalProperties during CleanUpDeadSystem",
                );
                parent_container.stored_volume -= physical_properties.volume;
                parent_container.stored_weight -= physical_properties.weight;
                let index = parent_container
                    .entities
                    .iter()
                    .position(|e| e == &entity)
                    .expect("Parent Container did not have child entity during CleanUpDeadSystem");
                parent_container.entities.remove(index);
            }

            entities
                .delete(entity)
                .expect("Failed to delete Entity in CleanUpDeadSystem");
        }
    }
}

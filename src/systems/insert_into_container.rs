use crate::components::{ActionInsertIntoContainer, Container, ContainerChild, PhysicalProperties};
use microprofile::scope;
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage};

// TODO: Redo this system. First remove from source container, then add to target container
pub struct InsertIntoContainerSystem;

impl<'a> System<'a> for InsertIntoContainerSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, ActionInsertIntoContainer>,
        WriteStorage<'a, Container>,
        ReadStorage<'a, PhysicalProperties>,
        WriteStorage<'a, ContainerChild>,
    );

    fn run(
        &mut self,
        (
            entities,
            lazy_world,
            action_insert_into_container_data,
            mut container_data,
            physical_properties_data,
            mut container_child_data,
        ): Self::SystemData,
    ) {
        microprofile::scope!("systems", "insert_into_container");

        for (entity, action_insert_into_container) in
            (&entities, &action_insert_into_container_data).join()
        {
            let mut passes_check = true;
            let container = container_data
                .get_mut(action_insert_into_container.container)
                .expect("TODO: ERRRRRRR MSG");
            let entity_properties = physical_properties_data
                .get(action_insert_into_container.entity)
                .expect("TODO: ERR MSG");
            if container.stored_volume + entity_properties.volume > container.volume_limit {
                passes_check = false;
            }
            if let Some(weight_limit) = container.weight_limit {
                if container.stored_weight + entity_properties.weight > weight_limit {
                    passes_check = false;
                }
            }
            if passes_check {
                container_child_data
                    .insert(
                        action_insert_into_container.entity,
                        ContainerChild {
                            parent: action_insert_into_container.container,
                        },
                    )
                    .expect("TODO: AHHHHH");
                container.stored_volume += entity_properties.volume;
                container.stored_weight += entity_properties.weight;
                container.entities.push(action_insert_into_container.entity);
                lazy_world.remove::<ActionInsertIntoContainer>(entity);
            }
        }
    }
}

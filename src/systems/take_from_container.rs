use crate::components::{
    ActionTakeFromContainer, Container, ContainerChild, EntityType, PhysicalProperties,
};
use microprofile::scope;
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage};

pub struct TakeFromContainerSystem;

impl<'a> System<'a> for TakeFromContainerSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, ActionTakeFromContainer>,
        WriteStorage<'a, Container>,
        WriteStorage<'a, ContainerChild>,
        ReadStorage<'a, EntityType>,
        ReadStorage<'a, PhysicalProperties>,
    );

    fn run(
        &mut self,
        (
            entities,
            lazy_world,
            action_take_from_container_data,
            mut container_data,
            mut container_child_data,
            entity_type_data,
            physical_properties_data,
        ): Self::SystemData,
    ) {
        microprofile::scope!("systems", "take_from_container");

        // TODO: Don't remove from input storage unless output storage can fit it
        for (self_entity, action_take_from_container) in
            (&entities, &action_take_from_container_data).join()
        {
            let container = container_data
                .get_mut(action_take_from_container.container)
                .expect("TODO: ERRRRRRR");
            for (i, container_child_entity) in container.entities.iter_mut().enumerate() {
                if let Some(entity_type) = entity_type_data.get(*container_child_entity) {
                    if entity_type == &action_take_from_container.entity_type {
                        lazy_world.remove::<ContainerChild>(*container_child_entity);
                        container.entities.remove(i);
                        let self_container = container_data
                            .get_mut(self_entity)
                            .expect("TODO: ERRRRRRRAAAAAHHHH");
                        let entity_properties = physical_properties_data
                            .get(*container_child_entity)
                            .expect("TODO: MORE ERR MSGGG");
                        self_container.stored_volume -= entity_properties.volume;
                        self_container.stored_weight -= entity_properties.weight;
                        self_container.entities.push(*container_child_entity);
                        break;
                    }
                }
            }
        }
    }
}

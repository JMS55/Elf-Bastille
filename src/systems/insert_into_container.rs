use crate::components::{
    ActionInsertIntoContainer, Container, ContainerChild, PhysicalProperties, Position,
};
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage};

pub struct InsertIntoContainerSystem;

impl<'a> System<'a> for InsertIntoContainerSystem {
    type SystemData = (
        ReadStorage<'a, ActionInsertIntoContainer>,
        WriteStorage<'a, Container>,
        WriteStorage<'a, ContainerChild>,
        ReadStorage<'a, PhysicalProperties>,
        WriteStorage<'a, Position>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            action_insert_into_container_data,
            mut container_data,
            mut container_child_data,
            physical_properties_data,
            mut position_data,
            entities,
            lazy_world,
        ): Self::SystemData,
    ) {
        for (action_insert_into_container, entity) in
            (&action_insert_into_container_data, &entities).join()
        {
            let target_container = container_data
                .get_mut(action_insert_into_container.target_container)
                .expect("Target container of ActionInsertIntoContainer had no Container component");
            let entity_physical_properties = physical_properties_data
                .get(action_insert_into_container.entity)
                .expect("Entity of ActionInsertIntoContainer had no PhysicalProperties component");

            let mut passes_checks = true;
            if target_container.stored_volume + entity_physical_properties.volume
                > target_container.volume_limit
            {
                passes_checks = false;
            }
            if let Some(weight_limit) = target_container.weight_limit {
                if target_container.stored_weight + entity_physical_properties.weight > weight_limit
                {
                    passes_checks = false;
                }
            }

            if passes_checks {
                target_container.stored_volume += entity_physical_properties.volume;
                target_container.stored_weight += entity_physical_properties.weight;
                target_container
                    .children
                    .push(action_insert_into_container.entity);

                if position_data
                    .get(action_insert_into_container.entity)
                    .is_some()
                {
                    position_data.remove(action_insert_into_container.entity);
                }

                if let Some(container_child) =
                    container_child_data.get(action_insert_into_container.entity)
                {
                    let parent_container = container_data
                        .get_mut(container_child.parent_container)
                        .expect("ContainerChild parent_container had no Container component");
                    parent_container.stored_volume -= entity_physical_properties.volume;
                    parent_container.stored_weight -= entity_physical_properties.weight;
                    let index = parent_container
                        .children
                        .iter()
                        .position(|child| child == &action_insert_into_container.entity)
                        .expect("ContainerChild parent_container did not contain child entity");
                    parent_container.children.remove(index);
                }

                container_child_data
                    .insert(
                        action_insert_into_container.entity,
                        ContainerChild {
                            parent_container: action_insert_into_container.target_container,
                        },
                    )
                    .expect("Adding ContainerChild component failed");
            }

            lazy_world.remove::<ActionInsertIntoContainer>(entity);
        }
    }
}

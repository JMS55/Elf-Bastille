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
            entity_type_data,
            physical_properties_data,
        ): Self::SystemData,
    ) {
        microprofile::scope!("systems", "take_from_container");

        for (self_entity, action_take_from_container) in
            (&entities, &action_take_from_container_data).join()
        {
            let mut output_container = container_data
                .get(self_entity)
                .expect("Could not get output Container during TakeFromContainerSystem")
                .clone();
            let input_container = container_data
                .get_mut(action_take_from_container.container)
                .expect("Could not get input Container during TakeFromContainerSystem");
            let mut result = None;
            for (index, container_child_entity) in input_container.entities.iter().enumerate() {
                if let Some(entity_type) = entity_type_data.get(*container_child_entity) {
                    if entity_type == &action_take_from_container.entity_type {
                        result = Some((index, *container_child_entity));
                        break;
                    }
                }
            }
            if let Some((index, entity)) = result {
                let entity_properties = physical_properties_data.get(entity).expect(
                    "Could not get entity PhysicalProperties during TakeFromContainerSystem",
                );
                let mut passes_check = true;
                if output_container.stored_volume + entity_properties.volume
                    > output_container.volume_limit
                {
                    passes_check = false;
                }
                if let Some(weight_limit) = output_container.weight_limit {
                    if output_container.stored_weight + entity_properties.weight > weight_limit {
                        passes_check = false;
                    }
                }
                if passes_check {
                    lazy_world.remove::<ContainerChild>(entity);
                    input_container.stored_volume -= entity_properties.volume;
                    input_container.stored_weight -= entity_properties.weight;
                    input_container.entities.remove(index);
                    output_container.stored_volume += entity_properties.volume;
                    output_container.stored_weight += entity_properties.weight;
                    output_container.entities.push(entity);
                    container_data
                        .insert(self_entity, output_container)
                        .expect("Could not insert output Container component during TakeFromContainerSystem");
                }
            }
            lazy_world.remove::<ActionTakeFromContainer>(self_entity);
        }
    }
}

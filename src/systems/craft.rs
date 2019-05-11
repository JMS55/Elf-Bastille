use crate::components::{ActionCraft, Container, Damageable, EntityType, PhysicalProperties};
use specs::{
    Builder, Entities, Entity, Join, LazyUpdate, Read, ReadStorage, System, World, WriteStorage,
};

pub struct CraftSystem;

impl<'a> System<'a> for CraftSystem {
    type SystemData = (
        ReadStorage<'a, ActionCraft>,
        WriteStorage<'a, Container>,
        ReadStorage<'a, EntityType>,
        ReadStorage<'a, PhysicalProperties>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            action_craft_data,
            mut container_data,
            entity_type_data,
            physical_properties_data,
            entities,
            lazy_world,
        ): Self::SystemData,
    ) {
        for (action_craft, container, entity) in
            (&action_craft_data, &mut container_data, &entities).join()
        {
            let mut input_types_left = action_craft.type_to_craft.get_recipe();
            let mut input_indices = Vec::new();
            for (child_index, child_entity_type) in container
                .children
                .iter()
                .map(|child_entity| {
                    entity_type_data
                        .get(*child_entity)
                        .expect("Container child had no EntityType component")
                })
                .enumerate()
            {
                if let Some(input_type_index) = input_types_left
                    .iter()
                    .position(|input_entity_type| child_entity_type == input_entity_type)
                {
                    input_types_left.remove(input_type_index);
                    input_indices.push(child_index);
                }
            }

            if input_types_left.is_empty() {
                let mut total_input_volume = 0;
                let mut total_input_weight = 0;
                for input_index in &input_indices {
                    let input_physical_properties = physical_properties_data
                        .get(container.children[*input_index])
                        .expect("Container child had no PhysicalProperties component");
                    total_input_volume += input_physical_properties.volume;
                    total_input_weight += input_physical_properties.weight;
                }

                let mut passes_checks = true;
                if container.stored_volume - total_input_volume
                    + action_craft.type_to_craft.get_volume()
                    > container.volume_limit
                {
                    passes_checks = false;
                }
                if let Some(weight_limit) = container.weight_limit {
                    if container.stored_weight - total_input_weight
                        + action_craft.type_to_craft.get_weight()
                        > weight_limit
                    {
                        passes_checks = false;
                    }
                }

                if passes_checks {
                    for input_index in input_indices {
                        container.stored_volume -= total_input_volume;
                        container.stored_weight -= total_input_weight;
                        container.children.remove(input_index);
                    }

                    container.stored_volume += action_craft.type_to_craft.get_volume();
                    container.stored_weight += action_craft.type_to_craft.get_weight();
                    // let entity = action_craft.type_to_craft.create_entity(world);
                    // container.children.push(entity);
                    unimplemented!("TODO");
                }
            }

            lazy_world.remove::<ActionCraft>(entity);
        }
    }
}

pub enum CraftableEntityType {
    Cup,
    Axe,
}

impl CraftableEntityType {
    pub fn create_entity(&self, world: &mut World) -> Entity {
        match self {
            CraftableEntityType::Cup => world
                .create_entity()
                .with(PhysicalProperties {
                    volume: self.get_volume(),
                    weight: self.get_weight(),
                })
                .build(),
            CraftableEntityType::Axe => world
                .create_entity()
                .with(PhysicalProperties {
                    volume: self.get_volume(),
                    weight: self.get_weight(),
                })
                .with(Damageable {
                    durability: 10,
                    on_break_callback: None,
                })
                .build(),
        }
    }

    pub fn get_recipe(&self) -> Vec<EntityType> {
        match self {
            CraftableEntityType::Cup => vec![EntityType::Wood, EntityType::Wood],
            CraftableEntityType::Axe => vec![
                EntityType::Wood,
                EntityType::Wood,
                EntityType::Wood,
                EntityType::Wood,
            ],
        }
    }

    pub fn get_volume(&self) -> u32 {
        match self {
            CraftableEntityType::Cup => 10,
            CraftableEntityType::Axe => 30,
        }
    }

    pub fn get_weight(&self) -> u32 {
        match self {
            CraftableEntityType::Cup => 5,
            CraftableEntityType::Axe => 35,
        }
    }
}

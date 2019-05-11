use components::*;
use fixed::types::I32F32;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::{ContextBuilder, Event, EventsLoop, WindowBuilder, WindowEvent};
use glium::Display;
use specs::{Builder, Entity, Join, LazyUpdate, RunNow, World};
use std::time::{Duration, Instant};
use systems::*;

mod components;
mod systems;

pub const DELTA_TIME: Duration = Duration::from_nanos(16700000);
pub const TREE_STAGES: u32 = 5;
pub const TIME_PER_TREE_STAGE: Duration = Duration::from_secs(7);
pub const WORLD_SIZE: f32 = 21.0;
pub const TEXTURE_SIZE: f32 = 32.0;
pub const NUMBER_OF_TEXTURES: f32 = 10.0;

fn main() {
    let mut event_loop = EventsLoop::new();
    let window = WindowBuilder::new()
        .with_dimensions(LogicalSize::new(
            (WORLD_SIZE * TEXTURE_SIZE) as f64,
            (WORLD_SIZE * TEXTURE_SIZE) as f64,
        ))
        .with_resizable(false)
        .with_title("Elf Bastille");
    let context = ContextBuilder::new()
        .with_vsync(true)
        .with_srgb(true)
        .with_depth_buffer(24);;
    let display = Display::new(window, context, &event_loop).expect("Could not create Display");

    let mut world = World::new();
    world.register::<ActionAttack>();
    world.register::<ActionCraft>();
    world.register::<ActionInsertIntoContainer>();
    world.register::<ActionMoveTo>();
    world.register::<AI>();
    world.register::<Container>();
    world.register::<ContainerChild>();
    world.register::<Damageable>();
    world.register::<Displayable>();
    world.register::<EntityType>();
    world.register::<MoveSpeed>();
    world.register::<PhysicalProperties>();
    world.register::<Position>();
    world.register::<TimeTracker>();
    world.register::<Walkable>();

    // let mut create_trees_system = CreateTreesSystem;
    let mut ai_system = AISystem;
    let mut insert_into_container_system = InsertIntoContainerSystem;
    // let mut take_from_container_system = TakeFromContainerSystem;
    let mut craft_system = CraftSystem;
    // let mut attack_system = AttackSystem;
    // let mut pathfinding_system = PathfindingSystem;
    // let mut movement_system = MovementSystem;
    let mut time_tracking_system = TimeTrackingSystem;
    // let mut clean_up_broken_system = CleanUpBrokenSystem;
    let mut render_system = RenderSystem::new(display);

    // Create entities
    {
        // for x in -10..=10 {
        //     for y in -10..=10 {
        //         world
        //             .create_entity()
        //             .with(EntityType::Dirt)
        //             .with(Displayable {
        //                 texture_atlas_index: 9,
        //             })
        //             .with(WorldLocation {
        //                 position: Position {
        //                     x: I32F32::from(x),
        //                     y: I32F32::from(y),
        //                     z: I32F32::from(0),
        //                 },
        //                 is_walkable: true,
        //             })
        //             .build();
        //     }
        // }

        // // Tree harvesting elf
        // fn tree_harvest_ai(self_entity: Entity, lazy_world: &LazyUpdate) {
        //     lazy_world.exec(move |world| {
        //         let container_data = world.read_storage::<Container>();
        //         let world_location_data = world.read_storage::<WorldLocation>();
        //         let entity_type_data = world.read_storage::<EntityType>();
        //         let time_tracker_data = world.read_storage::<TimeTracker>();
        //         let mut action_attack_data = world.write_storage::<ActionAttack>();
        //         let mut action_move_to_data = world.write_storage::<ActionMoveTo>();
        //         let mut action_take_from_container_data = world.write_storage::<ActionTakeFromContainer>();
        //         let self_container = container_data
        //             .get(self_entity)
        //             .expect("Entity with tree_harvest_ai() did not have a Container component");
        //         let self_position = &world_location_data
        //             .get(self_entity)
        //             .expect("Entity with tree_harvest_ai() did not have a WorldLocation component")
        //             .position;
        //         if let Some(axe_entity) = self_container.children.iter().find(|child| {
        //             entity_type_data
        //                 .get(**child)
        //                 .expect("Entity did not have a EntityType component in tree_harvest_ai()")
        //                 == &EntityType::Axe
        //         }) {
        //             if let Some(tree_entity) = (
        //                 &entity_type_data,
        //                 &time_tracker_data,
        //                 &world_location_data,
        //                 &world.entities(),
        //             )
        //                 .join()
        //                 .find(|(entity_type, time_tracker, world_location, _)| {
        //                     entity_type == &&EntityType::Tree
        //                         && time_tracker.time_passed
        //                             >= (TREE_STAGES - 1) * TIME_PER_TREE_STAGE
        //                         && world_location.position.is_adjacent_to(self_position)
        //                 })
        //                 .map(|(_, _, _, entity)| entity)
        //             {
        //                 action_attack_data
        //                     .insert(
        //                         self_entity,
        //                         ActionAttack {
        //                             weapon: axe_entity.clone(),
        //                             target_entity: tree_entity,
        //                         },
        //                     )
        //                     .expect("Could not insert ActionAttack component in tree_harvest_ai()");
        //             } else {
        //                 if let Some(tree_position) = (
        //                     &entity_type_data,
        //                     &time_tracker_data,
        //                     &world_location_data,
        //                 )
        //                     .join()
        //                     .find(|(entity_type, time_tracker, _)| {
        //                         entity_type == &&EntityType::Tree
        //                         && time_tracker.time_passed  >= (TREE_STAGES - 1) * TIME_PER_TREE_STAGE
        //                     }).map(|(_, _, world_location)| world_location.position.clone())
        //                 {
        //                      action_move_to_data
        //                     .insert(
        //                         self_entity,
        //                       ActionMoveTo::new(tree_position)
        //                     )
        //                     .expect("Could not insert ActionMoveTo(tree_position) component in tree_harvest_ai()");
        //                 }
        //             }
        //         } else {
        //             let axe_crate_position = Position {
        //                 x: I32F32::from(-7),
        //                 y: I32F32::from(0),
        //                 z: I32F32::from(1),
        //             };
        //             if self_position.is_adjacent_to(&axe_crate_position) {
        //                 if let Some(axe_crate_entity) = (&world_location_data,   &world.entities()).join().find(|(world_location, _)| world_location.position == axe_crate_position).map(|(_, entity)| entity) {
        //                                             action_take_from_container_data.insert(self_entity, ActionTakeFromContainer{entity_type: EntityType::Axe, source_container: axe_crate_entity}).expect("Could not insert ActionTakeFromContainer component in tree_harvest_ai()");
        //                 };
        //             } else {
        //                  action_move_to_data
        //                     .insert(
        //                         self_entity,
        //                       ActionMoveTo::new(axe_crate_position)
        //                     )
        //                     .expect("Could not insert ActionMoveTo(axe_crate_position) component in tree_harvest_ai()");
        //             }
        //         }
        //     });
        // }
        // world
        //     .create_entity()
        //     .with(EntityType::Elf)
        //     .with(Displayable {
        //         texture_atlas_index: 1,
        //     })
        //     .with(AI {
        //         set_action_callback: tree_harvest_ai,
        //     })
        //     .with(Container::new(40, Some(20)))
        //     .with(MoveSpeed {
        //         speed: I32F32::from_float(1.0 / 60.0),
        //     })
        //     .with(WorldLocation {
        //         position: Position {
        //             x: I32F32::from(3),
        //             y: I32F32::from(-4),
        //             z: I32F32::from(1),
        //         },
        //         is_walkable: false,
        //     })
        //     .build();

        // fn pick_up_wood_ai(self_entity: Entity, lazy_world: &LazyUpdate) {
        //     lazy_world.exec(move |world| {
        //         let container_child_data = world.read_storage::<ContainerChild>();
        //         let entity_type_data = world.read_storage::<EntityType>();
        //         let world_location_data = world.read_storage::<WorldLocation>();
        //         let mut action_move_to_data = world.write_storage::<ActionMoveTo>();
        //         let mut action_insert_into_container_data =
        //             world.write_storage::<ActionInsertIntoContainer>();
        //         if (&container_child_data, &entity_type_data)
        //             .join()
        //             .filter(|(container_child, entity_type)| {
        //                 container_child.parent_container == self_entity
        //                     && entity_type == &&EntityType::Wood
        //             })
        //             .count()
        //             >= 4
        //         {
        //             let self_position = &world_location_data
        //                 .get(self_entity)
        //                 .expect(
        //                     "Entity with pick_up_wood_ai() did not have a WorldLocation component",
        //                 )
        //                 .position;
        //             let wood_crate_position = Position {
        //                 x: I32F32::from(-6),
        //                 y: I32F32::from(1),
        //                 z: I32F32::from(1),
        //             };
        //             if self_position.is_adjacent_to(&wood_crate_position) {
        //                 if let Some(wood_crate_entity) = (&world_location_data, &world.entities()).join().find(|(world_location, _)| world_location.position == wood_crate_position).map(|(_, entity)| entity) {
        //                     action_insert_into_container_data
        //                     .insert(
        //                         self_entity, ActionInsertIntoContainer {
        //                             entity_type: EntityType::Wood,
        //                             target_container: wood_crate_entity,
        //                         }
        //                     )
        //                     .expect("Could not insert ActionInsertIntoContainer component in pick_up_wood_ai()");
        //                 };
        //             } else {
        //                action_move_to_data.insert(self_entity, ActionMoveTo {
        //                    path: Vec::new(), target: wood_crate_position
        //                }).expect("Could not insert ActionMoveTo component in pick_up_wood_ai()");
        //             }
        //         } else {
        //             /*
        //             TODO:
        //                 if next_to_wood {
        //                     pick_up_wood()
        //                 } else {
        //                     path_to_nearest_wood()
        //                 }
        //             */
        //         }
        //     });
        // }
        // world
        //     .create_entity()
        //     .with(EntityType::Elf)
        //     .with(Displayable {
        //         texture_atlas_index: 1,
        //     })
        //     .with(AI {
        //         set_action_callback: pick_up_wood_ai,
        //     })
        //     .with(Container::new(40, Some(20)))
        //     .with(MoveSpeed {
        //         speed: I32F32::from_float(1.0 / 60.0),
        //     })
        //     .with(WorldLocation {
        //         position: Position {
        //             x: I32F32::from(5),
        //             y: I32F32::from(-4),
        //             z: I32F32::from(1),
        //         },
        //         is_walkable: false,
        //     })
        //     .build();

        // world
        //     .create_entity()
        //     .with(EntityType::Elf)
        //     .with(Displayable {
        //         texture_atlas_index: 1,
        //     })
        //     // .with(AI {
        //     //     set_action_callback: craft_ai,
        //     // })
        //     .with(Container::new(40, Some(20)))
        //     .with(MoveSpeed {
        //         speed: I32F32::from_float(1.0 / 60.0),
        //     })
        //     .with(WorldLocation {
        //         position: Position {
        //             x: I32F32::from(-7),
        //             y: I32F32::from(1),
        //             z: I32F32::from(1),
        //         },
        //         is_walkable: false,
        //     })
        //     .build();

        // world
        //     .create_entity()
        //     .with(EntityType::Crate)
        //     .with(Displayable {
        //         texture_atlas_index: 0,
        //     })
        //     .with(Container::new(100, None))
        //     .with(WorldLocation {
        //         position: Position {
        //             x: I32F32::from(-7),
        //             y: I32F32::from(0),
        //             z: I32F32::from(1),
        //         },
        //         is_walkable: false,
        //     })
        //     .build();

        // world
        //     .create_entity()
        //     .with(EntityType::Crate)
        //     .with(Displayable {
        //         texture_atlas_index: 0,
        //     })
        //     .with(Container::new(100, None))
        //     .with(WorldLocation {
        //         position: Position {
        //             x: I32F32::from(-7),
        //             y: I32F32::from(2),
        //             z: I32F32::from(1),
        //         },
        //         is_walkable: false,
        //     })
        //     .build();

        // world
        //     .create_entity()
        //     .with(EntityType::Crate)
        //     .with(Displayable {
        //         texture_atlas_index: 0,
        //     })
        //     .with(Container::new(100, None))
        //     .with(WorldLocation {
        //         position: Position {
        //             x: I32F32::from(-6),
        //             y: I32F32::from(1),
        //             z: I32F32::from(1),
        //         },
        //         is_walkable: false,
        //     })
        //     .build();
    }

    let mut current_time = Instant::now();
    let mut accumulator = Duration::from_nanos(0);
    let mut should_close = false;
    while !should_close {
        event_loop.poll_events(|event| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => should_close = true,
                _ => {}
            },
            _ => {}
        });

        let new_time = Instant::now();
        accumulator += new_time - current_time;
        current_time = new_time;
        while accumulator >= DELTA_TIME {
            // create_trees_system.run_now(&world.res);
            world.maintain();
            ai_system.run_now(&world.res);
            world.maintain();
            insert_into_container_system.run_now(&world.res);
            world.maintain();
            craft_system.run_now(&world.res);
            world.maintain();
            // attack_system.run_now(&world.res);
            world.maintain();
            // pathfind_system.run_now(&world.res);
            world.maintain();
            // move_system.run_now(&world.res);
            world.maintain();
            time_tracking_system.run_now(&world.res);
            world.maintain();
            // clean_up_broken_system.run_now(&world.res);
            world.maintain();
            accumulator -= DELTA_TIME;
        }

        render_system.run_now(&world.res);
    }
}

use components::*;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::{ContextBuilder, Event, EventsLoop, WindowBuilder, WindowEvent};
use glium::Display;
use specs::{RunNow, World};
use std::time::{Duration, Instant};
use systems::*;

mod components;
mod misc;
mod systems;

pub const DELTA_TIME: Duration = Duration::from_nanos(16700000);
pub const WORLD_SIZE: f32 = 11.0;
pub const TEXTURE_SIZE: f32 = 48.0;
pub const NUMBER_OF_TEXTURES: f32 = 7.0;

fn main() {
    microprofile::init!();
    microprofile::set_enable_all_groups!(true);

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
    world.register::<ActionMoveTowards>();
    world.register::<ActionTakeFromContainer>();
    world.register::<AI>();
    world.register::<Container>();
    world.register::<ContainerChild>();
    world.register::<Damageable>();
    world.register::<Displayable>();
    world.register::<EntityType>();
    world.register::<Growable>();
    world.register::<Loot>();
    world.register::<MarkedForDeath>();
    world.register::<MovementInfo>();
    world.register::<PhysicalProperties>();
    world.register::<Position>();
    world.register::<Walkable>();

    // TODO: Temporary, remove soon
    {
        use fixed::types::I32F32;
        use specs::world::{Builder, Entity, LazyUpdate};

        fn elf_set_action(self_entity: Entity, lazy_update: &LazyUpdate) {
            lazy_update.insert(
                self_entity,
                ActionMoveTowards {
                    path: Vec::new(),
                    target: Position {
                        x: I32F32::from(3),
                        y: I32F32::from(1),
                        z: I32F32::from(0),
                    },
                },
            );
        }
        world
            .create_entity()
            .with(Position {
                x: I32F32::from(-3),
                y: I32F32::from(1),
                z: I32F32::from(0),
            })
            .with(MovementInfo {
                speed: I32F32::from_float(1.0 / 60.0),
            })
            .with(AI {
                set_action: elf_set_action,
            })
            .with(Displayable {
                texture_atlas_index: 0,
            })
            .build();

        world
            .create_entity()
            .with(Position {
                x: I32F32::from(3),
                y: I32F32::from(1),
                z: I32F32::from(0),
            })
            .with(Displayable {
                texture_atlas_index: 1,
            })
            .build();

        for z in -3..=3 {
            for y in 1..=2 {
                world
                    .create_entity()
                    .with(Position {
                        x: I32F32::from(0),
                        y: I32F32::from(y),
                        z: I32F32::from(z),
                    })
                    .with(Displayable {
                        texture_atlas_index: 2,
                    })
                    .build();
            }
        }

        for x in -5..=5 {
            for z in -5..=5 {
                world
                    .create_entity()
                    .with(Position {
                        x: I32F32::from(x),
                        y: I32F32::from(0),
                        z: I32F32::from(z),
                    })
                    .with(Walkable)
                    .with(Displayable {
                        texture_atlas_index: 6,
                    })
                    .build();
            }
        }
    }

    // TODO: Create systems
    // TODO: Action systems remove components at the end
    let mut ai_system = AISystem;
    let mut insert_into_container_system = InsertIntoContainerSystem;
    let mut take_from_container_system = TakeFromContainerSystem;
    let mut crafting_system = CraftingSystem;
    let mut attack_system = AttackSystem;
    let mut pathfinding_system = PathfindingSystem;
    let mut movement_system = MovementSystem;
    // let mut growth_system = GrowthSystem;
    // let mut loot_system = LootSystem;
    // let mut clean_up_dead_system = CleanUpDeadSystem;
    let mut render_system = RenderSystem::new(display);
    /*
        ✓ AISystem (AI) - Figures out what action to take, and adds the appropriate component
        ✓ InsertIntoContainerSystem (?)
        ✓ TakeFromContainerSystem(?)
        ✓ CraftingSystem (?)
        ✓ AttackSystem (?)
        ✓ PathfindingSystem (?)
        ✓ MovementSystem (?)
        GrowthSystem (Tree, Option<Displayable>) - Adds time passed to growable component, and then changes Displayable of needed things
        LootSystem (Loot)
        CleanUpDeadSystem (MarkedForDeath, Option<ContainerChild>)
        Render (Displayable, Position)
    */

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
            // TODO: Run systems
            ai_system.run_now(&world.res);
            world.maintain();
            insert_into_container_system.run_now(&world.res);
            world.maintain();
            take_from_container_system.run_now(&world.res);
            world.maintain();
            crafting_system.run_now(&world.res);
            world.maintain();
            attack_system.run_now(&world.res);
            // world.maintain(); TODO: IS THIS NEEDED?
            pathfinding_system.run_now(&world.res);
            movement_system.run_now(&world.res);
            // growth_system.run_now(&world.res);
            // loot_system.run_now(&world.res);
            // world.maintain();
            // clean_up_dead_system.run_now(&world.res);
            // world.maintain();
            accumulator -= DELTA_TIME;
        }

        render_system.run_now(&world.res);

        microprofile::flip!();
    }

    microprofile::dump_file_immediately!("profile.html", "");
    microprofile::shutdown!();
}

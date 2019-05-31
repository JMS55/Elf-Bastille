use components::*;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::{ContextBuilder, Event, EventsLoop, WindowBuilder, WindowEvent};
use glium::Display;
use specs::{Builder, RunNow, World};
use std::time::{Duration, Instant};
use systems::*;

mod components;
mod systems;

pub const DELTA_TIME: Duration = Duration::from_nanos(16700000);
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
    // Components //
    world.register::<Texture>();
    world.register::<LocationInfo>();
    world.register::<MovementInfo>();
    world.register::<Inventory>();
    world.register::<StorageInfo>();
    world.register::<IsStored>();
    world.register::<Attackable>();
    world.register::<Weapon>();
    // Entities //
    world.register::<Elf>();
    world.register::<Tree>();
    world.register::<Dirt>();
    // Actions //
    world.register::<ActionMove>();
    world.register::<ActionStore>();
    world.register::<ActionAttack>();

    let mut elf_system = ElfSystem;
    let mut tree_growth_system = TreeGrowthSystem;
    let mut create_trees_system = CreateTreesSystem::new();
    let mut store_system = StoreSystem;
    let mut attack_system = AttackSystem;
    let mut pathfind_system = PathfindSystem;
    let mut movement_system = MovementSystem;
    let mut render_system = RenderSystem::new(display);

    // Test world
    let log_entity = world
        .create_entity()
        .with(Texture { atlas_index: 7 })
        .with(LocationInfo::new(Location::new(2, 2, 1), false))
        .with(StorageInfo::new(10, 25))
        .build();
    let elf_builder = world.create_entity();
    let mut elf = Elf::new();
    elf.queue_action(ActionMove::new(Location::new(-8, 10, 1)));
    elf.queue_action(ActionMove::new(Location::new(2, 1, 1)));
    elf.queue_action(ActionStore::new(
        log_entity,
        elf_builder.entity,
        Duration::from_secs(2),
    ));
    elf.queue_action(ActionMove::new(Location::new(10, 10, 1))); // Should be skipped over
    elf.queue_action(ActionMove::new(Location::new(4, 7, 1)));
    elf_builder
        .with(elf)
        .with(Texture { atlas_index: 1 })
        .with(MovementInfo::new(Duration::from_millis(333)))
        .with(LocationInfo::new(Location::new(0, 0, 1), false))
        .with(Inventory::new(100, 40))
        .build();
    for x in -10..=10 {
        for y in -10..=10 {
            world
                .create_entity()
                .with(Dirt)
                .with(Texture { atlas_index: 9 })
                .with(LocationInfo::new(Location::new(x, y, 0), true))
                .build();
        }
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
            elf_system.run_now(&world.res);
            world.maintain();
            tree_growth_system.run_now(&world.res);
            create_trees_system.run_now(&world.res);
            world.maintain();
            store_system.run_now(&world.res);
            world.maintain();
            attack_system.run_now(&world.res);
            world.maintain();
            pathfind_system.run_now(&world.res);
            movement_system.run_now(&world.res);
            world.maintain();
            accumulator -= DELTA_TIME;
        }

        render_system.run_now(&world.res);
    }
}

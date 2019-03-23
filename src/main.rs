use components::*;
use glium::glutin::{
    dpi::LogicalSize, ContextBuilder, ElementState, Event, EventsLoop, VirtualKeyCode,
    WindowBuilder, WindowEvent,
};
use glium::Display;
use specs::{Builder, RunNow, World};
use std::time::{Duration, Instant};
use systems::*;

mod components;
// mod inspector;
mod systems;

pub const WORLD_WIDTH: u32 = 20;
pub const WORLD_HEIGHT: u32 = 20;
pub const NUMBER_OF_TEXTURES: u32 = 5;
const TEXTURE_SIZE: u32 = 16;
const TEXTURE_SCALE_FACTOR: u32 = 2;

fn main() {
    // Setup profiling
    microprofile::init!();
    microprofile::set_enable_all_groups!(true);

    // Setup windowing
    let mut event_loop = EventsLoop::new();
    let window = WindowBuilder::new()
        .with_dimensions(LogicalSize::new(
            (WORLD_WIDTH * TEXTURE_SIZE * TEXTURE_SCALE_FACTOR) as f64,
            (WORLD_HEIGHT * TEXTURE_SIZE * TEXTURE_SCALE_FACTOR) as f64,
        ))
        .with_resizable(false)
        .with_title("Elf Bastille");
    let context = ContextBuilder::new().with_vsync(true).with_srgb(true);
    let display = Display::new(window, context, &event_loop).expect("Could not create Display");

    // Create the world
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Movement>();
    world.register::<Displayable>();
    world.register::<Elf>();
    world.register::<Tree>();
    world.register::<ItemStorage>();
    world.register::<Item>();

    // Create systems
    let mut item_storage_system = ItemStorageSystem;
    let mut elf_system = ElfSystem;
    let mut pathfinding_system = PathfindingSystem;
    let mut movement_system = MovementSystem;
    let mut render_system = RenderSystem::new(display);

    // Create entities
    {
        world
            .create_entity()
            .with(Elf {})
            .with(Position { x: 0, y: 0 })
            .with(Movement {
                target: None,
                path: Vec::new(),
                move_speed: 1,
            })
            .with(ItemStorage {
                items: Vec::new(),
                volume_limit: 0,
                weight_limit: Some(0),
            })
            .with(Displayable {
                name: "Eve",
                texture_atlas_index: 0,
            })
            .build();
        world
            .create_entity()
            .with(Elf {})
            .with(Position { x: 19, y: 19 })
            .with(Movement {
                target: None,
                path: Vec::new(),
                move_speed: 1,
            })
            .with(ItemStorage {
                items: Vec::new(),
                volume_limit: 0,
                weight_limit: Some(0),
            })
            .with(Displayable {
                name: "Jeff",
                texture_atlas_index: 0,
            })
            .build();
        world
            .create_entity()
            .with(Elf {})
            .with(Position { x: 2, y: 15 })
            .with(Movement {
                target: None,
                path: Vec::new(),
                move_speed: 1,
            })
            .with(ItemStorage {
                items: Vec::new(),
                volume_limit: 0,
                weight_limit: Some(0),
            })
            .with(Displayable {
                name: "Jane",
                texture_atlas_index: 0,
            })
            .build();
        world
            .create_entity()
            .with(Elf {})
            .with(Position { x: 10, y: 14 })
            .with(Movement {
                target: None,
                path: Vec::new(),
                move_speed: 1,
            })
            .with(ItemStorage {
                items: Vec::new(),
                volume_limit: 0,
                weight_limit: Some(0),
            })
            .with(Displayable {
                name: "Alice",
                texture_atlas_index: 0,
            })
            .build();
        world
            .create_entity()
            .with(Elf {})
            .with(Position { x: 10, y: 10 })
            .with(Movement {
                target: None,
                path: Vec::new(),
                move_speed: 1,
            })
            .with(ItemStorage {
                items: Vec::new(),
                volume_limit: 0,
                weight_limit: Some(0),
            })
            .with(Displayable {
                name: "Bob",
                texture_atlas_index: 0,
            })
            .build();
        let item1 = world
            .create_entity()
            .with(Item {
                volume: 10,
                weight: 5,
            })
            .build();
        let item2 = world
            .create_entity()
            .with(Item {
                volume: 20,
                weight: 5,
            })
            .build();
        let item3 = world
            .create_entity()
            .with(ItemStorage {
                items: vec![item2],
                volume_limit: 30,
                weight_limit: None,
            })
            .with(Item {
                volume: 10,
                weight: 5,
            })
            .build();
        world
            .create_entity()
            .with(ItemStorage {
                items: vec![item1, item3],
                volume_limit: 50,
                weight_limit: None,
            })
            .with(Position { x: 15, y: 9 })
            .with(Displayable {
                name: "Crate",
                texture_atlas_index: 3,
            })
            .build();
        world
            .create_entity()
            .with(Tree { durability: 15 })
            .with(Position { x: 4, y: 5 })
            .with(Displayable {
                name: "Tree",
                texture_atlas_index: 1,
            })
            .build();
        world
            .create_entity()
            .with(Tree { durability: 10 })
            .with(Position { x: 17, y: 9 })
            .with(Displayable {
                name: "Tree",
                texture_atlas_index: 1,
            })
            .build();
        world
            .create_entity()
            .with(Tree { durability: 6 })
            .with(Position { x: 12, y: 17 })
            .with(Displayable {
                name: "Tree",
                texture_atlas_index: 1,
            })
            .build();
        world
            .create_entity()
            .with(Tree { durability: 8 })
            .with(Position { x: 7, y: 16 })
            .with(Displayable {
                name: "Tree",
                texture_atlas_index: 1,
            })
            .build();
        world
            .create_entity()
            .with(Tree { durability: 8 })
            .with(Position { x: 19, y: 0 })
            .with(Displayable {
                name: "Tree",
                texture_atlas_index: 1,
            })
            .build();
        for x in 8..=19 {
            world
                .create_entity()
                .with(Position { x, y: 7 })
                .with(Displayable {
                    name: "Wall",
                    texture_atlas_index: 2,
                })
                .build();
        }
        world
            .create_entity()
            .with(Position { x: 8, y: 8 })
            .with(Displayable {
                name: "Wall",
                texture_atlas_index: 2,
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 8, y: 9 })
            .with(Displayable {
                name: "Wall",
                texture_atlas_index: 2,
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 8, y: 10 })
            .with(Displayable {
                name: "Wall",
                texture_atlas_index: 2,
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 8, y: 11 })
            .with(Displayable {
                name: "Wall",
                texture_atlas_index: 2,
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 8, y: 12 })
            .with(Displayable {
                name: "Wall",
                texture_atlas_index: 2,
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 8, y: 13 })
            .with(Displayable {
                name: "Wall",
                texture_atlas_index: 2,
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 8, y: 14 })
            .with(Displayable {
                name: "Wall",
                texture_atlas_index: 2,
            })
            .build();
    }

    // Main loop
    let delta_time = Duration::from_millis(300);
    let mut current_time = Instant::now();
    let mut accumulator = Duration::from_secs(0);
    let mut is_paused = false;
    let mut should_close = false;
    while !should_close {
        // Poll events
        event_loop.poll_events(|event| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => should_close = true,
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == ElementState::Pressed {
                        match input.virtual_keycode {
                            Some(VirtualKeyCode::P) => {
                                is_paused = !is_paused;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        });

        // Update based on timer
        let new_time = Instant::now();
        accumulator += new_time - current_time;
        current_time = new_time;
        while accumulator >= delta_time {
            if !is_paused {
                item_storage_system.run_now(&world.res);
                elf_system.run_now(&world.res);
                world.maintain();
                pathfinding_system.run_now(&world.res);
                movement_system.run_now(&world.res);
            }
            accumulator -= delta_time;
        }

        // Render
        render_system.run_now(&world.res);

        microprofile::flip!();
    }

    microprofile::dump_file_immediately!("profile.html", "");
    microprofile::shutdown!();
}

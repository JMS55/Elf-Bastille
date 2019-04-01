use components::*;
use glium::glutin::dpi::{LogicalPosition, LogicalSize};
use glium::glutin::{
    ContextBuilder, ElementState, Event, EventsLoop, MouseButton, VirtualKeyCode, WindowBuilder,
    WindowEvent,
};
use glium::Display;
use specs::{Builder, RunNow, World};
use std::time::{Duration, Instant};
use systems::*;

mod components;
mod inspector;
mod systems;

pub const WORLD_WIDTH: u32 = 20;
pub const WORLD_HEIGHT: u32 = 20;
pub const NUMBER_OF_TEXTURES: u32 = 6;
pub const TEXTURE_SIZE: u32 = 16;
pub const TEXTURE_SCALE_FACTOR: u32 = 2;

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
    world.register::<Damageable>();
    world.register::<Displayable>();
    world.register::<Durability>();
    world.register::<Item>();
    world.register::<ItemStorage>();
    world.register::<Loot>();
    world.register::<MarkedForDeath>();
    world.register::<Movement>();
    world.register::<Position>();
    world.register::<TurnUsed>();
    world.register::<Weapon>();
    world.register::<TaskChopTrees>();
    world.register::<TaskReplenishItem>();

    // Create systems
    let mut pathfinding_system = PathfindingSystem;
    let mut movement_system = MovementSystem;
    let mut render_system = RenderSystem::new(display);
    let mut loot_system = LootSystem;
    let mut cleanup_dead_system = CleanupDeadSystem;

    // Create entities
    {
        world
            .create_entity()
            .with(Position { x: 0, y: 0 })
            .with(Movement {
                target: None,
                path: Vec::new(),
                move_speed: 1,
            })
            .with(ItemStorage::new(0, Some(0)))
            .with(Displayable {
                text: "Elf",
                texture_atlas_index: 0,
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 19, y: 19 })
            .with(Movement {
                target: None,
                path: Vec::new(),
                move_speed: 1,
            })
            .with(ItemStorage::new(0, Some(0)))
            .with(Displayable {
                text: "Elf",
                texture_atlas_index: 0,
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 2, y: 15 })
            .with(Movement {
                target: None,
                path: Vec::new(),
                move_speed: 1,
            })
            .with(ItemStorage::new(0, Some(0)))
            .with(Displayable {
                text: "Elf",
                texture_atlas_index: 0,
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 10, y: 14 })
            .with(Movement {
                target: None,
                path: Vec::new(),
                move_speed: 1,
            })
            .with(ItemStorage::new(0, Some(0)))
            .with(Displayable {
                text: "Elf",
                texture_atlas_index: 0,
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 10, y: 10 })
            .with(Movement {
                target: None,
                path: Vec::new(),
                move_speed: 1,
            })
            .with(ItemStorage::new(0, Some(0)))
            .with(Displayable {
                text: "Elf",
                texture_atlas_index: 0,
            })
            .build();
        let axe_item = Item {
            volume: 10,
            weight: 5,
        };
        let axe = world
            .create_entity()
            .with(axe_item)
            .with(Displayable {
                text: "Axe",
                texture_atlas_index: 5,
            })
            .build();
        let sheild_item = Item {
            volume: 20,
            weight: 5,
        };
        let shield = world
            .create_entity()
            .with(sheild_item)
            .with(Displayable {
                text: "Shield",
                texture_atlas_index: 5,
            })
            .build();
        let mut packet_item = Item {
            volume: 10,
            weight: 5,
        };
        let mut packet_item_storage = ItemStorage::new(30, Some(70));
        packet_item_storage
            .try_insert(shield, &sheild_item, Some(&mut packet_item))
            .unwrap();
        let packet = world
            .create_entity()
            .with(packet_item_storage)
            .with(packet_item)
            .with(Displayable {
                text: "Packet",
                texture_atlas_index: 5,
            })
            .build();
        let mut crate_item = Item {
            volume: 20,
            weight: 20,
        };
        let mut crate_item_storage = ItemStorage::new(50, None);
        crate_item_storage
            .try_insert(axe, &axe_item, Some(&mut crate_item))
            .unwrap();
        crate_item_storage
            .try_insert(packet, &packet_item, Some(&mut crate_item))
            .unwrap();
        world
            .create_entity()
            .with(crate_item_storage)
            .with(crate_item)
            .with(Position { x: 15, y: 9 })
            .with(Displayable {
                text: "Crate",
                texture_atlas_index: 3,
            })
            .build();
        world
            .create_entity()
            .with(Durability(15))
            .with(Position { x: 4, y: 5 })
            .with(Displayable {
                text: "Tree",
                texture_atlas_index: 1,
            })
            .build();
        world
            .create_entity()
            .with(Durability(10))
            .with(Position { x: 17, y: 9 })
            .with(Displayable {
                text: "Tree",
                texture_atlas_index: 1,
            })
            .build();
        world
            .create_entity()
            .with(Durability(6))
            .with(Position { x: 12, y: 17 })
            .with(Displayable {
                text: "Tree",
                texture_atlas_index: 1,
            })
            .build();
        world
            .create_entity()
            .with(Durability(8))
            .with(Position { x: 7, y: 16 })
            .with(Displayable {
                text: "Tree",
                texture_atlas_index: 1,
            })
            .build();
        world
            .create_entity()
            .with(Durability(8))
            .with(Position { x: 19, y: 0 })
            .with(Displayable {
                text: "Tree",
                texture_atlas_index: 1,
            })
            .build();
        for x in 8..=19 {
            world
                .create_entity()
                .with(Position { x, y: 7 })
                .with(Displayable {
                    text: "Wall",
                    texture_atlas_index: 2,
                })
                .build();
        }
        world
            .create_entity()
            .with(Position { x: 8, y: 8 })
            .with(Displayable {
                text: "Wall",
                texture_atlas_index: 2,
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 8, y: 9 })
            .with(Displayable {
                text: "Wall",
                texture_atlas_index: 2,
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 8, y: 10 })
            .with(Displayable {
                text: "Wall",
                texture_atlas_index: 2,
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 8, y: 11 })
            .with(Displayable {
                text: "Wall",
                texture_atlas_index: 2,
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 8, y: 12 })
            .with(Displayable {
                text: "Wall",
                texture_atlas_index: 2,
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 8, y: 13 })
            .with(Displayable {
                text: "Wall",
                texture_atlas_index: 2,
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 8, y: 14 })
            .with(Displayable {
                text: "Wall",
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
    let mut mouse_position = (0.0, 0.0);
    while !should_close {
        // Poll events
        event_loop.poll_events(|event| {
            imgui_winit_support::handle_event(
                &mut render_system.imgui,
                &event,
                render_system.display.gl_window().get_hidpi_factor(),
                render_system.display.gl_window().get_hidpi_factor().round(),
            );

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => should_close = true,
                    WindowEvent::KeyboardInput { input, .. }
                        if input.state == ElementState::Pressed =>
                    {
                        match input.virtual_keycode {
                            Some(VirtualKeyCode::P) => {
                                if is_paused {
                                    render_system.selected_tile = None;
                                    // TODO: Replace this unsafe block with the new methods when imgui-rs 0.1 comes out - render_system.imgui.io().want_capture_mouse
                                    unsafe { (*imgui::sys::igGetIO()).want_capture_mouse = false };
                                }
                                is_paused = !is_paused;
                            }
                            _ => {}
                        }
                    }
                    WindowEvent::CursorMoved {
                        position: LogicalPosition { x, y },
                        ..
                    } => {
                        mouse_position = (x, y);
                    }
                    WindowEvent::MouseInput {
                        state: ElementState::Pressed,
                        button: MouseButton::Left,
                        ..
                    // TODO: Replace this unsafe block with the new methods when imgui-rs 0.1 comes out - render_system.imgui.io().want_capture_mouse
                    } if is_paused && unsafe { !(*imgui::sys::igGetIO()).want_capture_mouse } => {
                        let new_tile = Position {
                            x: mouse_position.0 as u32 / (TEXTURE_SIZE * TEXTURE_SCALE_FACTOR),
                            y: mouse_position.1 as u32 / (TEXTURE_SIZE * TEXTURE_SCALE_FACTOR),
                        };
                        match render_system.selected_tile {
                            Some(selected_tile) if selected_tile == new_tile => {
                                render_system.selected_tile = None;
                            }
                            _ => {
                                render_system.selected_tile = Some(new_tile);
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        });

        imgui_winit_support::update_mouse_cursor(
            &render_system.imgui,
            &render_system.display.gl_window(),
        );

        // Update based on timer
        let new_time = Instant::now();
        accumulator += new_time - current_time;
        current_time = new_time;
        while accumulator >= delta_time {
            if !is_paused {
                pathfinding_system.run_now(&world.res);
                movement_system.run_now(&world.res);
                loot_system.run_now(&world.res);
                cleanup_dead_system.run_now(&world.res);
                world.maintain();
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

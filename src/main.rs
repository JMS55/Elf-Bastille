use components::*;
use glium::glutin::dpi::{LogicalPosition, LogicalSize};
use glium::glutin::{
    ContextBuilder, ElementState, Event, EventsLoop, Icon, MouseButton, WindowBuilder, WindowEvent,
};
use glium::Display;
use specs::{Builder, Join, RunNow, World};
use std::time::{Duration, Instant};
use systems::*;

mod components;
mod gui;
mod systems;

pub const DELTA_TIME: Duration = Duration::from_nanos(16700000);
pub const VIEWPORT_SIZE: f32 = 21.0;
pub const TEXTURE_SIZE: f32 = 32.0;
pub const NUMBER_OF_TEXTURES: f32 = 11.0;

fn main() {
    let mut event_loop = EventsLoop::new();
    let window = WindowBuilder::new()
        .with_dimensions(LogicalSize::new(
            (VIEWPORT_SIZE * TEXTURE_SIZE) as f64,
            (VIEWPORT_SIZE * TEXTURE_SIZE) as f64,
        ))
        .with_resizable(false)
        .with_title("Elf Bastille")
        .with_window_icon(Some(
            Icon::from_bytes(include_bytes!("../icon.png")).expect("Failed to load icon"),
        ));
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
    world.register::<CraftingMaterial>();
    world.register::<CraftingAid>();
    // Entities //
    world.register::<Elf>();
    world.register::<Tree>();
    world.register::<Dirt>();
    // Actions //
    world.register::<ActionMove>();
    world.register::<ActionStore>();
    world.register::<ActionAttack>();
    world.register::<ActionCraft>();

    let mut elf_system = ElfSystem;
    let mut tree_growth_system = TreeGrowthSystem;
    let mut create_trees_system = CreateTreesSystem::new();
    let mut storage_system = StorageSystem;
    let mut craft_system = CraftSystem;
    let mut attack_system = AttackSystem;
    let mut pathfind_system = PathfindSystem;
    let mut movement_system = MovementSystem;
    let mut render_system = RenderSystem::new(display);

    // Test world - Entities don't have an IsStored component
    let mut ran_once = false;
    let log_entity = world
        .create_entity()
        .with(Texture { atlas_index: 7 })
        .with(LocationInfo::new(Location::new(2, 2, 1), false))
        .with(StorageInfo::new(10, 25, "Log"))
        .build();
    let lizardman_entity = world
        .create_entity()
        .with(Texture { atlas_index: 10 })
        .with(LocationInfo::new(Location::new(5, 7, 1), false))
        .with(Attackable::new(15, WeaponType::Sword, None))
        .build();
    let sword_storage_info = StorageInfo::new(10, 6, "Sword");
    let sword_entity = world
        .create_entity()
        .with(Weapon::new(
            3,
            20,
            Duration::from_secs(1),
            WeaponType::Sword,
        ))
        .with(sword_storage_info)
        .build();
    let axe_storage_info = StorageInfo::new(7, 10, "Axe");
    let axe_entity = world
        .create_entity()
        .with(Weapon::new(7, 20, Duration::from_secs(2), WeaponType::Axe))
        .with(axe_storage_info)
        .build();
    let elf_builder = world.create_entity();
    let mut elf = Elf::new();
    elf.queue_action(ActionMove::new(Location::new(-8, 10, 1)));
    // Should be skipped over
    elf.queue_action(ActionStore::new(
        log_entity,
        elf_builder.entity,
        Duration::from_secs(2),
    ));
    elf.queue_action(ActionMove::new(Location::new(1, 2, 1)));
    elf.queue_action(ActionStore::new(
        log_entity,
        elf_builder.entity,
        Duration::from_secs(2),
    ));
    // Should be skipped over
    elf.queue_action(ActionMove::new(Location::new(10, 10, 1)));
    elf.queue_action(ActionMove::new(Location::new(4, 7, 1)));
    elf.queue_action(ActionAttack::new(lizardman_entity));
    let mut elf_inventory = Inventory::new(100, 100);
    elf_inventory.stored_entities.insert(sword_entity);
    elf_inventory.stored_entities.insert(axe_entity);
    elf_inventory.volume_free -= sword_storage_info.volume;
    elf_inventory.weight_free -= sword_storage_info.weight;
    elf_inventory.volume_free -= axe_storage_info.volume;
    elf_inventory.weight_free -= axe_storage_info.weight;
    elf_builder
        .with(elf)
        .with(Texture { atlas_index: 1 })
        .with(MovementInfo::new(Duration::from_millis(333)))
        .with(LocationInfo::new(Location::new(0, 0, 1), false))
        .with(elf_inventory)
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
    let mut mouse_location = LogicalPosition::new(0.0, 0.0);
    while !should_close {
        event_loop.poll_events(|event| {
            imgui_winit_support::handle_event(
                &mut render_system.gui.imgui,
                &event,
                render_system.display.gl_window().get_hidpi_factor(),
                render_system.display.gl_window().get_hidpi_factor().round(),
            );

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => should_close = true,
                    WindowEvent::CursorMoved { position, .. } => {
                        mouse_location = position;
                    }
                    WindowEvent::MouseInput { state, button, .. }
                        if (state == ElementState::Pressed && button == MouseButton::Left) =>
                    {
                        let click_location = Location::new(
                            (mouse_location.x / TEXTURE_SIZE as f64) as i32
                                - (VIEWPORT_SIZE / 2.0).floor() as i32,
                            (-mouse_location.y / TEXTURE_SIZE as f64) as i32
                                + (VIEWPORT_SIZE / 2.0).floor() as i32,
                            0,
                        );
                        render_system.gui.handle_click(click_location, &world);
                    }
                    WindowEvent::KeyboardInput { input, .. }
                        if input.state == ElementState::Pressed =>
                    {
                        match input.scancode {
                            // W
                            17 => {
                                render_system.camera_center.y += 1;
                            }
                            // A
                            30 => {
                                render_system.camera_center.x -= 1;
                            }
                            // S
                            31 => {
                                render_system.camera_center.y -= 1;
                            }
                            // D
                            32 => {
                                render_system.camera_center.x += 1;
                            }
                            _ => {}
                        }

                    }
                    _ => {}
                },
                _ => {}
            }

            imgui_winit_support::update_mouse_cursor(
                &render_system.gui.imgui,
                &render_system.display.gl_window(),
            );
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
            storage_system.run_now(&world.res);
            world.maintain();
            craft_system.run_now(&world.res);
            world.maintain();
            attack_system.run_now(&world.res);
            world.maintain();
            pathfind_system.run_now(&world.res);
            movement_system.run_now(&world.res);
            world.maintain();

            // Test world
            if !ran_once {
                let tree_entity = {
                    let mut tree_entity = None;
                    for (_, entity, location) in (
                        &world.read_storage::<Tree>(),
                        &world.entities(),
                        &world.read_storage::<LocationInfo>(),
                    )
                        .join()
                    {
                        if location.location == Location::new(-10, -7, 1) {
                            tree_entity = Some(entity);
                            break;
                        }
                    }
                    tree_entity
                };
                for elf in (&mut world.write_storage::<Elf>()).join() {
                    elf.queue_action(ActionMove::new(Location::new(-9, -7, 1)));
                    elf.queue_action(ActionAttack::new(tree_entity.unwrap()));
                }
                ran_once = true;
            }

            accumulator -= DELTA_TIME;
        }

        render_system.run_now(&world.res);
    }
}

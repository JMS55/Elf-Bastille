use components::*;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::{ContextBuilder, Event, EventsLoop, WindowBuilder, WindowEvent};
use glium::Display;
use specs::{RunNow, World};
use std::time::{Duration, Instant};
use systems::*;

mod components;
mod systems;

pub const WORLD_WIDTH: u32 = 20;
pub const WORLD_HEIGHT: u32 = 20;
pub const NUMBER_OF_TEXTURES: u32 = 6;
pub const TEXTURE_SIZE: u32 = 16;
pub const TEXTURE_SCALE_FACTOR: u32 = 2;
pub const DELTA_TIME: Duration = Duration::from_nanos(16700000);

fn main() {
    microprofile::init!();
    microprofile::set_enable_all_groups!(true);

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

    let mut world = World::new();
    world.register::<AI>();
    world.register::<Container>();
    world.register::<ContainerChild>();
    world.register::<Damageable>();
    world.register::<Displayable>();
    world.register::<EntityType>();
    world.register::<Growable>();
    world.register::<Loot>();
    world.register::<MarkedForDeath>();
    world.register::<MovementSpeed>();
    world.register::<PhysicalProperties>();
    world.register::<Position>();
    world.register::<Walkable>();

    // TODO: Create systems
    let mut ai_system = AISystem;
    /*
        ✓ AISystem (AI) - Figures out what action to take, and adds the appropriate component
        InsertIntoContainerSystem (?)
        TakeFromContainerSystem(?)
        CraftSystem (?)
        AttackSystem (?)
        PathfindingSystem (?)
        MovementSystem (?)
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
            accumulator -= DELTA_TIME;
        }

        // TODO: Render
        // TODO: Use accumulator for interpolating render or something

        microprofile::flip!();
    }

    microprofile::dump_file_immediately!("profile.html", "");
    microprofile::shutdown!();
}

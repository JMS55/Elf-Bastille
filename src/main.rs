use components::*;
use glium::glutin::{ContextBuilder, Event, EventsLoop, WindowBuilder, WindowEvent};
use glium::Display;
use specs::{RunNow, World};
use std::time::{Duration, Instant};
use systems::*;

mod components;
mod systems;

pub const DELTA_TIME: Duration = Duration::from_nanos(16700000);

fn main() {
    microprofile::init!();
    microprofile::set_enable_all_groups!(true);

    let mut event_loop = EventsLoop::new();
    let window = WindowBuilder::new().with_title("Elf Bastille");
    let context = ContextBuilder::new().with_vsync(true).with_srgb(true);
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

    // TODO: Create systems
    let mut ai_system = AISystem;
    let mut insert_into_container_system = InsertIntoContainerSystem;
    let mut take_from_container_system = TakeFromContainerSystem;
    /*
        ✓ AISystem (AI) - Figures out what action to take, and adds the appropriate component
        ✓ InsertIntoContainerSystem (?)
        TakeFromContainerSystem(?)
        CraftingSystem (?)
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
            insert_into_container_system.run_now(&world.res);
            world.maintain();
            take_from_container_system.run_now(&world.res);
            world.maintain();
            // crafting_system.run_now(&world.res);
            // world.maintain();
            // attack_system.run_now(&world.res);
            // pathfinding_system.run_now(&world.res);
            // movementSpeed.run_now(&world.res);
            // growth_system.run_now(&world.res);
            // loot_system.run_now(&world.res);
            //world.maintain();
            // clean_up_dead_system.run_now(&world.res);
            // world.maintain();
            accumulator -= DELTA_TIME;
        }

        // TODO: Render

        microprofile::flip!();
    }

    microprofile::dump_file_immediately!("profile.html", "");
    microprofile::shutdown!();
}

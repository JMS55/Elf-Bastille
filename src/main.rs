use components::*;
use context::Context;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::{
    ContextBuilder, ElementState, Event, EventsLoop, VirtualKeyCode, WindowBuilder, WindowEvent,
};
use glium::Display;
use std::time::{Duration, Instant};
use systems::render::{TEXTURE_SCALE_FACTOR, TEXTURE_SIZE};
use systems::*;

mod actions;
mod components;
mod context;
mod systems;

pub const WORLD_WIDTH: u32 = 20;
pub const WORLD_HEIGHT: u32 = 20;

fn main() {
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

    let mut context = Context::new();
    let growth_system = GrowthSystem;
    let render_system = RenderSystem::new(display);

    let x = context.entity_type_components.insert(EntityType::Elf);
    context.displayable_components.insert(
        x,
        Displayable {
            texture_atlas_index: 0,
        },
    );
    context
        .position_components
        .insert(x, Position { x: 10, y: 10 });

    let delta_time = Duration::from_millis(300);
    let mut current_time = Instant::now();
    let mut accumulator = Duration::from_secs(0);
    let mut is_paused = false;
    let mut should_close = false;
    while !should_close {
        event_loop.poll_events(|event| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => should_close = true,
                WindowEvent::KeyboardInput { input, .. }
                    if input.state == ElementState::Pressed =>
                {
                    match input.virtual_keycode {
                        Some(VirtualKeyCode::P) => {
                            is_paused = !is_paused;
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            _ => {}
        });

        let new_time = Instant::now();
        accumulator += new_time - current_time;
        current_time = new_time;
        while accumulator >= delta_time {
            if !is_paused {
                for (entity_key, ai_component) in context.ai_components.iter() {
                    (ai_component.get_action)(&context, entity_key).execute();
                }
                growth_system.run(&mut context);
            }
            accumulator -= delta_time;
        }
        render_system.render(&context);
    }
}

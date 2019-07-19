mod components;
mod systems;

use components::*;
use glium::glutin::dpi::LogicalPosition;
use glium::glutin::{ElementState, Event, EventsLoop, MouseButton, WindowEvent};
use specs::{RunNow, World, WorldExt};
use std::time::{Duration, Instant};
use systems::*;

pub const DELTA_TIME: Duration = Duration::from_nanos(16700000);

fn main() {
    let mut event_loop = EventsLoop::new();
    let mut current_time = Instant::now();
    let mut accumulator = Duration::from_nanos(0);
    let mut cursor_position = LogicalPosition::new(0.0, 0.0);
    let mut should_close = false;

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Texture>();
    let mut render_system = RenderSystem::new(&event_loop);

    while !should_close {
        event_loop.poll_events(|event| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => should_close = true,
                WindowEvent::CursorMoved { position, .. } => {
                    cursor_position = position;
                    render_system.set_hovered_tile(cursor_position);
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    if state == ElementState::Pressed && button == MouseButton::Left {}
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == ElementState::Pressed {
                        match input.scancode {
                            // Esc
                            #[cfg(debug_assertions)]
                            1 => {
                                should_close = true;
                            }
                            // W
                            17 => {
                                render_system.move_camera(0, 1, cursor_position);
                            }
                            // A
                            30 => {
                                render_system.move_camera(-1, 0, cursor_position);
                            }
                            // S
                            31 => {
                                render_system.move_camera(0, -1, cursor_position);
                            }
                            // D
                            32 => {
                                render_system.move_camera(1, 0, cursor_position);
                            }
                            // -
                            12 => {
                                render_system.zoom_camera(false, cursor_position);
                            }
                            // +
                            13 => {
                                render_system.zoom_camera(true, cursor_position);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        });

        let new_time = Instant::now();
        accumulator += new_time - current_time;
        current_time = new_time;
        while accumulator >= DELTA_TIME {
            accumulator -= DELTA_TIME;
        }
        render_system.run_now(&world);
    }
}

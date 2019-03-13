use components::*;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use specs::{Builder, DispatcherBuilder, RunNow, World};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use systems::*;

mod components;
mod systems;

pub const WORLD_WIDTH: u32 = 20;
pub const WORLD_HEIGHT: u32 = 20;
const TILE_SIZE: u32 = 32;

fn main() {
    // Create the world
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Movement>();
    world.register::<Sprite>();
    world.register::<Elf>();
    world.register::<Tree>();

    // Create the dispatcher
    let mut dispatcher = DispatcherBuilder::new()
        .with(ElfSystem, "elf", &[])
        .with(PathfindingSystem, "pathfinding", &["elf"])
        .with(MovementSystem, "movement", &["pathfinding"])
        .build();

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
            .with(Sprite { name: "elf" })
            .build();
        world
            .create_entity()
            .with(Elf {})
            .with(Position { x: 20, y: 20 })
            .with(Movement {
                target: None,
                path: Vec::new(),
                move_speed: 1,
            })
            .with(Sprite { name: "elf" })
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
            .with(Sprite { name: "elf" })
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
            .with(Sprite { name: "elf" })
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
            .with(Sprite { name: "elf" })
            .build();
        world
            .create_entity()
            .with(Tree { durability: 15 })
            .with(Position { x: 4, y: 5 })
            .with(Sprite { name: "tree" })
            .build();
        world
            .create_entity()
            .with(Tree { durability: 10 })
            .with(Position { x: 17, y: 9 })
            .with(Sprite { name: "tree" })
            .build();
        world
            .create_entity()
            .with(Tree { durability: 6 })
            .with(Position { x: 12, y: 17 })
            .with(Sprite { name: "tree" })
            .build();
        world
            .create_entity()
            .with(Tree { durability: 8 })
            .with(Position { x: 7, y: 16 })
            .with(Sprite { name: "tree" })
            .build();
        world
            .create_entity()
            .with(Tree { durability: 8 })
            .with(Position { x: 20, y: 0 })
            .with(Sprite { name: "tree" })
            .build();
    }

    // Initialize SDL2
    let sdl_context = sdl2::init().expect("Could not initialize sdl2");
    let video_subsystem = sdl_context
        .video()
        .expect("Could not initialize sdl2 video subsystem");
    let window = video_subsystem
        .window(
            "Elf Bastille",
            WORLD_WIDTH * TILE_SIZE,
            WORLD_HEIGHT * TILE_SIZE,
        )
        .build()
        .expect("Could not create window");
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not create canvas");
    canvas.set_draw_color((130, 60, 30, 255));

    // Setup textures and render system
    let texture_creator = canvas.texture_creator();
    let elf_texture = texture_creator.load_texture("elf.png").unwrap();
    let tree_texture = texture_creator.load_texture("tree.png").unwrap();
    let mut textures = HashMap::new();
    textures.insert("elf", elf_texture);
    textures.insert("tree", tree_texture);
    let mut render_system = RenderSystem {
        tile_size: TILE_SIZE,
        textures,
        canvas,
    };

    // Setup timer system
    let dt = Duration::from_millis(500);
    let mut current_time = Instant::now();
    let mut accumulator = Duration::from_secs(0);

    'mainloop: loop {
        // Handle input
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            if let Event::Quit { .. } = event {
                break 'mainloop;
            }
        }

        // Update based on timer
        let new_time = Instant::now();
        accumulator += new_time - current_time;
        current_time = new_time;
        while accumulator >= dt {
            dispatcher.dispatch(&world.res);
            world.maintain();
            accumulator -= dt;
        }

        // Render
        render_system.run_now(&world.res);
    }
}

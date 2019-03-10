use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use specs::{
    Builder, Component, DispatcherBuilder, Entities, Join, ReadStorage, RunNow, System, VecStorage,
    World, WriteStorage,
};
use specs_derive::Component;
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Sprite>();
    world.register::<Elf>();
    world.register::<Tree>();

    world
        .create_entity()
        .with(Elf {})
        .with(Position { x: 0, y: 0, z: 1 })
        .with(Sprite { name: "elf" })
        .build();
    world
        .create_entity()
        .with(Elf {})
        .with(Position { x: 20, y: 20, z: 1 })
        .with(Sprite { name: "elf" })
        .build();
    world
        .create_entity()
        .with(Elf {})
        .with(Position { x: 10, y: 10, z: 1 })
        .with(Sprite { name: "elf" })
        .build();
    world
        .create_entity()
        .with(Tree { durability: 15 })
        .with(Position { x: 4, y: 5, z: 1 })
        .with(Sprite { name: "tree" })
        .build();
    world
        .create_entity()
        .with(Tree { durability: 10 })
        .with(Position { x: 17, y: 9, z: 1 })
        .with(Sprite { name: "tree" })
        .build();
    world
        .create_entity()
        .with(Tree { durability: 6 })
        .with(Position { x: 12, y: 17, z: 1 })
        .with(Sprite { name: "tree" })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(ElfSystem, "elf_system", &[])
        .build();

    let sdl_context = sdl2::init().expect("Could not initialize sdl2");
    let video_subsystem = sdl_context
        .video()
        .expect("Could not initialize sdl2 video subsystem");
    let window = video_subsystem
        .window("Elf Bastille", 20 * 32, 20 * 32)
        .build()
        .expect("Could not create window");
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not create canvas");
    canvas.set_draw_color((130, 60, 30, 255));

    let texture_creator = canvas.texture_creator();
    let elf_texture = texture_creator.load_texture("elf.png").unwrap();
    let tree_texture = texture_creator.load_texture("tree.png").unwrap();
    let mut textures = HashMap::new();
    textures.insert("elf", elf_texture);
    textures.insert("tree", tree_texture);
    let mut render_system = RenderSystem {
        tile_size: 32,
        textures,
        canvas,
    };

    let dt = Duration::from_millis(500);
    let mut current_time = Instant::now();
    let mut accumulator = Duration::from_secs(0);
    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }

        let new_time = Instant::now();
        accumulator += new_time - current_time;
        current_time = new_time;
        while accumulator >= dt {
            dispatcher.dispatch(&world.res);
            world.maintain();
            accumulator -= dt;
        }

        render_system.run_now(&world.res);
    }
}

struct RenderSystem<'t> {
    tile_size: u32,
    textures: HashMap<&'static str, Texture<'t>>,
    canvas: Canvas<Window>,
}

impl<'t, 's> System<'s> for RenderSystem<'t> {
    type SystemData = (ReadStorage<'s, Position>, WriteStorage<'s, Sprite>);

    fn run(&mut self, data: Self::SystemData) {
        self.canvas.clear();
        let (position_data, sprite_data) = data;
        // Todo: sort each (position, sprite) pair by position.z, render reverse Z order
        for (position, sprite) in (&position_data, &sprite_data).join() {
            self.canvas
                .copy(
                    self.textures.get(sprite.name).unwrap(),
                    None,
                    Some(Rect::new(
                        (position.x * self.tile_size).saturating_sub(self.tile_size) as i32,
                        (position.y * self.tile_size).saturating_sub(self.tile_size) as i32,
                        self.tile_size,
                        self.tile_size,
                    )),
                )
                .unwrap();
        }
        self.canvas.present();
    }
}

struct ElfSystem;
impl<'a> System<'a> for ElfSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Elf>,
        WriteStorage<'a, Tree>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut position_data, elf_data, mut tree_data) = data;
        let tree_positions: Vec<Position> = (&tree_data, &position_data)
            .join()
            .map(|(_, position)| position.clone())
            .collect();
        for (_, elf_position) in (&elf_data, &mut position_data).join() {
            let tree = (&mut tree_data)
                .join()
                .zip(tree_positions.iter())
                .filter(|tree| {
                    let mut adjacent = false;
                    for offset in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
                        let new_x = elf_position.x as i32 + offset.0;
                        let new_y = elf_position.y as i32 + offset.1;
                        if new_x == tree.1.x as i32 && new_y == tree.1.y as i32 {
                            adjacent = true;
                            break;
                        }
                    }
                    adjacent && tree.0.durability != 0
                })
                .next();
            match tree {
                Some((tree, _)) => tree.durability -= 1,
                None => {
                    let mut frontier = VecDeque::new();
                    let mut came_from = HashMap::new();
                    let mut target = None;
                    frontier.push_back((elf_position.x, elf_position.y));
                    came_from.insert((elf_position.x, elf_position.y), None);

                    while !frontier.is_empty() {
                        let visiting = frontier.pop_front().unwrap();

                        // If visiting tree, break
                        if !tree_positions
                            .iter()
                            .filter(|position| (position.x, position.y) == visiting)
                            .collect::<Vec<&Position>>()
                            .is_empty()
                        {
                            target = Some(visiting);
                            break;
                        }

                        // TODO: Check if neighbor tile is unoccupied
                        for offset in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
                            let neighbor =
                                (visiting.0 as i32 + offset.0, visiting.1 as i32 + offset.1);
                            if neighbor.0 < 0
                                || neighbor.1 < 0
                                || neighbor.0 > 20
                                || neighbor.1 > 20
                            {
                                continue;
                            }
                            let neighbor = (neighbor.0 as u32, neighbor.1 as u32);
                            // if !position_data
                            //     .join()
                            //     .filter(|position| (position.x, position.y) == neighbor)
                            //     .collect::<Vec<&Position>>()
                            //     .is_empty()
                            // {
                            //     continue;
                            // }

                            if !came_from.contains_key(&neighbor) {
                                frontier.push_back(neighbor);
                                came_from.insert(neighbor, Some(visiting));
                            }
                        }
                    }

                    if let Some(mut current) = target {
                        let mut path = Vec::new();
                        while current != (elf_position.x, elf_position.y) {
                            path.push(current);
                            current = came_from.get(&current).unwrap().unwrap();
                        }
                        // TODO: Broken
                        elf_position.x = path[path.len() - 2].0;
                        elf_position.y = path[path.len() - 2].1;
                    }
                }
            }
        }

        for (entity, tree) in (&entities, &tree_data).join() {
            if tree.durability == 0 {
                entities.delete(entity).unwrap();
            }
        }
    }
}

#[derive(Component, Copy, Clone)]
#[storage(VecStorage)]
struct Position {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(Component, Copy, Clone)]
#[storage(VecStorage)]
struct Sprite {
    name: &'static str,
}

#[derive(Component, Copy, Clone)]
#[storage(VecStorage)]
struct Elf;

#[derive(Component, Copy, Clone)]
#[storage(VecStorage)]
struct Tree {
    durability: u32,
}

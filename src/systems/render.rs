use crate::components::{Position, Sprite};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use specs::{Join, ReadStorage, System, WriteStorage};
use std::collections::HashMap;

pub struct RenderSystem<'t> {
    pub tile_size: u32,
    pub textures: HashMap<&'static str, Texture<'t>>,
    pub canvas: Canvas<Window>,
}

impl<'t, 's> System<'s> for RenderSystem<'t> {
    type SystemData = (ReadStorage<'s, Position>, WriteStorage<'s, Sprite>);

    fn run(&mut self, (position_data, sprite_data): Self::SystemData) {
        self.canvas.clear();

        for (position, sprite) in (&position_data, &sprite_data).join() {
            self.canvas
                .copy(
                    &self.textures[sprite.name],
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

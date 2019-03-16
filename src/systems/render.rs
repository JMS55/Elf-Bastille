use crate::components::{Position, Sprite};
use crate::TILE_SIZE;
use imgui::ImGui;
use imgui_opengl_renderer::Renderer;
use imgui_sdl2::ImguiSdl2;
use microprofile::scope;
use sdl2::mouse::MouseState;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use specs::{Entities, Join, ReadStorage, System, WriteStorage};
use std::collections::HashMap;

pub struct RenderSystem<'r> {
    pub tile_size: u32,
    pub textures: HashMap<&'static str, Texture<'r>>,
    pub should_render_gui: bool,

    pub canvas: Canvas<Window>,
    pub mouse_state: MouseState,
    pub imgui: ImGui,
    pub imgui_sdl: ImguiSdl2,
    pub imgui_renderer: Renderer,
}

impl<'r, 's> System<'s> for RenderSystem<'r> {
    type SystemData = (
        ReadStorage<'s, Position>,
        WriteStorage<'s, Sprite>,
        Entities<'s>,
    );

    fn run(&mut self, (position_data, sprite_data, entities): Self::SystemData) {
        self.canvas.clear();

        // Render game
        microprofile::scope!("rendering", "game");
        for (position, sprite) in (&position_data, &sprite_data).join() {
            self.canvas
                .copy(
                    &self.textures[sprite.name],
                    None,
                    Some(Rect::new(
                        (position.x * self.tile_size) as i32,
                        (position.y * self.tile_size) as i32,
                        self.tile_size,
                        self.tile_size,
                    )),
                )
                .unwrap();
        }

        // Render gui
        if self.should_render_gui {
            microprofile::scope!("rendering", "gui");

            let inspector_window =
                self.imgui_sdl
                    .frame(&self.canvas.window(), &mut self.imgui, &self.mouse_state);

            let selected_position = Position {
                x: self.mouse_state.x() as u32 / TILE_SIZE,
                y: self.mouse_state.y() as u32 / TILE_SIZE,
            };
            if let Some((entity, _, sprite)) = (&entities, &position_data, &sprite_data)
                .join()
                .find(|(_, position, _)| position == &&selected_position)
            {
                inspector_window.text(sprite.name);
            } else {
                inspector_window.text("No selected entity");
            }

            self.imgui_renderer.render(inspector_window);
        }

        self.canvas.window().gl_swap_window();
    }
}

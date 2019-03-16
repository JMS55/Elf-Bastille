use crate::components::{Displayable, Elf, ItemStorage, Movement, Position, Tree};
use crate::inspector::Inspectable;
use crate::{TILE_SIZE, WORLD_HEIGHT, WORLD_WIDTH};
use imgui::{im_str, ImGui, ImGuiCond};
use imgui_opengl_renderer::Renderer;
use imgui_sdl2::ImguiSdl2;
use microprofile::scope;
use sdl2::mouse::MouseState;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use specs::{Entities, Join, ReadStorage, System};
use std::collections::HashMap;

pub struct RenderSystem<'r> {
    pub selected_position: Option<Position>,
    pub tile_size: u32,
    pub textures: HashMap<&'static str, Texture<'r>>,
    pub canvas: Canvas<Window>,
    pub mouse_state: MouseState,
    pub imgui: ImGui,
    pub imgui_sdl: ImguiSdl2,
    pub imgui_renderer: Renderer,
}

impl<'r, 's> System<'s> for RenderSystem<'r> {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Displayable>,
        ReadStorage<'s, Elf>,
        ReadStorage<'s, Tree>,
        ReadStorage<'s, Position>,
        ReadStorage<'s, Movement>,
        ReadStorage<'s, ItemStorage>,
    );

    fn run(
        &mut self,
        (
            entities,
            displayable_data,
            elf_data,
            tree_data,
            position_data,
            movement_data,
            item_storage_data,
        ): Self::SystemData,
    ) {
        self.canvas.clear();

        // Render game
        microprofile::scope!("rendering", "game");
        for (position, displayable) in (&position_data, &displayable_data).join() {
            self.canvas
                .copy(
                    &self.textures[displayable.name],
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
        microprofile::scope!("rendering", "gui");
        if let Some(selected_position) = self.selected_position {
            self.canvas
                .copy(
                    &self.textures["selected"],
                    None,
                    Some(Rect::new(
                        (selected_position.x * self.tile_size) as i32,
                        (selected_position.y * self.tile_size) as i32,
                        self.tile_size,
                        self.tile_size,
                    )),
                )
                .unwrap();

            let inspector =
                self.imgui_sdl
                    .frame(&self.canvas.window(), &mut self.imgui, &self.mouse_state);
            inspector
                .window(im_str!("Inspector"))
                .size(
                    (
                        (WORLD_WIDTH * TILE_SIZE) as f32 / 2.0,
                        (WORLD_HEIGHT * TILE_SIZE) as f32 / 2.0,
                    ),
                    ImGuiCond::FirstUseEver,
                )
                .build(|| {
                    if let Some((entity, position, displayable)) =
                        (&entities, &position_data, &displayable_data)
                            .join()
                            .find(|(_, position, _)| position == &&selected_position)
                    {
                        displayable.create_ui(&inspector);
                        if let Some(elf) = &elf_data.get(entity) {
                            elf.create_ui(&inspector);
                        }
                        if let Some(tree) = &tree_data.get(entity) {
                            tree.create_ui(&inspector);
                        }
                        position.create_ui(&inspector);
                        if let Some(movement) = &movement_data.get(entity) {
                            movement.create_ui(&inspector);
                        }
                        if let Some(item_storage) = &item_storage_data.get(entity) {
                            // item_storage.create_ui(&inspector);
                        }
                    }
                });
            self.imgui_renderer.render(inspector);
        }

        self.canvas.window().gl_swap_window();
    }
}

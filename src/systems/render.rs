use crate::components::{Displayable, Elf, Item, ItemStorage, Movement, Position, Tree};
use crate::{NUMBER_OF_TEXTURES, TEXTURE_SCALE_FACTOR, TEXTURE_SIZE, WORLD_HEIGHT, WORLD_WIDTH};
use glium::texture::{RawImage2d, SrgbTexture2d};
use glium::uniforms::{MagnifySamplerFilter, Sampler};
use glium::{
    implement_vertex, index::PrimitiveType, uniform, Blend, Display, DrawParameters, IndexBuffer,
    Program, Surface, VertexBuffer,
};
use imgui::{im_str, FrameSize, ImGui, ImGuiCond};
use imgui_glium_renderer::Renderer;
use microprofile::scope;
use specs::{Entities, Join, ReadStorage, System};
use std::io::Cursor;

pub struct RenderSystem {
    pub selected_tile: Option<Position>,
    pub display: Display,
    pub imgui: ImGui,
    imgui_renderer: Renderer,
    texture_atlas: SrgbTexture2d,
    program: Program,
    template_vertices: VertexBuffer<TemplateVertex>,
    indices: IndexBuffer<u8>,
}

impl RenderSystem {
    pub fn new(display: Display) -> Self {
        let mut imgui = ImGui::init();
        imgui.set_ini_filename(None);
        imgui_winit_support::configure_keys(&mut imgui);

        let imgui_renderer =
            Renderer::init(&mut imgui, &display).expect("Failed to create ImGui renderer");

        let image = image::load(
            Cursor::new(&include_bytes!("../../texture_atlas.png")[..]),
            image::PNG,
        )
        .expect("Failed to load texture atlas")
        .to_rgba();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture_atlas =
            SrgbTexture2d::new(&display, image).expect("Failed to create texture atlas texture");

        let vertex_shader_src = format!(
            r#"
            #version 130

            in vec2 position;
            in vec2 texture;
            in vec2 tile_position;
            in float texture_atlas_index;
            out vec2 v_texture;

            void main() {{
                v_texture = texture;
                v_texture.x += texture_atlas_index / {};
                vec2 position_offset = tile_position;
                position_offset.x = 2.0 * position_offset.x / {} - 1.0;
                position_offset.y = -2.0 * position_offset.y / {} + 1.0;
                gl_Position = vec4(position + position_offset, 0.0, 1.0);
            }}
        "#,
            NUMBER_OF_TEXTURES as f32, WORLD_WIDTH as f32, WORLD_HEIGHT as f32
        );
        let fragment_shader_src = r#"
            #version 130

            uniform sampler2D texture_atlas;
            in vec2 v_texture;
            out vec4 pixel;

            void main() {
                pixel = texture(texture_atlas, v_texture);
            }
        "#;
        let program = Program::from_source(&display, &vertex_shader_src, fragment_shader_src, None)
            .expect("Failed to create program");

        let template_vertices = VertexBuffer::new(
            &display,
            &[
                TemplateVertex {
                    position: [0.0, 0.0],
                    texture: [0.0, 1.0],
                },
                TemplateVertex {
                    position: [2.0 / WORLD_WIDTH as f32, 0.0],
                    texture: [1.0 / NUMBER_OF_TEXTURES as f32, 1.0],
                },
                TemplateVertex {
                    position: [0.0, -2.0 / WORLD_HEIGHT as f32],
                    texture: [0.0, 0.0],
                },
                TemplateVertex {
                    position: [2.0 / WORLD_WIDTH as f32, -2.0 / WORLD_HEIGHT as f32],
                    texture: [1.0 / NUMBER_OF_TEXTURES as f32, 0.0],
                },
            ],
        )
        .expect("Failed to create template vertex buffer");

        let indices = IndexBuffer::new(
            &display,
            PrimitiveType::TrianglesList,
            &[0u8, 1, 2, 1, 2, 3],
        )
        .expect("Failed to create template index buffer");

        Self {
            selected_tile: None,
            display,
            imgui,
            imgui_renderer,
            texture_atlas,
            program,
            template_vertices,
            indices,
        }
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Displayable>,
        ReadStorage<'a, Elf>,
        ReadStorage<'a, Tree>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Movement>,
        ReadStorage<'a, ItemStorage>,
        ReadStorage<'a, Item>,
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
            item_data,
        ): Self::SystemData,
    ) {
        let mut draw_target = self.display.draw();
        draw_target.clear_color_srgb(0.50, 0.25, 0.12, 1.0);

        // Render game
        microprofile::scope!("rendering", "game");
        let mut instance_data = (&position_data, &displayable_data)
            .join()
            .map(|(position, displayable)| InstanceData {
                tile_position: [position.x as f32, position.y as f32],
                texture_atlas_index: displayable.texture_atlas_index as f32,
            })
            .collect::<Vec<_>>();
        if let Some(selected_tile) = self.selected_tile {
            instance_data.push(InstanceData {
                tile_position: [selected_tile.x as f32, selected_tile.y as f32],
                texture_atlas_index: 4.0,
            });
        }
        let instances = VertexBuffer::new(&self.display, &instance_data)
            .expect("Failed to create vertex buffer");

        let draw_parameters = DrawParameters {
            blend: Blend::alpha_blending(),
            multisampling: false,
            dithering: false,
            ..Default::default()
        };
        draw_target
            .draw(
                (
                    &self.template_vertices,
                    instances
                        .per_instance()
                        .expect("Draw call failed: Instancing not supported"),
                ),
                &self.indices,
                &self.program,
                &uniform!(texture_atlas: Sampler::new(&self.texture_atlas)
                        .magnify_filter(MagnifySamplerFilter::Nearest)),
                &draw_parameters,
            )
            .expect("Draw call f9iled");

        // Render gui
        if let Some(selected_tile) = self.selected_tile {
            microprofile::scope!("rendering", "gui");
            let inspector = self.imgui.frame(
                FrameSize::new(
                    (WORLD_WIDTH * TEXTURE_SIZE * TEXTURE_SCALE_FACTOR) as f64,
                    (WORLD_HEIGHT * TEXTURE_SIZE * TEXTURE_SCALE_FACTOR) as f64,
                    self.display.gl_window().get_hidpi_factor(),
                ),
                1.0 / 60.0, // TODO: Proper delta time
            );
            inspector
                .window(im_str!("Inspector"))
                .size(
                    (
                        (WORLD_WIDTH * TEXTURE_SIZE * TEXTURE_SCALE_FACTOR) as f32 / 1.5,
                        (WORLD_HEIGHT * TEXTURE_SIZE * TEXTURE_SCALE_FACTOR) as f32 / 2.0,
                    ),
                    ImGuiCond::FirstUseEver,
                )
                .build(|| {
                    if let Some((entity, position, displayable)) =
                        (&entities, &position_data, &displayable_data)
                            .join()
                            .find(|(_, position, _)| position == &&selected_tile)
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
                        if let Some(item) = &item_data.get(entity) {
                            item.create_ui(&inspector);
                        }
                        if let Some(item_storage) = &item_storage_data.get(entity) {
                            item_storage.create_ui(
                                &inspector,
                                &item_data,
                                &item_storage_data,
                                &displayable_data,
                            );
                        }
                    }
                });
            self.imgui_renderer
                .render(&mut draw_target, inspector)
                .expect("Failed to render inspector window");
        }

        draw_target
            .finish()
            .expect("Could not swap display buffers");
    }
}

#[derive(Copy, Clone)]
struct TemplateVertex {
    position: [f32; 2],
    texture: [f32; 2],
}

#[derive(Copy, Clone)]
struct InstanceData {
    tile_position: [f32; 2],
    texture_atlas_index: f32,
}

implement_vertex!(TemplateVertex, position, texture);
implement_vertex!(InstanceData, tile_position, texture_atlas_index);

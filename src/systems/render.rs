pub const NUMBER_OF_TEXTURES: u32 = 6;
pub const TEXTURE_SIZE: u32 = 16;
pub const TEXTURE_SCALE_FACTOR: u32 = 2;

use crate::components::{Displayable, Position};
use crate::Context;
use crate::{WORLD_HEIGHT, WORLD_WIDTH};
use glium::texture::{RawImage2d, SrgbTexture2d};
use glium::uniforms::{MagnifySamplerFilter, Sampler};
use glium::{
    implement_vertex, index::PrimitiveType, uniform, Blend, Display, DrawParameters, IndexBuffer,
    Program, Surface, VertexBuffer,
};
use std::io::Cursor;

pub struct RenderSystem {
    display: Display,
    texture_atlas: SrgbTexture2d,
    program: Program,
    template_vertices: VertexBuffer<TemplateVertex>,
    indices: IndexBuffer<u8>,
}

impl RenderSystem {
    pub fn new(display: Display) -> Self {
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
            display,
            texture_atlas,
            program,
            template_vertices,
            indices,
        }
    }

    pub fn render(&self, context: &Context) {
        let mut draw_target = self.display.draw();
        draw_target.clear_color_srgb(0.50, 0.25, 0.12, 1.0);

        let instance_data = context
            .displayable_components
            .iter()
            .zip(context.position_components.iter())
            .filter_map(
                |((displayable_entity_key, displayable), (position_entity_key, position))| {
                    if displayable_entity_key == position_entity_key {
                        return Some(InstanceData {
                            tile_position: [position.x as f32, position.y as f32],
                            texture_atlas_index: displayable.texture_atlas_index as f32,
                        });
                    }
                    None
                },
            )
            .collect::<Vec<InstanceData>>();
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
            .expect("Draw call failed");

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

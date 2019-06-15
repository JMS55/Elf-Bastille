use crate::components::{Elf, Inventory, Location, LocationInfo, StorageInfo, Texture};
use crate::gui::GUI;
use crate::{NUMBER_OF_TEXTURES, VIEWPORT_SIZE};
use glium::index::PrimitiveType;
use glium::texture::{RawImage2d, SrgbTexture2d};
use glium::uniforms::{MagnifySamplerFilter, Sampler};
use glium::{
    implement_vertex, uniform, Blend, Depth, DepthTest, Display, DrawParameters, IndexBuffer,
    Program, Surface, VertexBuffer,
};
use rayon::iter::ParallelIterator;
use specs::{Entities, ParJoin, ReadStorage, System, WriteStorage};
use std::io::Cursor;

pub struct RenderSystem {
    pub camera_center: Location,
    pub gui: GUI,
    pub display: Display,
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

            in vec3 initial;
            in vec2 texture;
            in vec3 instance;
            in float texture_atlas_index;
            out vec2 v_texture;

            void main() {{
                v_texture = texture;
                v_texture.x += texture_atlas_index / {};
                vec3 position = initial + instance;
                gl_Position = vec4(position, 1.0);
            }}
        "#,
            NUMBER_OF_TEXTURES as f32
        );
        let fragment_shader_src = r#"
            #version 130

            uniform sampler2D texture_atlas;
            in vec2 v_texture;
            out vec4 pixel;

            void main() {
                pixel = texture(texture_atlas, v_texture);
                if (pixel.a < 0.5) {
                    discard;
                }
            }
        "#;
        let program = Program::from_source(&display, &vertex_shader_src, fragment_shader_src, None)
            .expect("Failed to create program");

        let template_vertices = VertexBuffer::new(
            &display,
            &[
                TemplateVertex {
                    initial: [1.0 / VIEWPORT_SIZE, 1.0 / VIEWPORT_SIZE, 0.0],
                    texture: [0.0, 1.0],
                },
                TemplateVertex {
                    initial: [-1.0 / VIEWPORT_SIZE, 1.0 / VIEWPORT_SIZE, 0.0],
                    texture: [1.0 / NUMBER_OF_TEXTURES as f32, 1.0],
                },
                TemplateVertex {
                    initial: [1.0 / VIEWPORT_SIZE, -1.0 / VIEWPORT_SIZE, 0.0],
                    texture: [0.0, 0.0],
                },
                TemplateVertex {
                    initial: [-1.0 / VIEWPORT_SIZE, -1.0 / VIEWPORT_SIZE, 0.0],
                    texture: [1.0 / NUMBER_OF_TEXTURES as f32, 0.0],
                },
            ],
        )
        .expect("Failed to create template vertex buffer");

        let indices = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0, 1, 3, 0, 2, 3])
            .expect("Failed to create template index buffer");

        Self {
            camera_center: Location::new(0, 0, 0),
            gui: GUI::new(&display),
            display,
            texture_atlas,
            program,
            template_vertices,
            indices,
        }
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        ReadStorage<'a, LocationInfo>,
        ReadStorage<'a, Texture>,
        ReadStorage<'a, Inventory>,
        ReadStorage<'a, StorageInfo>,
        WriteStorage<'a, Elf>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (location_data, texture_data, inventory_data,storage_info_data, mut elf_data, entities): Self::SystemData,
    ) {
        let mut draw_target = self.display.draw();
        draw_target.clear_color_srgb_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        let camera_center = self.camera_center;
        let instance_data = (&location_data, &texture_data)
            .par_join()
            .map(|(location_info, texture)| InstanceData {
                instance: [
                    2.0 * ((location_info.location.x - camera_center.x) as f32) / VIEWPORT_SIZE,
                    2.0 * ((location_info.location.y - camera_center.y) as f32) / VIEWPORT_SIZE,
                    (-location_info.location.z as f32) / VIEWPORT_SIZE,
                ],
                texture_atlas_index: texture.atlas_index as f32,
            })
            .collect::<Vec<InstanceData>>();
        let instances = VertexBuffer::new(&self.display, &instance_data)
            .expect("Failed to create vertex buffer");
        let draw_parameters = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: Blend::alpha_blending(),
            multisampling: false,
            dithering: false,
            ..Default::default()
        };
        draw_target
            .draw((&self.template_vertices, instances.per_instance().expect("Instancing not supported")),
                &self.indices,
                &self.program,
                &uniform!(texture_atlas: Sampler::new(&self.texture_atlas).magnify_filter(MagnifySamplerFilter::Nearest)),
                &draw_parameters,
            )
            .expect("World draw call failed");
        self.gui.render(
            &mut draw_target,
            &self.display,
            &inventory_data,
            &location_data,
            &storage_info_data,
            &mut elf_data,
            &entities,
        );
        draw_target
            .finish()
            .expect("Could not swap display buffers");
    }
}

#[derive(Copy, Clone)]
struct TemplateVertex {
    initial: [f32; 3],
    texture: [f32; 2],
}

#[derive(Copy, Clone)]
struct InstanceData {
    instance: [f32; 3],
    texture_atlas_index: f32,
}

implement_vertex!(TemplateVertex, initial, texture);
implement_vertex!(InstanceData, instance, texture_atlas_index);

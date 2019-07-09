use crate::components::{Position, Texture};
use glium::glutin::{ContextBuilder, EventsLoop, WindowBuilder};
use glium::index::PrimitiveType;
use glium::texture::{RawImage2d, SrgbTexture2d};
use glium::uniforms::{MagnifySamplerFilter, Sampler};
use glium::{
    implement_vertex, uniform, Display, DrawParameters, IndexBuffer, Program, Surface, VertexBuffer,
};
use rayon::iter::ParallelIterator;
use specs::{ParJoin, ReadStorage, System};
use std::io::Cursor;

const TEXTURE_ATLAS_SIZE: u32 = 11;
const TILE_SIZE_IN_PIXELS: u32 = 32;
const DEFAULT_NUMBER_OF_TILES_ON_SCREEN: u32 = 11;

pub struct RenderSystem<'dp> {
    pub camera_center: Position,
    pub camera_zoom_factor: u32,
    display: Display,
    program: Program,
    template_vertices: VertexBuffer<TemplateVertex>,
    indices: IndexBuffer<u8>,
    draw_parameters: DrawParameters<'dp>,
    texture_atlas: SrgbTexture2d,
}

impl<'dp> RenderSystem<'dp> {
    pub fn new(event_loop: &EventsLoop) -> Self {
        let primary_monitor = event_loop.get_primary_monitor();
        let dpi_factor = primary_monitor.get_hidpi_factor() as u32;
        let window = WindowBuilder::new().with_fullscreen(Some(primary_monitor));
        let context = ContextBuilder::new().with_vsync(true).with_srgb(true);
        let display = Display::new(window, context, &event_loop).expect("Could not create Display");

        let vertex_shader_src = r#""#;
        let fragment_shader_src = r#""#;
        let program = Program::from_source(&display, &vertex_shader_src, fragment_shader_src, None)
            .expect("Could not create Program");
        let template_vertices = VertexBuffer::new(
            &display,
            &[
                TemplateVertex {
                    vertex: [1.0, 1.0],
                    texture: [0.0, 1.0],
                },
                TemplateVertex {
                    vertex: [-1.0, 1.0],
                    texture: [1.0 / TEXTURE_ATLAS_SIZE as f32, 1.0],
                },
                TemplateVertex {
                    vertex: [1.0, -1.0],
                    texture: [0.0, 0.0],
                },
                TemplateVertex {
                    vertex: [-1.0, -1.0],
                    texture: [1.0 / TEXTURE_ATLAS_SIZE as f32, 0.0],
                },
            ],
        )
        .expect("Could not create template VertexBuffer");
        let indices = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0, 1, 3, 0, 2, 3])
            .expect("Could not create template IndexBuffer");
        let draw_parameters = DrawParameters {
            multisampling: false,
            dithering: false,
            ..Default::default()
        };

        let image = image::load(
            Cursor::new(&include_bytes!("../../texture_atlas.png")[..]),
            image::PNG,
        )
        .expect("Could not load texture atlas")
        .to_rgba();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture_atlas =
            SrgbTexture2d::new(&display, image).expect("Could not create texture atlas");

        let camera_center = Position { x: 0, y: 0 };
        let mut screen_size = display.get_framebuffer_dimensions();
        screen_size = (screen_size.0 / dpi_factor, screen_size.1 / dpi_factor);
        let camera_zoom_factor = screen_size
            .0
            .max(screen_size.1)
            .checked_div(DEFAULT_NUMBER_OF_TILES_ON_SCREEN * TILE_SIZE_IN_PIXELS)
            .unwrap_or(1);

        Self {
            camera_center,
            camera_zoom_factor,
            display,
            program,
            template_vertices,
            indices,
            draw_parameters,
            texture_atlas,
        }
    }

    pub fn zoom_camera(&mut self, in_or_out: bool) {
        if in_or_out {
            self.camera_zoom_factor = self.camera_zoom_factor.saturating_add(1);
        } else {
            self.camera_zoom_factor = self.camera_zoom_factor.saturating_sub(1);
        }
    }
}

impl<'dp, 's> System<'s> for RenderSystem<'dp> {
    type SystemData = (ReadStorage<'s, Texture>, ReadStorage<'s, Position>);

    fn run(&mut self, (texture_data, position_data): Self::SystemData) {
        let mut draw_target = self.display.draw();
        draw_target.clear_color_srgb(0.0, 0.0, 0.0, 1.0);
        let instance_data = (&texture_data, &position_data)
            .par_join()
            .map(|(texture, position)| InstanceData {
                position: [position.x as f32, position.y as f32],
                texture_atlas_index: texture.atlas_index as f32,
            })
            .collect::<Vec<InstanceData>>();
        let instances = VertexBuffer::new(&self.display, &instance_data)
            .expect("Could not create instance VertexBuffer");
        let uniforms = uniform! {
            texture_atlas: Sampler::new(&self.texture_atlas).magnify_filter(MagnifySamplerFilter::Nearest),
            camera_center: [self.camera_center.x as f32, self.camera_center.y as f32]
        };
        draw_target
            .draw(
                (
                    &self.template_vertices,
                    instances.per_instance().expect("Instancing not supported"),
                ),
                &self.indices,
                &self.program,
                &uniforms,
                &self.draw_parameters,
            )
            .expect("Could not submit draw call");
        draw_target.finish().expect("Could not draw frame");
    }
}

#[derive(Copy, Clone)]
struct TemplateVertex {
    vertex: [f32; 2],
    texture: [f32; 2],
}

#[derive(Copy, Clone)]
struct InstanceData {
    position: [f32; 2],
    texture_atlas_index: f32,
}

implement_vertex!(TemplateVertex, vertex, texture);
implement_vertex!(InstanceData, position, texture_atlas_index);

use crate::components::{Position, Texture};
use glium::glutin::dpi::LogicalPosition;
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

pub struct RenderSystem {
    camera_center: Position,
    camera_zoom_factor: u32,
    hovered_tile: Position,
    display: Display,
    program: Program,
    template_vertices: VertexBuffer<TemplateVertex>,
    indices: IndexBuffer<u8>,
    texture_atlas: SrgbTexture2d,
}

impl RenderSystem {
    pub fn new(event_loop: &EventsLoop) -> Self {
        let primary_monitor = event_loop.get_primary_monitor();
        let dpi_factor = primary_monitor.get_hidpi_factor();
        let window = WindowBuilder::new()
            .with_fullscreen(Some(primary_monitor))
            .with_decorations(false);
        let context = ContextBuilder::new().with_vsync(true).with_srgb(true);
        let display = Display::new(window, context, &event_loop).expect("Could not create Display");

        let vertex_shader_src = format!(
            r#"
            #version 130

            uniform vec2  screen_size;
            uniform vec2  camera_center;
            uniform float camera_zoom_factor;
            in vec2  t_vertex;
            in vec2  t_texture;
            in vec2  i_position;
            in float i_texture_atlas_index;
            out vec2 v_texture;

            void main() {{
                v_texture = t_texture;
                v_texture.x += i_texture_atlas_index / {};
                gl_Position = vec4(t_vertex + i_position - camera_center, 0.0, 1.0);
                gl_Position.x = gl_Position.x * {} * 2 * (camera_zoom_factor / screen_size.x);
                gl_Position.y = gl_Position.y * {} * 2 * (camera_zoom_factor / screen_size.y);
            }}
        "#,
            TEXTURE_ATLAS_SIZE as f32, TILE_SIZE_IN_PIXELS as f32, TILE_SIZE_IN_PIXELS as f32,
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
            .expect("Could not create Program");
        let template_vertices = VertexBuffer::new(
            &display,
            &[
                TemplateVertex {
                    t_vertex: [0.5, 0.5],
                    t_texture: [0.0, 1.0],
                },
                TemplateVertex {
                    t_vertex: [-0.5, 0.5],
                    t_texture: [1.0 / TEXTURE_ATLAS_SIZE as f32, 1.0],
                },
                TemplateVertex {
                    t_vertex: [0.5, -0.5],
                    t_texture: [0.0, 0.0],
                },
                TemplateVertex {
                    t_vertex: [-0.5, -0.5],
                    t_texture: [1.0 / TEXTURE_ATLAS_SIZE as f32, 0.0],
                },
            ],
        )
        .expect("Could not create template VertexBuffer");
        let indices = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0, 1, 3, 0, 2, 3])
            .expect("Could not create template IndexBuffer");

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
        screen_size = (
            (screen_size.0 as f64 / dpi_factor) as u32,
            (screen_size.1 as f64 / dpi_factor) as u32,
        );
        let camera_zoom_factor = screen_size
            .0
            .max(screen_size.1)
            .checked_div(20 * TILE_SIZE_IN_PIXELS)
            .unwrap_or(1);

        Self {
            camera_center,
            camera_zoom_factor,
            hovered_tile: Position { x: 0, y: 0 },
            display,
            program,
            template_vertices,
            indices,
            texture_atlas,
        }
    }

    pub fn move_camera(&mut self, x_offset: i32, y_offset: i32, cursor_position: LogicalPosition) {
        self.camera_center.x += x_offset;
        self.camera_center.y += y_offset;
        self.set_hovered_tile(cursor_position);
    }

    pub fn zoom_camera(&mut self, in_or_out: bool, cursor_position: LogicalPosition) {
        if in_or_out && self.camera_zoom_factor != u32::max_value() {
            self.camera_zoom_factor += 1;
        }
        if !in_or_out && self.camera_zoom_factor != 1 {
            self.camera_zoom_factor -= 1;
        }
        self.set_hovered_tile(cursor_position);
    }

    pub fn set_hovered_tile(&mut self, cursor_position: LogicalPosition) {
        let screen_size = self.display.get_framebuffer_dimensions();
        let tile_size = (TILE_SIZE_IN_PIXELS * self.camera_zoom_factor) as f64;

        let h_tiles_across = screen_size.0 as f64 / tile_size;
        let h_tiles_per_side = (h_tiles_across - 1.0) / 2.0;
        let h_pixels_per_side = h_tiles_per_side * tile_size;
        self.hovered_tile.x = ((cursor_position.x - h_pixels_per_side) / tile_size).floor() as i32
            + self.camera_center.x;

        let v_tiles_across = screen_size.1 as f64 / tile_size;
        let v_tiles_per_side = (v_tiles_across - 1.0) / 2.0;
        let v_pixels_per_side = v_tiles_per_side * tile_size;
        self.hovered_tile.y = ((cursor_position.y - v_pixels_per_side) / tile_size).floor() as i32
            * -1
            + self.camera_center.y;
    }
}

impl<'s> System<'s> for RenderSystem {
    type SystemData = (ReadStorage<'s, Texture>, ReadStorage<'s, Position>);

    fn run(&mut self, (texture_data, position_data): Self::SystemData) {
        let mut draw_target = self.display.draw();
        draw_target.clear_color_srgb(0.0, 0.0, 0.0, 1.0);
        let instance_data = (&texture_data, &position_data)
            .par_join()
            .map(|(texture, position)| InstanceData {
                i_position: [position.x as f32, position.y as f32],
                i_texture_atlas_index: texture.atlas_index as f32,
            })
            .collect::<Vec<InstanceData>>();
        let instances = VertexBuffer::new(&self.display, &instance_data)
            .expect("Could not create instance VertexBuffer");
        let screen_size = self.display.get_framebuffer_dimensions();
        let uniforms = uniform! {
            screen_size: [screen_size.0 as f32, screen_size.1 as f32],
            camera_center: [self.camera_center.x as f32, self.camera_center.y as f32],
            camera_zoom_factor: self.camera_zoom_factor as f32,
            texture_atlas: Sampler::new(&self.texture_atlas).magnify_filter(MagnifySamplerFilter::Nearest),
        };
        let draw_parameters = DrawParameters {
            multisampling: false,
            dithering: false,
            ..Default::default()
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
                &draw_parameters,
            )
            .expect("Could not submit draw call");
        draw_target.finish().expect("Could not draw frame");
    }
}

#[derive(Copy, Clone)]
struct TemplateVertex {
    t_vertex: [f32; 2],
    t_texture: [f32; 2],
}

#[derive(Copy, Clone)]
struct InstanceData {
    i_position: [f32; 2],
    i_texture_atlas_index: f32,
}

implement_vertex!(TemplateVertex, t_vertex, t_texture);
implement_vertex!(InstanceData, i_position, i_texture_atlas_index);

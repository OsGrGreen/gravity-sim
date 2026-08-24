use glium::{Display, DrawParameters, IndexBuffer, Program, VertexBuffer, framebuffer::SimpleFrameBuffer, glutin::surface::WindowSurface, program::TransformFeedbackMode};

use crate::rendering::{render::Vertex, render_camera::RenderCamera};

pub mod render;
pub mod render_camera;
mod char_font_pos;
pub mod text;


pub struct RenderContext<'a> {
    pub framebuffer: &'a mut SimpleFrameBuffer<'a>,
    pub camera: &'a RenderCamera,
    pub time: f32,
}

pub struct Mesh {
    pub vertices: VertexBuffer<Vertex>,
    // How to be rendered (Indicies) (Kan behöva öka från u16 till u32)
    pub indices: IndexBuffer<u16>,
}

impl Mesh{
    pub fn new(vertices: VertexBuffer<Vertex>, indices: IndexBuffer<u16>) -> Mesh {
        Mesh { vertices, indices }
    }

    pub fn create(shape: &Vec<Vertex>, inds: &Vec<u16>, disp: &Display<WindowSurface>) -> Mesh {
        let vertices = glium::VertexBuffer::new(disp, &shape).unwrap();
        let indices = glium::IndexBuffer::new(disp,glium::index::PrimitiveType::TrianglesList, inds).expect("Could not create");
        Mesh{
            vertices,
            indices
        }
    }
}

pub struct Material {
    pub program: Program,
    pub draw_params: DrawParameters<'static>,
}

impl Material{
    pub fn new(program: Program, draw_params: DrawParameters<'static>) -> Material {
        Material { program, draw_params }
    }

    pub fn create<'a>(vert_shader: &'a str, frag_shader: &'a str, geo_shader: Option<&'a str>, tess_ctrl: Option<&'a str>, tess_eval: Option<&'a str>, disp: &Display<WindowSurface>, params: Option<DrawParameters<'static>>, transform_feedback:Option<(Vec<std::string::String>, TransformFeedbackMode)>) -> Material {
        let program = glium::Program::new(disp, glium::program::ProgramCreationInput::SourceCode{
                vertex_shader: vert_shader,
                fragment_shader: frag_shader,
                geometry_shader: geo_shader,
                tessellation_control_shader: tess_ctrl,
                tessellation_evaluation_shader: tess_eval,
                transform_feedback_varyings: transform_feedback,
                outputs_srgb: true,
                uses_point_size: false,
            }).expect("Failed to create program");
        let draw_params = params.unwrap_or_default();
        Material { program, draw_params }
    }
}
use std::{any::Any, println};

use glam::{Mat4, Vec3};
use glium::{Display, IndexBuffer, Surface, Texture2d, Vertex, VertexBuffer, framebuffer::SimpleFrameBuffer, glutin::surface::WindowSurface, index::{IndicesSource, PrimitiveType}, uniforms::MagnifySamplerFilter, vertex::VerticesSource};

use crate::{managers::{RenderManager, handles::{MaterialHandle, MeshHandle, TextureHandle}}, rendering::{RenderContext, render::{Renderer, VertexSimple}, render_camera::RenderCamera, text}, scene::objects::transform::Transform, spline::Spline};
pub mod point;


pub trait Renderable {
    fn render(
        &self,
        transform: &Transform,
        context: &mut RenderContext,
        manager: &RenderManager,
    );

    fn as_any(&mut self) -> &mut dyn Any;

    fn id(&self) -> String;
} 

//I should probably make the renderobjects save the VBO and indicies and not the renderer...
#[derive(Debug)]
pub struct RenderObject<>{
    pub render_id: Option<String>,
    instanced_vbo: Option<VertexBuffer<VertexSimple>>
}

pub struct TextureMeshRenderer {
    pub render_id: String,
    pub texture: TextureHandle
}

impl  TextureMeshRenderer {
    pub fn new(id: String, texture: TextureHandle) -> TextureMeshRenderer{
        TextureMeshRenderer { render_id: id, texture}
    }
}

impl Renderable for TextureMeshRenderer {
    fn render(
        &self,
        transform: &Transform,
        context: &mut RenderContext,
        manager: &RenderManager,
    ) {
        let possible_renderer = manager.renderers.get(&self.render_id);
        if let Some(renderer) = possible_renderer {
            let found_material = manager.materials.get(&renderer.material);
            let found_mesh = manager.meshes.get(&renderer.mesh);
            // Not used right now since I can not mutate the RenderManager...
            if manager.last_used_texture.is_some_and(|x| x == self.texture) {
                if let Some((material, mesh)) = found_material.zip(found_mesh) {
                    let fbo = &mut context.framebuffer;
                    let camera = context.camera;
                    fbo.draw(&mesh.vertices, &mesh.indices, &material.program, &uniform! {projection: camera.perspective.to_cols_array_2d(), view: camera.getMatrix(), model: transform.model_matrix().to_cols_array_2d()}, &material.draw_params).unwrap();
                } else {
                    println!("Did not found mesh or material");
                }
            } else {
                let found_texture = manager.textures.get(&self.texture);
                if let Some(((material, mesh), texture)) = found_material.zip(found_mesh).zip(found_texture) {
                    let fbo = &mut context.framebuffer;
                    let camera = context.camera;
                    fbo.draw(&mesh.vertices, &mesh.indices, &material.program, &uniform! {tex: texture.sampled().magnify_filter(MagnifySamplerFilter::Nearest), projection: camera.perspective.to_cols_array_2d(), view: camera.getMatrix(), model: transform.model_matrix().to_cols_array_2d()}, &material.draw_params).unwrap();
                } else {
                    println!("Did not found mesh or material");
                }
            }
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn id(&self) -> String {
        self.render_id.clone()
    }
}

pub struct MeshRenderer {
    pub render_id: String,
}

impl MeshRenderer {
    pub fn new(id: String) -> MeshRenderer {
        MeshRenderer { render_id: id }
    }
}

impl Renderable for MeshRenderer {
    fn render(
        &self,
        transform: &Transform,
        context: &mut RenderContext,
        manager: &RenderManager,
    ) {
        let possible_renderer = manager.renderers.get(&self.render_id);
        if let Some(renderer) = possible_renderer {
            let found_material = manager.materials.get(&renderer.material);
            let found_mesh = manager.meshes.get(&renderer.mesh);

            if let Some((material, mesh)) = found_material.zip(found_mesh) {
                let fbo = &mut context.framebuffer;
                let camera = context.camera;
                fbo.draw(&mesh.vertices, &mesh.indices, &material.program, &uniform! {projection: camera.perspective.to_cols_array_2d(), view: camera.getMatrix(), model: transform.model_matrix().to_cols_array_2d()}, &material.draw_params).unwrap();
            }else {
                println!("Did not found mesh or material");
            }
        }
    }
    
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn id(&self) -> String {
        self.render_id.clone()
    }
}


pub struct SplineRenderer {
    pub render_id: String,
    vertices: VertexBuffer<VertexSimple>,
    indices: IndexBuffer<u16>,
}

impl SplineRenderer {
    pub fn new(id: String, spline: &Spline, manager: &mut RenderManager, display: &Display<WindowSurface>, ) -> SplineRenderer {
        let (vertices, indices, renderer) = spline.spline_renderer(display, manager);
        manager.add_renderer(id.clone(), renderer);
        SplineRenderer {
            render_id: id,
            vertices,
            indices,
        }
    }

    pub fn update(&mut self, spline: &Spline){
        self.vertices.write(&spline.to_vertex());
        self.indices.write(&spline.get_indicies());
    }
}

impl Renderable for SplineRenderer{
    fn render(
        &self,
        transform: &Transform,
        context: &mut RenderContext,
        manager: &RenderManager,
    ) {
        let possible_renderer = manager.renderers.get(&self.render_id);
        if let Some(renderer) = possible_renderer {
            let possible_material = manager.materials.get(&renderer.material);
            if let Some(material) = possible_material {
                let fbo = &mut context.framebuffer;
                let camera = context.camera;
                fbo.draw(&self.vertices, &self.indices, &material.program, &uniform! {u_screenSize: [640, 360], u_thickness: 50.0 as f32, steps: 48.0 as f32, model: transform.model_matrix().to_cols_array_2d(), projection: camera.perspective.to_cols_array_2d(), view:camera.getMatrix()}, &material.draw_params).unwrap();
            }
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn id(&self) -> String {
        self.render_id.clone()
    }
}

pub struct LineRenderer<'a> {
    pub render_id: String,
    pub point_a: &'a Transform,
    pub point_b: &'a Transform
}


/*impl RenderObject<>{

    //We can make this quicker by removing some unneccessary draw calls. 
    //If we have already passed that uniform to the GPU once it will reuse it until it gets a new one (by providing it here)
    pub fn draw(&mut self, transform: Transform, fbo: &mut SimpleFrameBuffer::<'_>, camera: &RenderCamera, renderer: &Renderer, time: f32) -> () {
            if self.instanced_vbo.is_none(){
                fbo.draw(&renderer.vbo, &renderer.indicies, &renderer.program, &uniform! {projection: camera.perspective.to_cols_array_2d(), view: camera.getMatrix(), model: transform.model_matrix().to_cols_array_2d()}, &renderer.draw_params).unwrap();
            }else{
                fbo.draw((&renderer.vbo,self.instanced_vbo.as_ref().unwrap().per_instance().unwrap()), &renderer.indicies, &renderer.program, &uniform! {projection: camera.perspective.to_cols_array_2d(), view: camera.getMatrix(), model: transform.model_matrix().to_cols_array_2d(), u_time: time}, &renderer.draw_params).unwrap();
            }
    }

    pub fn draw_with_texture(&mut self, transform: Transform, fbo: &mut SimpleFrameBuffer::<'_>, camera: &RenderCamera, renderer: &Renderer,texture: &Texture2d, time: f32) -> () {
        if self.instanced_vbo.is_none(){
            fbo.draw(&renderer.vbo, &renderer.indicies, &renderer.program, &uniform! {tex: texture.sampled().magnify_filter(MagnifySamplerFilter::Nearest), projection: camera.perspective.to_cols_array_2d(), view: camera.getMatrix(), model: transform.model_matrix().to_cols_array_2d(), u_time: time}, &renderer.draw_params).unwrap();
        }else{
            fbo.draw((&renderer.vbo,self.instanced_vbo.as_ref().unwrap().per_instance().unwrap()), &renderer.indicies, &renderer.program, &uniform! {tex: texture,projection: camera.perspective.to_cols_array_2d(), view: camera.getMatrix(), model: transform.model_matrix().to_cols_array_2d()}, &renderer.draw_params).unwrap();
        }
    }

    
    pub fn new(render_id: Option<String>) -> RenderObject {
        RenderObject{
            render_id: render_id,
            instanced_vbo: None
        }
    }

}*/


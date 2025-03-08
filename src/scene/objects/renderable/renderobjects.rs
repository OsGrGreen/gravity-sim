use glam::Vec3;
use glium::{framebuffer::SimpleFrameBuffer, Surface, Texture2d, Vertex, VertexBuffer};

use crate::rendering::{render::{Renderer, VertexSimple}, render_camera::RenderCamera};

use super::ModelObject;


//I should probably make the renderobjects save the VBO and indicies and not the renderer...
#[derive(Debug)]
pub struct RenderObject<>{
    pub render_id: Option<String>,
    pub model_object: ModelObject,
    instanced_vbo: Option<VertexBuffer<VertexSimple>>
}

impl RenderObject<>{

    //We can make this quicker by removing some unneccessary draw calls. 
    //If we have already passed that uniform to the GPU once it will reuse it until it gets a new one (by providing it here)
    pub fn draw(&mut self, fbo: &mut SimpleFrameBuffer::<'_>, camera: &RenderCamera, renderer: &Renderer) -> () {
            if self.instanced_vbo.is_none(){
                fbo.draw(&renderer.vbo, &renderer.indicies, &renderer.program, &uniform! {projection: camera.perspective.to_cols_array_2d(), view: camera.camera_matrix.to_cols_array_2d(), model: self.model_object.get_model().to_cols_array_2d()}, &renderer.draw_params).unwrap();
            }else{
                fbo.draw((&renderer.vbo,self.instanced_vbo.as_ref().unwrap().per_instance().unwrap()), &renderer.indicies, &renderer.program, &uniform! {projection: camera.perspective.to_cols_array_2d(), view: camera.camera_matrix.to_cols_array_2d(), model: self.model_object.get_model().to_cols_array_2d()}, &renderer.draw_params).unwrap();
            }
    }

    pub fn draw_with_texture(&mut self, fbo: &mut SimpleFrameBuffer::<'_>, camera: &RenderCamera, renderer: &Renderer,texture: &Texture2d) -> () {
        if self.instanced_vbo.is_none(){
            fbo.draw(&renderer.vbo, &renderer.indicies, &renderer.program, &uniform! {tex: texture, projection: camera.perspective.to_cols_array_2d(), view: camera.camera_matrix.to_cols_array_2d(), model: self.model_object.get_model().to_cols_array_2d()}, &renderer.draw_params).unwrap();
        }else{
            fbo.draw((&renderer.vbo,self.instanced_vbo.as_ref().unwrap().per_instance().unwrap()), &renderer.indicies, &renderer.program, &uniform! {tex: texture,projection: camera.perspective.to_cols_array_2d(), view: camera.camera_matrix.to_cols_array_2d(), model: self.model_object.get_model().to_cols_array_2d()}, &renderer.draw_params).unwrap();
        }
    }

    
    pub fn new(render_id: Option<String>) -> RenderObject {
        RenderObject{
            render_id: render_id,
            model_object: ModelObject::new(),
            instanced_vbo: None
        }
    }

}


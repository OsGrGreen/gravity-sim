use std::println;

use glam::Vec3;
use glium::{framebuffer::SimpleFrameBuffer, uniforms::MagnifySamplerFilter, Surface, Texture2d, Vertex, VertexBuffer};

use crate::{assetmanager::{RenderManager, handles::{MaterialHandle, MeshHandle}}, rendering::{RenderContext, render::{Renderer, VertexSimple}, render_camera::RenderCamera}, scene::objects::transform::Transform};

pub mod point;


pub trait Renderable {
    fn render(
        &self,
        transform: &Transform,
        context: &mut RenderContext,
        manager: &RenderManager,
    );
}

//I should probably make the renderobjects save the VBO and indicies and not the renderer...
#[derive(Debug)]
pub struct RenderObject<>{
    pub render_id: Option<String>,
    instanced_vbo: Option<VertexBuffer<VertexSimple>>
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

            //let texture = manager.textures[self.texture]
            //if let Some(texture) = &self.material.texture {
                //fbo.draw(&renderer.vbo, &renderer.indicies, &renderer.program, &uniform! {tex: texture.sampled().magnify_filter(MagnifySamplerFilter::Nearest), projection: camera.perspective.to_cols_array_2d(), view: camera.getMatrix(), model: transform.model_matrix().to_cols_array_2d()}, &renderer.draw_params).unwrap();
            //} else {
            if let Some((material, mesh)) = found_material.zip(found_mesh) {
                let fbo = &mut context.framebuffer;
                let camera = context.camera;
                fbo.draw(&mesh.vertices, &mesh.indices, &material.program, &uniform! {projection: camera.perspective.to_cols_array_2d(), view: camera.getMatrix(), model: transform.model_matrix().to_cols_array_2d()}, &material.draw_params).unwrap();
            }else {
                println!("Did not found mesh or material");
            }
        }
        //}
    }
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


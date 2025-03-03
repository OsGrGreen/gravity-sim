pub mod physics;
pub mod renderable;

use glium::framebuffer::SimpleFrameBuffer;
use renderable::renderobjects::RenderObject;
use physics::PhysicsObject;

use crate::rendering::{render::Renderer, render_camera::RenderCamera};

pub struct WorldObject{
    pub render_object: RenderObject,
    physics_object: Box<dyn PhysicsObject>
}

impl WorldObject{
    pub fn new(render: RenderObject, physics: Box<dyn PhysicsObject>) -> WorldObject{
        WorldObject{
            render_object: render,
            physics_object: physics,
        }
    } 

   pub fn draw(&mut self, fbo: &mut SimpleFrameBuffer<'_>, camera: &RenderCamera, renderer: &Renderer){
    self.render_object.draw(fbo, camera, renderer);
   }

   pub fn update_physics(&mut self, dt: f32){

   }
}
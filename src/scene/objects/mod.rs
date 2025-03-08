pub mod physics;
pub mod renderable;

use std::fmt::Debug;

use glam::Vec3;
use glium::{framebuffer::SimpleFrameBuffer, Texture2d};
use renderable::renderobjects::RenderObject;
use physics::PhysicsObject;

use crate::rendering::{render::Renderer, render_camera::RenderCamera};

pub struct WorldObject{
    pub id: String,
    pub render_object: RenderObject,
    pub physics_object: Box<dyn PhysicsObject>
}

impl WorldObject{
    pub fn new(name: String, render: RenderObject, physics: Box<dyn PhysicsObject>) -> WorldObject{
        let mut wo = WorldObject{
            id: name,
            render_object: render,
            physics_object: physics,
        };
        wo.physics_object.init(&mut wo.render_object);
        return wo;
    } 

   pub fn draw(&mut self, fbo: &mut SimpleFrameBuffer<'_>, camera: &RenderCamera, renderer: &Renderer, texture: &mut Option<&Texture2d>){
    if texture.is_none(){
        self.render_object.draw(fbo, camera, renderer);
    }else{
        self.render_object.draw_with_texture(fbo, camera, renderer, texture.unwrap());
    }
   }

   pub fn update_physics(&mut self, dt: f32){
    self.physics_object.update_physics(dt, &mut self.render_object);
   }

   pub fn distance(&self, obj: &WorldObject) -> (Vec3,f32){
        (obj.render_object.model_object.get_posistion()-self.render_object.model_object.get_posistion(),self.render_object.model_object.get_posistion().distance(obj.render_object.model_object.get_posistion()))
   }
}

impl Debug for WorldObject{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorldObject").field("render_object", &self.render_object).finish()
    }
}
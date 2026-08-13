pub mod physics;
pub mod renderable;
pub mod transform;
pub mod colliders;

use glam::Vec3;
use glium::{framebuffer::SimpleFrameBuffer, Texture2d};
use renderable::RenderObject;
use physics::PhysicsObject;

use crate::{assetmanager::RenderManager, rendering::{RenderContext, render::Renderer, render_camera::RenderCamera}, scene::objects::{colliders::Collider, renderable::Renderable, transform::Transform}};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ObjectId {
    pub index: usize,
}

impl ObjectId{
    pub fn new(index: usize) -> ObjectId{
        return ObjectId { index:index }
    }
}

pub struct WorldObject{
    pub id: ObjectId,
    pub transform: Transform,
    pub render: Option<Box<dyn Renderable>>,
    pub physics: PhysicsObject,
    pub collider: Option<Collider>,
}

impl WorldObject{
    pub fn new(index: usize, transform: Transform, render: Option<Box<dyn Renderable>>, physics: PhysicsObject, collider: Option<Collider>) -> WorldObject{
        let mut wo = WorldObject{
            id: ObjectId { index },
            render,
            physics,
            transform,
            collider
        };
        return wo;
    } 

   pub fn draw(&mut self, context: &mut RenderContext, asset_manager: &RenderManager){
        if let Some(renderable) = &self.render {
            renderable.render(&self.transform, context, asset_manager);
        }
        //self.render.draw(self.transform, fbo, camera, renderer, time);
        //self.render.draw_with_texture(self.transform, fbo, camera, renderer, texture.unwrap(), time);
   }

   pub fn collides(&self, obj2: &WorldObject) -> bool{
    if let Some(unwrapped_collider) = &self.collider {
        unwrapped_collider.collides(&obj2.collider)
    } else {
        false
    }
   }

   

   pub fn update_physics(&mut self, dt: f32){
        self.physics.update_physics(dt, &mut self.transform);
   }

   pub fn distance(&self, obj: &WorldObject) -> (Vec3,f32){
        (obj.transform.position-self.transform.position,self.transform.position.distance(obj.transform.position))
   }

}

/*impl std::fmt::Debug for WorldObject {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_struct("WorldObject")
            .field("Transform", &self.transform)
            .finish()
    }
}*/
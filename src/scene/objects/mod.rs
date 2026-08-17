pub mod physics;
pub mod renderable;
pub mod transform;
pub mod colliders;

use glam::Vec3;
use glium::{framebuffer::SimpleFrameBuffer, Texture2d};
use renderable::RenderObject;
use physics::PhysicsObject;

use crate::{assetmanager::RenderManager, rendering::{RenderContext, render::Renderer, render_camera::RenderCamera}, scene::objects::{colliders::Collider, renderable::{LineRenderer, Renderable, SplineRenderer}, transform::Transform}, spline::Spline};



pub enum SceneObject{
    World(WorldObject),
    Spline(SplineObject)
}

impl SceneObject {
    pub fn id(&self) -> ObjectId {
        match self {
            SceneObject::World(o) => o.data.id,
            SceneObject::Spline(o) => o.data.id,
        }
    }

    pub fn transform(&self) -> &Transform {
        match self {
            SceneObject::World(o) => &o.data.transform,
            SceneObject::Spline(o) => &o.data.transform,
        }
    }

    pub fn update_physics(&mut self, dt: f32) {
        match self {
            SceneObject::World(obj) => {
                obj.update_physics(dt);
            }
            _ => {
                // Does not have physics
            }
        }
    }

    pub fn draw(&mut self, context: &mut RenderContext, asset_manager: &RenderManager) {
        match self {
            SceneObject::World(o) => o.draw(context, asset_manager),
            SceneObject::Spline(o) => o.draw(context, asset_manager),
        }
    }

    pub fn distance(&self, other: &SceneObject) -> (Vec3,f32){
        (other.transform().position-self.transform().position,self.transform().position.distance(other.transform().position))
    }

    pub fn distance_point(&self, other: Vec3) -> (Vec3,f32){
        (other-self.transform().position,self.transform().position.distance(other))
    }
}

pub trait SceneObjectBehaviour {
    fn data(&self) -> &ObjectData;
    fn data_mut(&mut self) -> &mut ObjectData;
}

impl SceneObjectBehaviour for SceneObject {
    fn data(&self) -> &ObjectData {
        match self {
            SceneObject::World(o) => &o.data,
            SceneObject::Spline(o) => &o.data,
        }
    }

    fn data_mut(&mut self) -> &mut ObjectData {
        match self {
            SceneObject::World(o) => &mut o.data,
            SceneObject::Spline(o) => &mut o.data,
        }
    }
}

impl SceneObjectBehaviour for WorldObject {
    fn data(&self) -> &ObjectData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut ObjectData {
        &mut self.data
    }
}

impl SceneObjectBehaviour for SplineObject {
    fn data(&self) -> &ObjectData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut ObjectData {
        &mut self.data
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ObjectId {
    pub index: usize,
}

impl ObjectId{
    pub fn new(index: usize) -> ObjectId{
        return ObjectId { index:index }
    }
}

pub struct ObjectData {
    pub id: ObjectId,
    pub transform: Transform,
    pub render: Option<Box<dyn Renderable>>, // Should probably change such that the SplineObject only has a SplineRenderer and not anything else...
}

pub struct SplineObject {
    pub data: ObjectData,
    pub spline: Spline, 
}

impl SplineObject {
    pub fn new(index: usize, spline: Spline,transform: Transform, render: Option<Box<dyn Renderable>>) -> SplineObject{
        let data = ObjectData{
            id: ObjectId { index }, transform, render,
        };
        let wo = SplineObject{
            data,
            spline,
        };
        return wo;
    } 

    pub fn draw(&mut self, context: &mut RenderContext, asset_manager: &RenderManager){
        if let Some(renderable) = &mut self.data.render {
            let spline_renderable = renderable
                .as_any()
                .downcast_mut::<SplineRenderer>()
                .expect("Expected SplineRenderer");
            spline_renderable.update(&self.spline);
            renderable.render(&self.data.transform, context, asset_manager);
        }
   }

}

pub struct WorldObject{
    pub data: ObjectData,
    pub physics: PhysicsObject,
    pub collider: Option<Collider>,
}

impl WorldObject{
    pub fn new(index: usize, transform: Transform, render: Option<Box<dyn Renderable>>, physics: PhysicsObject, collider: Option<Collider>) -> WorldObject{
        let data = ObjectData{
            id: ObjectId { index }, transform, render,
        };
        let wo = WorldObject{
            data,
            physics,
            collider
        };
        return wo;
    } 

   pub fn draw(&mut self, context: &mut RenderContext, asset_manager: &RenderManager){
        if let Some(renderable) = &self.data.render {
            renderable.render(&self.data.transform, context, asset_manager);
        }
   }

   pub fn collides(&self, obj2: &WorldObject) -> bool{
    if let Some(unwrapped_collider) = &self.collider {
        unwrapped_collider.collides(&obj2.collider)
    } else {
        false
    }
   }

   pub fn update_physics(&mut self, dt: f32){
        self.physics.update_physics(dt, &mut self.data.transform);
   }

   pub fn distance(&self, obj: &WorldObject) -> (Vec3,f32){
        (obj.data.transform.position-self.data.transform.position,self.data.transform.position.distance(obj.data.transform.position))
   }

    pub fn distance_point(&self, other: Vec3) -> (Vec3,f32){
        (other-self.data.transform.position,self.data.transform.position.distance(other))
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
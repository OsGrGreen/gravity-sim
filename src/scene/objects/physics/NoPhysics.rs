use std::any::Any;

use glam::Vec3;

use crate::scene::objects::{renderable::renderobjects::RenderObject, WorldObject};

use super::PhysicsObject;

pub struct NoPhysics{

}

impl PhysicsObject for NoPhysics{
    fn update_physics(&mut self, dt:f32, model: &mut RenderObject) {

    }
    
    fn get_collision(&self) {
        todo!()
    }

    fn set_force(&mut self, force: Vec3) {

    }
    
    fn add_force(&mut self, force: Vec3) {
    }
    
    fn init(&self, model: &mut RenderObject) {
    }
    
    fn get_mass(&self) -> f32 {
        0.0
    }

    fn set_init_velocity(&mut self, vel: Vec3){
    }
    
    fn collides(&self, dist: f32, obj2: &WorldObject)-> bool {
        false
    }
    
    fn get_size(&self) -> f32 {
        0.0
    }
}

impl NoPhysics {
    pub fn new() -> NoPhysics {
        NoPhysics { }
    }
}
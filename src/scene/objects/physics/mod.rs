use glam::Vec3;
use sphere::Sphere;

use super::{renderable::renderobjects::RenderObject, WorldObject};

pub mod sphere;
pub mod controllers;

pub trait PhysicsObject{
    fn update_physics(&mut self, dt:f32, model: &mut RenderObject);
    fn init(&self, model: &mut RenderObject);
    fn get_collision(&self);
    fn add_force(&mut self, force: Vec3);
    fn set_force(&mut self, force: Vec3);
    fn get_mass(&self) -> f32;
    fn set_init_velocity(&mut self, vel: Vec3);
    fn collides(&self, dist: f32, obj2: &WorldObject) -> bool;
    fn get_size(&self) -> f32;
}


pub fn physics_object_factory(id: usize, data: Vec<f32>) -> Box<dyn PhysicsObject> {
    match id {
        0 => {assert!(data.len() == 2); Box::new(Sphere::new(data))}, 
        _ => panic!("Unknown shape"),
    }
}
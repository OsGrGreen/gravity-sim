use std::any::Any;

use glam::Vec3;

use crate::scene::objects::{renderable::renderobjects::RenderObject, WorldObject};

use super::PhysicsObject;

pub struct Sphere{
    radius: f32,
    mass: f32,
    velocity: Vec3,
    acceleration: Vec3
}

impl PhysicsObject for Sphere{
    fn update_physics(&mut self, dt:f32, model: &mut RenderObject) {
        self.velocity += self.acceleration;
        model.model_object.translate(dt*self.velocity);
        self.acceleration = Vec3::ZERO;
        //model.model_object.rotate(Vec3::new(0.0, 1.0, 0.0), dt/self.radius);
    }
    
    fn get_collision(&self) {
        todo!()
    }

    fn set_force(&mut self, force: Vec3) {
        self.velocity = force;
    }
    
    fn add_force(&mut self, force: Vec3) {
        self.acceleration += force/self.mass;
    }
    
    fn init(&self, model: &mut RenderObject) {
        model.model_object.scale(Vec3::new(self.radius, self.radius, self.radius));
    }
    
    fn get_mass(&self) -> f32 {
        self.mass
    }

    fn set_init_velocity(&mut self, vel: Vec3){
        self.velocity = vel;
    }
    
    fn collides(&self, dist: f32, obj2: &WorldObject)-> bool {
        if dist <= self.radius+obj2.physics_object.get_size(){
            return true
        }
        return false;
    }
    
    fn get_size(&self) -> f32 {
        self.radius
    }
}

impl Sphere {
    pub fn new(data: Vec<f32>) -> Self {
        Sphere { radius:data[0], mass:data[1], velocity:Vec3::ZERO, acceleration: Vec3::ZERO}
    }

}
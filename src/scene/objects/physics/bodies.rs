use glam::Vec3;

use crate::scene::objects::transform::Transform;

use super::PhysicsObject;

#[derive(Clone, Debug)]
pub struct RigidBody{
    pub mass: f32,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub force: Vec3,
    pub activated: bool,
}

impl RigidBody {

    pub fn new(mass: f32) -> Self {
        RigidBody { mass: mass, velocity: Vec3::ZERO, acceleration: Vec3::ZERO, force: Vec3::ZERO, activated: true}
    }

    pub fn new_data(data: Vec<f32>) -> Self {
        match data.len() {
            0 => panic!("RigidBody::new requires at least one value"),
            4 => RigidBody { mass: data[0], velocity: Vec3 { x: data[1], y: data[2], z: data[3] }, acceleration: Vec3::ZERO, force: Vec3::ZERO , activated: true},
            7 => RigidBody { mass: data[0], velocity: Vec3 { x: data[1], y: data[2], z: data[3] }, acceleration: Vec3 { x: data[4], y: data[5], z: data[6] }, force: Vec3::ZERO , activated: true},
            _ => RigidBody { mass: data[0], velocity: Vec3::ZERO, acceleration: Vec3::ZERO, force: Vec3::ZERO, activated: true }
        }
    }


    pub fn update_physics(&mut self, dt: f32, transform: &mut Transform,) {
        if self.activated {
            println!("Force is: {:?}", self.force);
            let acceleration = self.force / self.mass;
            
            self.velocity += acceleration * dt;
            transform.position += self.velocity * dt;

            self.force = Vec3::ZERO;
        }
    }
}

#[derive(Clone, Debug)]
pub struct StaticBody{
    pub mass: f32,
}

impl StaticBody {
    pub fn new(data: f32) -> Self {
        StaticBody {mass:data}
    }
}
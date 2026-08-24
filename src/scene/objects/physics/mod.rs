
use glam::Vec3;

use crate::scene::objects::{physics::bodies::StaticBody, transform::Transform};

use bodies::RigidBody;

pub mod bodies;
pub mod controllers;

/*pub trait PhysicsObject{
    fn update_physics(&mut self, dt:f32, model: &mut Transform);
    fn add_force(&mut self, force: Vec3);
    fn set_force(&mut self, force: Vec3);
    fn mass(&self) -> f32;
    fn set_velocity(&mut self, vel: Vec3);
}*/
#[derive(Debug, Clone)]
pub enum PhysicsObject {
    RigidBody(RigidBody),
    StaticBody(StaticBody),
    Nothing()
    // KinematicBody(KinematicBody),
}


impl PhysicsObject {

    pub fn rigid_body(mass: f32) -> Self {
        PhysicsObject::RigidBody(RigidBody::new(mass))
    }

    pub fn static_body(mass: f32) -> Self {
        PhysicsObject::StaticBody(StaticBody::new(mass))
    }

    pub fn copy_state(&self) -> Self {
        match self {
            PhysicsObject::RigidBody(body) => {
                PhysicsObject::RigidBody(body.clone())
            }
            PhysicsObject::StaticBody(body) => {
                PhysicsObject::StaticBody(body.clone())
            }
            PhysicsObject::Nothing() => PhysicsObject::Nothing(),
        }
    }


    pub fn update_physics(&mut self, dt: f32, transform: &mut Transform) {
        match self {
            PhysicsObject::RigidBody(body) => {
                body.update_physics(dt, transform);
            }
            PhysicsObject::StaticBody(_) => (),
            PhysicsObject::Nothing() => (),
        }
    }

    pub fn set_velocity(&mut self, velocity: Vec3) {
        match self {
            PhysicsObject::RigidBody(body) => {
                body.velocity = velocity;
            }
            PhysicsObject::StaticBody(_) => (),
            PhysicsObject::Nothing() => (),
        }
    }

    pub fn activate(&mut self) {
        match self {
            PhysicsObject::RigidBody(body) => {
                body.activated = true;
            }
            PhysicsObject::StaticBody(_) => (),
            PhysicsObject::Nothing() => (),
        }
    }

    pub fn disable(&mut self) {
        match self {
            PhysicsObject::RigidBody(body) => {
                body.activated = false;
            }
            PhysicsObject::StaticBody(_) => (),
            PhysicsObject::Nothing() => (),
        }
    }

    pub fn velocity(&self) -> Vec3 {
        match self {
            PhysicsObject::RigidBody(body) => {
                body.velocity
            }
            PhysicsObject::StaticBody(_) => Vec3::ZERO,
            PhysicsObject::Nothing() => Vec3::ZERO,
        }
    }

    pub fn acceleration(&self) -> Vec3 {
        match self {
            PhysicsObject::RigidBody(body) => {
                body.acceleration
            }
            PhysicsObject::StaticBody(_) => Vec3::ZERO,
            PhysicsObject::Nothing() => Vec3::ZERO,
        }
    }

    pub fn force(&self) -> Vec3 {
        match self {
            PhysicsObject::RigidBody(body) => {
                body.force
            }
            PhysicsObject::StaticBody(_) => Vec3::ZERO,
            PhysicsObject::Nothing() => Vec3::ZERO,
        }
    }

    pub fn add_force(&mut self, force: Vec3) {
        match self {
            PhysicsObject::RigidBody(body) => {
                body.force += force;
            }
            PhysicsObject::StaticBody(_) => (),
            PhysicsObject::Nothing() => (),
        }
    }

    pub fn set_force(&mut self, force: Vec3) {
        match self {
            PhysicsObject::RigidBody(body) => {
                body.force = force;
            }
            PhysicsObject::StaticBody(_) => (),
            PhysicsObject::Nothing() => (),
        }
    }

    pub fn mass(&self) -> f32 {
        match self {
            PhysicsObject::RigidBody(body) => {
                body.mass
            }
            PhysicsObject::StaticBody(body) => {
                body.mass
            },
            PhysicsObject::Nothing() => 0.0,
        }
    }
}





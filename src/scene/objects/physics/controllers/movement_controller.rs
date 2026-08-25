
use core::f32;
use std::println;

use glam::{Quat, Vec3};

use crate::{scene::{SceneContent, objects::{ObjectId, SceneObject, SceneObjectBehaviour, WorldObject}, renders::TemporaryRender}, util::input_handler::InputHandler};

use super::Controller;

pub trait MovementType {
    fn change_movement(&mut self, world_object: &ObjectId, scene: &mut SceneContent, input: &InputHandler);
}


pub struct Movement<T: MovementType>{
    mover: T,
    ids: Vec<ObjectId>,
}

impl<T: MovementType> Movement<T>{
    pub fn new(mover: T) -> Movement<T>{
        Movement {mover: mover, ids:Vec::new()}
    }
}

impl<T: MovementType>  Controller for Movement<T>{
    fn update(&mut self, scene: &mut SceneContent, input: &InputHandler) {
        for id in &self.ids {
            self.mover.change_movement(id, scene, input);
        };
    }
    
    fn add(&mut self, objects: Vec<&ObjectId>) {
        for obj in objects{
            self.ids.push(*obj);
        }
    }
    
    fn add_single(&mut self, object: &ObjectId) {
        self.ids.push(*object);
    }
}

pub struct SimpleMover {
    speed: f32,
    rot: f32,
}

impl SimpleMover {
    pub fn new(speed: f32) -> SimpleMover {
        SimpleMover {speed, rot: 0.0} 
    }

    fn move_world(&mut self, world_object: &mut WorldObject, input: &InputHandler) {

        let delta_y = input.get_movement().y; 
        let delta_x = input.get_movement().x;
        let delta_z = if input.is_mouse_pressed(winit::event::MouseButton::Left) {1.0} else if input.is_mouse_pressed(winit::event::MouseButton::Right) {-1.0 } else {0.0};

        let prev_pos = world_object.data.transform.position;
        let new_pos = prev_pos + Vec3::new(delta_x, delta_y, delta_z) * self.speed;

        println!("Previous posistion was: {:?}, with input ({}, {}), new pos is: {:?}", prev_pos, delta_x, delta_y, new_pos);
        println!("World object transform is: {:?}\n\n", world_object.data.transform);

        if input.is_pressed(winit::keyboard::KeyCode::KeyQ) {
            self.rot += 0.01;
            let rotation = Quat::from_rotation_x(self.rot);
            world_object.data.transform.set_position(new_pos);
            world_object.data.transform.set_rotation(rotation);
        }else {
            world_object.data.transform.set_position(new_pos);
        }        
    }

}


impl MovementType for SimpleMover {
    fn change_movement(&mut self, world_object: &ObjectId, scene: &mut SceneContent, input: &InputHandler) {
        let object = scene.objects().get_mut(*world_object);
        match object {
            Some(SceneObject::World(mut_obj)) => self.move_world(mut_obj, input),
            _ => (),
        }
    }
}

pub struct QuatMover{
    thrust: f32,
    orientation: Quat,
}

impl QuatMover{
    pub fn new() -> QuatMover {
        QuatMover {thrust: 1.0, orientation:Quat::from_rotation_x(0.0)*Quat::from_rotation_y(0.0)*Quat::from_rotation_z(0.0)} 
    }

    fn move_world(&mut self, object_id: &ObjectId, scene: &mut SceneContent, input: &InputHandler) {
        // Read user-input
        let new_renders: Vec<TemporaryRender> = Vec::new();

        let object = scene.objects().get_mut(*object_id);
        match object {
            Some(SceneObject::World(world_object)) => {


                let new_thrust = input.get_movement().y; 
                let mouse = input.delta();
                let pitch_delta = mouse.y;
                let yaw_delta = mouse.x;
                let side_thrust =  input.get_movement().x;
                let mut roll_delta = 0.0;

                if input.is_pressed(winit::keyboard::KeyCode::KeyQ) {
                    roll_delta += 1.0;
                }

                if input.is_pressed(winit::keyboard::KeyCode::KeyE) {
                    roll_delta -= 1.0;
                }


                let (pitch, yaw, roll) = (pitch_delta*0.001, yaw_delta *0.001, roll_delta * 0.05);

                let delta_rotation =
                    Quat::from_rotation_x(pitch) * Quat::from_rotation_y(yaw) * Quat::from_rotation_z(roll);

                self.orientation = (self.orientation * delta_rotation).normalize();
                //self.orientation = self.orientation.normalize();

                let forward = self.orientation * Vec3::Z;
                let up      = self.orientation * Vec3::Y;
                let right   = self.orientation * Vec3::X;

                let mut force = forward * new_thrust*self.thrust + right*side_thrust*self.thrust/2.0;
                //println!("Transform: {:?}, forward: {:?}", world_object.data.transform.position, forward);

                if input.is_pressed(winit::keyboard::KeyCode::Tab) {
                    force = -world_object.physics.velocity();
                }else {
                    world_object.data.transform.rotation = self.orientation;
                }
                world_object.physics.add_force(force);
            },
            _ => (),
        }
        //println!("New renders are: {:?}", new_renders);
        scene.render_objects.extend(new_renders);
    }
}

impl MovementType for QuatMover {
    fn change_movement(&mut self, object_id: &ObjectId, scene: &mut SceneContent, input: &InputHandler) {
        self.move_world(object_id, scene, input);
    }
}
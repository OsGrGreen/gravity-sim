
use glam::{Quat, Vec3, Vec4};

use crate::{scene::{Scene, SceneContent, objects::{ObjectId, SceneObject, WorldObject}}, util::input_handler::InputHandler};

use super::Controller;

pub trait MovementType {
    fn change_movement(&mut self, world_object: &mut SceneObject, input: &InputHandler);
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
        let mut objects = scene.objects();
        for id in &self.ids {
            let object = &mut objects[id.index];
            self.mover.change_movement(object, input);
        };
    }
    
    fn add(&mut self, objects: Vec<&crate::scene::objects::WorldObject>) {
        for obj in objects{
            self.ids.push(obj.data.id.clone());
        }
    }

    fn add_single(&mut self, object: &crate::scene::objects::WorldObject) {
        self.ids.push(object.data.id);
    }
}

pub struct SimpleMover {
    speed: f32,
}

impl SimpleMover {
    pub fn new(speed: f32) -> SimpleMover {
        SimpleMover {speed} 
    }

    fn move_world(&mut self, world_object: &mut WorldObject, input: &InputHandler) {

        let delta_y = input.get_movement().y; 
        let delta_x = input.get_movement().x;
        let delta_z = if input.is_mouse_pressed(winit::event::MouseButton::Left) {1.0} else if input.is_mouse_pressed(winit::event::MouseButton::Right) {-1.0 } else {0.0};

        let prev_pos = world_object.data.transform.position;
        let new_pos = prev_pos + Vec3::new(delta_x, delta_y, delta_z) * self.speed;

        println!("Previous posistion was: {:?}, with input ({}, {}), new pos is: {:?}", prev_pos, delta_x, delta_y, new_pos);
        println!("World object transform is: {:?}\n\n", world_object.data.transform);
        world_object.data.transform.set_position(new_pos);
    }

}


impl MovementType for SimpleMover {
    fn change_movement(&mut self, world_object: &mut SceneObject, input: &InputHandler) {
        match world_object {
            SceneObject::World(world_object) => self.move_world(world_object, input),
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
        QuatMover {thrust: 0.0, orientation:Quat::IDENTITY} 
    }

    fn move_world(&mut self, world_object: &mut WorldObject, input: &InputHandler) {
        // Read user-input

        // Get target force

        let new_thrust = -input.get_movement().y; 
        let mouse = input.ndc_pos();
        let pitch_delta = mouse.y;
        let yaw_delta = mouse.x;
        let roll_delta = input.get_movement().x;

        self.orientation =
            self.orientation
            * Quat::from_rotation_x(pitch_delta)
            * Quat::from_rotation_y(yaw_delta)
            * Quat::from_rotation_z(roll_delta);

        let forward = self.orientation * Vec3::Z;

        let force = forward * new_thrust;
        // Interpolate between forces

        // Bound to max

        world_object.physics.add_force(force);
    }
}

impl MovementType for QuatMover {
    fn change_movement(&mut self, world_object: &mut SceneObject, input: &InputHandler) {
        match world_object {
            SceneObject::World(world_object) => self.move_world(world_object, input),
            _ => (),
        }
    }
}

use glam::{Quat, Vec3, Vec4};

use crate::{scene::{Scene, SceneContent, objects::{ObjectId, WorldObject}}, util::input_handler::InputHandler};

use super::Controller;

pub trait MovementType {
    fn change_movement(&mut self, world_object: &mut WorldObject, input: &InputHandler);
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
            self.ids.push(obj.id.clone());
        }
    }

    fn add_single(&mut self, object: &crate::scene::objects::WorldObject) {
        self.ids.push(object.id);
    }
}

pub struct PlayerMover{
    thrust: f32,
    orientation: Quat,
}

impl PlayerMover{
    pub fn new() -> PlayerMover {
        PlayerMover {thrust: 0.0, orientation:Quat::IDENTITY} 
    }
}

impl MovementType for PlayerMover{
    fn change_movement(&mut self, world_object: &mut WorldObject, input: &InputHandler) {
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

        world_object.physics.set_force(force);
        //println!("Movement is x: {}, y: {}", input.get_movement().x, input.get_movement().y);
    }
}
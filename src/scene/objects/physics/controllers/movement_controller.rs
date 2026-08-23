
use glam::{Mat4, Quat, Vec3, Vec4};

use crate::{scene::{Scene, SceneContent, objects::{ObjectId, SceneObject, SceneObjectBehaviour, WorldObject, transform::Transform}, renders::TemporaryRender}, util::input_handler::InputHandler};

use super::Controller;

pub trait MovementType {
    fn change_movement(&mut self, world_object: ObjectId, scene: &mut SceneContent, input: &InputHandler);
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
            self.mover.change_movement(*id, scene, input);
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
    fn change_movement(&mut self, world_object: ObjectId, scene: &mut SceneContent, input: &InputHandler) {
        let world_object = &mut scene.objects()[world_object.index];
        match world_object {
            SceneObject::World(world_object) => self.move_world(world_object, input),
            _ => (),
        }
    }
}

pub struct QuatMover{
    thrust: f32,
    orientation: Quat,
    pitch: f32,
    yaw: f32,
    roll: f32,
    current_dir: u8,
}

impl QuatMover{
    pub fn new() -> QuatMover {
        QuatMover {thrust: 1.0, orientation:Quat::IDENTITY, pitch: 0.0, yaw: -3.15, roll: 0.0, current_dir: 0} 
    }

    fn move_world(&mut self, object_id: ObjectId, scene: &mut SceneContent, input: &InputHandler) {
        // Read user-input
        let mut new_renders: Vec<TemporaryRender> = Vec::new();

        let object = &mut scene.objects()[object_id.index];
        match object {
            SceneObject::World(world_object) => {

                println!("Moving world_object {:?}", object_id);

                let new_thrust = input.get_movement().y; 
                let mouse = -input.delta();
                let pitch_delta = mouse.y;
                let yaw_delta = mouse.x;
                let side_thrust =  input.get_movement().x;

                if input.is_pressed(winit::keyboard::KeyCode::KeyQ) {
                    self.current_dir = (self.current_dir + 1) % 3;
                }

                if input.is_pressed(winit::keyboard::KeyCode::KeyE) {
                    self.current_dir = (self.current_dir + 2) % 3;
                }

                /*let (pitch, yaw, roll) = match self.current_dir {
                    0 => (roll_delta, 0.0, 0.0),
                    1 => (0.0, roll_delta, 0.0),
                    2 => (0.0, 0.0, roll_delta),
                    _ => unreachable!(),
                };*/

                let (pitch, yaw, roll) = (pitch_delta*2.0, yaw_delta * 2.0, 0.0);

                println!(
                    "Thrust: {}, Pitch: {}, Yaw: {}, Roll: {}",
                    new_thrust, pitch, yaw, roll
                );

                self.pitch = pitch;
                self.yaw = yaw;
                self.roll += roll;

                /*self.orientation =
                    Quat::from_rotation_x(self.pitch)
                    * Quat::from_rotation_y(self.yaw)
                    * Quat::from_rotation_z(self.roll);
                    */


                let delta_rotation =
                    Quat::from_rotation_x(pitch) * Quat::from_rotation_y(yaw) * Quat::from_rotation_z(0.0);

                self.orientation = (self.orientation * delta_rotation).normalize();

                //self.orientation = self.orientation.normalize();

                println!("{}, {}, {}", self.pitch, self.yaw, self.roll);
                //println!("orientation is {:?}", self.orientation);
                let forward = self.orientation * Vec3::NEG_Z;
                let up      = self.orientation * Vec3::Y;
                let right   = -(self.orientation * Vec3::X);

                println!("Forward is: {}, up is {}, right is {}", forward, up, right);
                let mut force = forward * new_thrust*self.thrust + right*side_thrust*self.thrust/2.0;
                //println!("Transform: {:?}, forward: {:?}", world_object.data.transform.position, forward);
                
                // Interpolate between forces
                if let Some(renderable) = &world_object.data.render {
                    let mut transform = Transform::new_from_pos(world_object.data.transform.position + forward);
                    transform.scale(Vec3::new(0.1, 0.1, 0.1));
                    let direction = TemporaryRender { transform, render_id: renderable.id(), reset: true };
                    new_renders.push(direction);
                }


                // Bound to max
                if input.is_pressed(winit::keyboard::KeyCode::Tab) {
                    force = -world_object.physics.velocity();
                }else {
                    world_object.data.transform.rotation = self.orientation * 0.5;
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
    fn change_movement(&mut self, object_id: ObjectId, scene: &mut SceneContent, input: &InputHandler) {
        self.move_world(object_id, scene, input);
    }
}
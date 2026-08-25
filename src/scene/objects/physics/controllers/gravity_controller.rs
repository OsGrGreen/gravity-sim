
use std::time::Instant;

use glam::{Mat4, Vec3};

use crate::{scene::{SceneContent, objects::{ObjectId, SceneObject, WorldObject}}, util::{input_handler::InputHandler, ray_library::{mouse_ray, ray_plane_intersection}}};

use super::Controller;

const FORCE_MULTIPLIER: f32 = 33.0;

#[derive(Debug, Clone)]
pub struct PlayerGravity{
    G: f32,
    ids: Vec<ObjectId>,
    activated: bool,
}


impl PlayerGravity{
    pub fn new(g_const: f32) -> PlayerGravity{
        PlayerGravity { G: g_const , ids:Vec::new(), activated: true}
    }
}

impl Controller for PlayerGravity {
fn update(&mut self, scene: &mut SceneContent, input: &InputHandler) {
        if !self.activated {
            let camera_mat = &scene.camera.getMatrix();
            let camera_view = Mat4::from_cols_array_2d(camera_mat);
            let camera_projection = scene.camera.perspective;

            if input.is_pressed(winit::keyboard::KeyCode::Enter) {
                self.activated = true;
                for id in &self.ids {
                    let object = scene.objects().get_mut(*id).expect("Object did not exist");
                    match object {
                        SceneObject::World(world_object) => world_object.physics.activate(),
                        SceneObject::Spline(_) => (),
                    }
                }
            }

            let (ray_origin, ray_direction) =
            mouse_ray(input.pos(), camera_projection, camera_view);
            for id in &self.ids {
                    let object = scene.objects().get_mut(*id).expect("Object did not exist");
                    match object {
                        SceneObject::World(world_object) => {
                            if let Some(intersection) =  ray_plane_intersection(
                                ray_origin,
                                ray_direction,
                                Vec3::ZERO,
                                Vec3::Z,
                            ){
                                let (direction, _) = world_object.distance_point(intersection);
                                world_object.physics.set_velocity(direction);
                                //println!("World object real state is: ({:?}, {:?})", world_object.data.transform, world_object.physics);
                            }
                        },
                        SceneObject::Spline(_) => (),
                    }
                };
        }else {
            let new_time = Instant::now();
            for id in &self.ids {
                let mut total_force = Vec3::ZERO;

                let obj = scene.objects.get(*id).expect("Object did not exist");
                let object = match obj {
                    SceneObject::World(o) => o,
                    SceneObject::Spline(_) => panic!("Is not world_object"),
                };
                
                for world_object in scene.objects.world_objects() {
                    if world_object.data.id != object.data.id {
                        let (dir, distance) = object.distance(world_object);
                        let force = self.G*(object.physics.mass()*world_object.physics.mass())/(distance*distance);
                        
                        total_force += dir.normalize() * force;
                        if object.collides(world_object){
                                //println!("Collision between {:?} and {:?}", obj1,obj2);
                        }
                    }
                }
                if let Some(SceneObject::World(object)) = scene.objects().get_mut(*id) {
                    object.physics.add_force(total_force);
                }
            }   
        }
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

#[derive(Debug, Clone)]
pub struct Gravity{
    G: f32,
    ids: Vec<ObjectId>,
}

impl Gravity{
    pub fn new(g_const: f32) -> Gravity{
        Gravity { G: g_const , ids:Vec::new()}
    }
}

impl Controller for Gravity{
    fn update(&mut self, scene: &mut SceneContent, _: &InputHandler) {
        for id in &self.ids {
            let mut total_force = Vec3::ZERO;

            let obj = scene.objects.get(*id).expect("Object did not exist");
            let object = match obj {
                SceneObject::World(o) => o,
                SceneObject::Spline(_) => panic!("Is not world_object"),
            };
                
            for world_object in scene.objects.world_objects() {
                if world_object.data.id != object.data.id {
                    let (dir, distance) = object.distance(world_object);
                    let force = self.G*(object.physics.mass()*world_object.physics.mass())/(distance*distance);
                        
                    total_force += dir.normalize() * force;
                    if object.collides(world_object){
                            //println!("Collision between {:?} and {:?}", obj1,obj2);
                    }
                }
            }
            if let Some(SceneObject::World(object)) = scene.objects().get_mut(*id) {
                object.physics.add_force(total_force);
            }
        }
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
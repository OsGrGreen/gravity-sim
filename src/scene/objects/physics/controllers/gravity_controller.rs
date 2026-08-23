
use glam::{Mat4, Vec3};

use crate::{scene::{Scene, SceneContent, objects::{self, ObjectId, SceneObject, WorldObject}}, util::{input_handler::InputHandler, ray_library::{mouse_ray, ray_plane_intersection}}};

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
        let mut objects: Vec<&mut WorldObject> = scene.objects().iter_mut().filter_map(|obj| {
            match obj {
                SceneObject::World(obj) => Some(obj),
                SceneObject::Spline(_) => None,
            }
        }).collect();

        if !self.activated {
            println!("Big slay");
            let camera_mat = &scene.camera.getMatrix();
            let camera_view = Mat4::from_cols_array_2d(camera_mat);
            let camera_projection = scene.camera.perspective;

            if input.is_pressed(winit::keyboard::KeyCode::Enter) {
                self.activated = true;
                for id in &self.ids {
                    let object = &mut scene.objects()[id.index];
                    match object {
                        SceneObject::World(world_object) => world_object.physics.activate(),
                        SceneObject::Spline(_) => (),
                    }
                }
            }

            let objects = scene.objects();

            

            let (ray_origin, ray_direction) =
            mouse_ray(input.pos(), camera_projection, camera_view);
            for id in &self.ids {
                    let object = &mut objects[id.index];
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
            for i in 0..objects.len() {
                let (before, after) = objects.split_at_mut(i);
                let (obj1, after) = after.split_first_mut().unwrap(); // Get obj1 from after
                if self.ids.contains(&obj1.data.id){
                    for obj2 in before.iter().chain(after.iter()) {
                        if obj2.physics.mass() != 0.0 {
                            let (dir, distance) = obj1.distance(obj2);
                            let force = self.G*(obj1.physics.mass()*obj2.physics.mass())/(distance*distance);
                            println!("Force is : {:?} in direction: {:?}, distance: {:?}, mass 1: {}, mass 2: {}", force, dir, distance, obj1.physics.mass(), obj2.physics.mass());
                            obj1.physics.add_force(dir.normalize()*force);
                            //println!("World object real state during simulation: ({:?}, {:?})", obj1.data.transform, obj1.physics);
                            //Collision
                            if obj1.collides(obj2){
                                //println!("Collision between {:?} and {:?}", obj1,obj2);
                            }
                        } 
                    }
                }
            }
        }
    }
    
    fn add(&mut self, objects: Vec<&crate::scene::objects::WorldObject>) {
        for obj in objects{
            self.ids.push(obj.data.id);
        }
    }
    
    fn add_single(&mut self, object: &crate::scene::objects::WorldObject) {
        self.ids.push(object.data.id);
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
        let mut objects: Vec<&mut WorldObject> = scene.objects().iter_mut().filter_map(|obj| {
            match obj {
                SceneObject::World(obj) => Some(obj),
                SceneObject::Spline(_) => None,
            }
        }).collect();

        for i in 0..objects.len() {
            let (before, after) = objects.split_at_mut(i);
            let (obj1, after) = after.split_first_mut().unwrap(); // Get obj1 from after
            if self.ids.contains(&obj1.data.id){
                for obj2 in before.iter().chain(after.iter()) {
                    let (dir, distance) = obj1.distance(obj2);
                    let force = self.G*(obj1.physics.mass()*obj2.physics.mass())/(distance*distance);
                    obj1.physics.add_force(dir.normalize()*force);
                    //Collision
                    if obj1.collides(obj2){
                        //println!("Collision between {:?} and {:?}", obj1,obj2);
                    }
                }
            }
        }
    }
    
    fn add(&mut self, objects: Vec<&crate::scene::objects::WorldObject>) {
        for obj in objects{
            self.ids.push(obj.data.id);
        }
    }
    
    fn add_single(&mut self, object: &crate::scene::objects::WorldObject) {
        self.ids.push(object.data.id);
    }
}
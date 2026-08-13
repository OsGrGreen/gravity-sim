
use crate::{scene::{Scene, SceneContent, objects::{self, ObjectId, SceneObject, WorldObject}}, util::input_handler::InputHandler};

use super::Controller;

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
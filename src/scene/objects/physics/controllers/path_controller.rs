

use crate::{scene::{SceneContent, objects::{ObjectId, SceneObjectBehaviour}}, spline::Spline, util::input_handler::InputHandler};

use super::Controller;

pub struct Path{
    spline: Spline,
    t: f32,
    timestep: f32,
    ids: Vec<ObjectId>,
}

impl Path{
    pub fn new(spline: Spline, timestep: f32) -> Path{
        Path { spline: spline, t:0.0, timestep:timestep, ids:Vec::new()}
    }
}

impl Controller for Path{
    fn update(&mut self, scene: &mut SceneContent, _: &InputHandler) {
        for id in &self.ids {
            let obj = scene.objects().get_mut(*id).expect("Object did not exist");
            let new_pos = self.spline.evaluate(self.t);
            let current_pos = obj.transform().position;
            obj.data_mut().transform.set_position(new_pos);
            //obj.physics_object.set_force(new_pos-current_pos);
            self.t += self.timestep;
            if self.t + self.timestep >= self.spline.len() as f32{
                self.t -= self.spline.len() as f32;
            }
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
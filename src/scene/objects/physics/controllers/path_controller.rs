
use std::time;

use crate::{scene::objects::ObjectId, spline::Spline, util::input_handler::InputHandler};

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
    fn update(&mut self, objects: &mut Vec<crate::scene::objects::WorldObject>, _: &InputHandler) {
        for id in &self.ids {
            let obj = &mut objects[id.index];
            let new_pos = self.spline.evaluate(self.t);
            let current_pos = obj.render_object.model_object.get_posistion();
            obj.render_object.model_object.set_posistion(new_pos);
            //obj.physics_object.set_force(new_pos-current_pos);
            self.t += self.timestep;
            if self.t + self.timestep >= self.spline.len() as f32{
                self.t -= self.spline.len() as f32;
            }
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
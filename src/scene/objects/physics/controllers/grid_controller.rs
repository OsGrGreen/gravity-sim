use crate::scene::objects::{ObjectId, physics::controllers::Controller};

pub struct GridGravityController{
    G: f32,
    ids: Vec<ObjectId>,
}


impl Controller for GridGravityController {
    fn update(&mut self, scene: &mut crate::scene::SceneContent, _: &crate::util::input_handler::InputHandler) {
        // For each vertex in the renderer we need to displace it according to the normal.
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
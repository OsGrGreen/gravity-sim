use glam::{Mat4, Vec3};

use crate::{scene::{SceneContent, objects::{self, ObjectId, WorldObject, physics::controllers::Controller}}, util::{input_handler::InputHandler, ray_library::{mouse_ray, ray_plane_intersection}}};

pub struct Line{
    pub line_render: ObjectId,
    pub line_points: Vec<ObjectId>
}

pub struct LineController{
    ids: Vec<Line>,
}

impl LineController {
    pub fn new(opt_ids: Option<Vec<Line>>) -> LineController {
        if let Some(ids) = opt_ids {
            LineController { ids }
        }else {
            LineController { ids: vec![] }
        }
    }
}

impl Controller for LineController {
    fn update(&mut self, scene: &mut SceneContent, input: &InputHandler) {
        let mut objects = scene.objects();
        
    }

    fn add(&mut self, objects: Vec<&WorldObject>) {
        todo!()
    }

    fn add_single(&mut self, object: &WorldObject) {
        return;
    }
}

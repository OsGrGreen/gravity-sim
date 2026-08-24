
use crate::{scene::{SceneContent, objects::{self, ObjectId, WorldObject, physics::controllers::Controller}}, spline::Spline, util::input_handler::InputHandler};

pub struct Line{
    pub line: ObjectId,
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
        let objects = scene.objects();

        for line in &self.ids {
            let pos1 = match objects.get(line.line_points[0].index) {
                Some(obj) => obj.transform().position,
                None => continue,
            };

            let pos2 = match objects.get(line.line_points[1].index) {
                Some(obj) => obj.transform().position,
                None => continue,
            };

            let line_object = match &mut objects[line.line.index] {
                objects::SceneObject::Spline(line_object) => line_object,
                _ => continue,
            };

            line_object.spline = Spline::new([
                pos1,
                pos1,
                pos2,
                pos2,
            ]);
        }
    }

    fn add(&mut self, objects: Vec<&WorldObject>) {
        todo!()
    }

    fn add_single(&mut self, object: &WorldObject) {
        return;
    }
}

use glam::{Mat4, Vec3};

use crate::{scene::{SceneContent, objects::{ObjectId, SceneObjectBehaviour, physics::controllers::Controller}}, util::{input_handler::InputHandler, ray_library::{mouse_ray, ray_plane_intersection}}};

pub struct MouseDragController{
    ids: Vec<ObjectId>
}

impl MouseDragController {
    pub fn new(opt_ids: Option<Vec<ObjectId>>) -> MouseDragController {
        if let Some(ids) = opt_ids {
            MouseDragController { ids }
        }else {
            MouseDragController { ids: vec![] }
        }
    }
}

impl Controller for MouseDragController {
    // Must know where camera is so I can cast a ray from the specific point from the camera in the direction of the mouse until it hits the plane that the point is on...
    fn update(&mut self, scene: &mut SceneContent, input: &InputHandler) {
        let camera_mat = &scene.camera.getMatrix();
        let camera_view = Mat4::from_cols_array_2d(camera_mat);
        let camera_projection = scene.camera.perspective;
        let objects = scene.objects();

        let (ray_origin, ray_direction) =
            mouse_ray(input.pos(), camera_projection, camera_view);


        for id in &self.ids {
            let object = &mut objects[id.index];
            if let Some(intersection) =  ray_plane_intersection(
                ray_origin,
                ray_direction,
                Vec3::ZERO,
                Vec3::Z,
            ){
                object.data_mut().transform.set_position(intersection);
            }
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
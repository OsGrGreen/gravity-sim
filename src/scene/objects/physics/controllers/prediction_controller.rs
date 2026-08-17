use glam::Vec3;

use crate::{scene::{Scene, SceneContent, objects::{ObjectId, SceneObject, SceneObjectBehaviour, WorldObject, physics::{self, PhysicsObject, bodies::RigidBody, controllers::Controller}, renderable::{self, MeshRenderer}, transform::{self, Transform}}, renders::TemporaryRender}, util::input_handler::InputHandler};

#[derive(Debug)]
pub struct PredictionState {
    pub transform: Transform,
    pub physics: PhysicsObject,
}

pub struct PredictionController{
    pub steps: usize,
    pub dt: f32,
    pub controller: Box<dyn Controller>,
    ids: Vec<ObjectId>,
}

impl PredictionController {
    pub fn new(steps: usize, dt: f32,controller: Box<dyn Controller>) -> PredictionController {
        PredictionController { steps, dt, controller, ids: Vec::new()}
    }


    fn do_step(&self, world_object: &WorldObject, scene: &SceneContent, previous: PredictionState) -> PredictionState {
        let mut physics = self.update_gravity(world_object, &previous.transform, scene, previous.physics);
        let mut transform = previous.transform;
        physics.update_physics(self.dt, &mut transform);
        PredictionState { transform, physics }
    }

    fn update_gravity(&self, target_object: &WorldObject, current_transform: &Transform, scene: &SceneContent, previous: PhysicsObject) -> PhysicsObject {
        let objects: Vec<&WorldObject> = scene.read_objects().iter().filter_map(|obj| {
            match obj {
                SceneObject::World(obj) => Some(obj),
                SceneObject::Spline(_) => None,
            }
        }).collect();

        let g = 1.0;
        let mut physics = previous;
        let predicted_pos = current_transform.position; 

        for obj2 in objects {
            if obj2.data.id == target_object.data.id {
                continue;
            }

            let (dir, distance) = obj2.distance_point(predicted_pos);
            let force = g * (target_object.physics.mass() * obj2.physics.mass()) / (distance * distance);
            physics.add_force(-dir.normalize() * force);
        }
        physics
    }
}

impl Controller for PredictionController{
    fn update(&mut self, scene: &mut crate::scene::SceneContent, _: &crate::util::input_handler::InputHandler) {
        let mut new_renders = Vec::new();
        let objects = scene.read_objects();
        for id in &self.ids {
            let object = &objects[id.index];
            match object {
                SceneObject::World(world_object) => {
                    let mut prev_state = PredictionState{
                        transform: world_object.data.transform,
                        physics: world_object.physics.clone(),
                    };
                    prev_state.physics.activate();
                    //println!("Inital prediction state is: {:?}", prev_state);
                    for i in 0..self.steps {
                        let new_state = self.do_step(&world_object, scene, prev_state);
                        if i == 0{
                            //println!("First predicted state is: {:?}", new_state);
                        }
                        if let Some(renderable) = &world_object.data.render {
                            let render = TemporaryRender { transform: new_state.transform.clone(), render_id: renderable.id(), reset: true };
                            new_renders.push(render);
                        }
                        prev_state = new_state;
                    }
                },
                SceneObject::Spline(_) => continue,
            }

        };
        scene.render_objects.extend(new_renders);
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

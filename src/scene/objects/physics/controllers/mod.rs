use crate::{scene::{SceneContent, objects::{ObjectId, WorldObject}}, util::input_handler::InputHandler};

pub mod gravity_controller;

pub mod path_controller;

pub mod movement_controller;

pub mod mouse_movement_controller;

pub mod line_controller;

pub mod prediction_controller;

pub mod grid_controller;

pub trait Controller {
    fn update(&mut self, scene: &mut SceneContent, input: &InputHandler);
    fn add(&mut self, objects: Vec<&ObjectId>);
    fn add_single(&mut self, object: &ObjectId);
}